//! SQL DDL Type Provider
//!
//! Generates Fusabi types from SQL DDL (Data Definition Language) statements.
//!
//! # Supported Databases
//!
//! - PostgreSQL
//! - MySQL
//! - SQLite
//!
//! # Example
//!
//! ```rust,ignore
//! use fusabi_provider_sql::SqlProvider;
//! use fusabi_type_providers::{TypeProvider, ProviderParams};
//!
//! let provider = SqlProvider::new();
//! let schema = provider.resolve_schema("schema.sql", &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "Database")?;
//! ```

mod parser;
mod types;

pub use types::{SqlDialect, SqlSchema, SqlType};

use fusabi_type_providers::{
    GeneratedModule, GeneratedTypes, NamingStrategy, ProviderError, ProviderParams,
    ProviderResult, RecordDef, Schema, TypeDefinition, TypeExpr, TypeGenerator, TypeProvider,
};

/// SQL DDL type provider
pub struct SqlProvider {
    generator: TypeGenerator,
}

impl SqlProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Parse SQL DDL from string
    fn parse_sql(&self, sql: &str) -> ProviderResult<types::SqlSchema> {
        parser::parse_sql_ddl(sql)
    }

    /// Generate types from parsed SQL schema
    fn generate_from_schema(
        &self,
        schema: &types::SqlSchema,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        let mut result = GeneratedTypes::new();
        let mut tables_module = GeneratedModule::new(vec![namespace.to_string()]);

        // Generate a RecordDef for each table
        for (_table_name, table) in &schema.tables {
            let type_def = self.table_to_typedef(table)?;
            tables_module.types.push(type_def);
        }

        if !tables_module.types.is_empty() {
            result.modules.push(tables_module);
        }

        Ok(result)
    }

    /// Convert a SQL table to a Fusabi RecordDef
    fn table_to_typedef(&self, table: &types::Table) -> ProviderResult<TypeDefinition> {
        let mut fields = Vec::new();

        for column in &table.columns {
            let type_expr = self.sql_type_to_type_expr(&column.sql_type)?;

            // Wrap in option if nullable and not primary key
            let final_type = if column.is_nullable() && !column.is_primary_key() {
                TypeExpr::Named(format!("{} option", type_expr))
            } else {
                type_expr
            };

            fields.push((column.name.clone(), final_type));
        }

        Ok(TypeDefinition::Record(RecordDef {
            name: self.generator.naming.apply(&table.name),
            fields,
        }))
    }

    /// Map SQL types to Fusabi types
    fn sql_type_to_type_expr(&self, sql_type: &types::SqlType) -> ProviderResult<TypeExpr> {
        let type_name = match sql_type {
            // Integer types -> int
            SqlType::TinyInt
            | SqlType::SmallInt
            | SqlType::Int
            | SqlType::Serial => "int".to_string(),

            // BigInt -> int64 or bigint
            SqlType::BigInt | SqlType::BigSerial => "int".to_string(),

            // Floating point -> float
            SqlType::Real | SqlType::Float => "float".to_string(),

            // Double precision -> float
            SqlType::Double => "float".to_string(),

            // Decimal/Numeric -> float (or could be a custom decimal type)
            SqlType::Decimal { .. } | SqlType::Numeric { .. } => "float".to_string(),

            // String types -> string
            SqlType::Char { .. } | SqlType::VarChar { .. } | SqlType::Text => "string".to_string(),

            // Boolean -> bool
            SqlType::Boolean => "bool".to_string(),

            // Date/Time -> string (could be custom datetime types)
            SqlType::Date | SqlType::Time | SqlType::Timestamp | SqlType::TimestampTz => {
                "string".to_string()
            }

            // Binary -> bytes or string
            SqlType::Blob | SqlType::Bytea => "bytes".to_string(),

            // JSON -> any or custom JSON type
            SqlType::Json | SqlType::JsonB => "string".to_string(),

            // UUID -> string
            SqlType::Uuid => "string".to_string(),

            // Array types -> list
            SqlType::Array { element_type } => {
                let element = self.sql_type_to_type_expr(element_type)?;
                format!("{} list", element)
            }

            // Custom types -> use type name as-is
            SqlType::Custom(name) => name.clone(),
        };

        Ok(TypeExpr::Named(type_name))
    }
}

impl Default for SqlProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for SqlProvider {
    fn name(&self) -> &str {
        "SqlProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        // Support inline SQL or file paths
        let sql_str = if source.to_uppercase().trim().starts_with("CREATE") {
            // Inline SQL
            source.to_string()
        } else if source.starts_with("file://") {
            // File URL
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        } else {
            // Treat as file path
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        };

        // Store SQL as custom schema
        Ok(Schema::Custom(sql_str))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(sql_str) => {
                let parsed = self.parse_sql(sql_str)?;
                self.generate_from_schema(&parsed, namespace)
            }
            _ => Err(ProviderError::ParseError(
                "Expected SQL schema".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_table() {
        let provider = SqlProvider::new();
        let sql = r#"
            CREATE TABLE users (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email TEXT,
                age INT
            );
        "#;

        let schema = provider.resolve_schema(sql, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Database").unwrap();

        assert_eq!(types.modules.len(), 1);
        let module = &types.modules[0];
        assert_eq!(module.types.len(), 1);

        // Check that we got a Users record type
        if let TypeDefinition::Record(record) = &module.types[0] {
            assert_eq!(record.name, "Users");
            assert_eq!(record.fields.len(), 4);

            // Check field names
            assert_eq!(record.fields[0].0, "id");
            assert_eq!(record.fields[1].0, "name");
            assert_eq!(record.fields[2].0, "email");
            assert_eq!(record.fields[3].0, "age");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_generate_multiple_tables() {
        let provider = SqlProvider::new();
        let sql = r#"
            CREATE TABLE users (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );

            CREATE TABLE posts (
                id INT PRIMARY KEY,
                user_id INT NOT NULL,
                title TEXT NOT NULL,
                content TEXT
            );
        "#;

        let schema = provider.resolve_schema(sql, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Database").unwrap();

        assert_eq!(types.modules.len(), 1);
        let module = &types.modules[0];
        assert_eq!(module.types.len(), 2);
    }

    #[test]
    fn test_nullable_fields() {
        let provider = SqlProvider::new();
        let sql = r#"
            CREATE TABLE items (
                id INT PRIMARY KEY,
                required_field VARCHAR(100) NOT NULL,
                optional_field VARCHAR(100)
            );
        "#;

        let schema = provider.resolve_schema(sql, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Database").unwrap();

        if let TypeDefinition::Record(record) = &types.modules[0].types[0] {
            // id is primary key - not nullable
            assert!(!record.fields[0].1.to_string().contains("option"));

            // required_field has NOT NULL - not nullable
            assert!(!record.fields[1].1.to_string().contains("option"));

            // optional_field is nullable
            assert!(record.fields[2].1.to_string().contains("option"));
        }
    }

    #[test]
    fn test_type_mappings() {
        let provider = SqlProvider::new();
        let sql = r#"
            CREATE TABLE types_test (
                int_col INT,
                bigint_col BIGINT,
                varchar_col VARCHAR(255),
                text_col TEXT,
                bool_col BOOLEAN,
                float_col REAL,
                double_col DOUBLE PRECISION,
                decimal_col DECIMAL(10, 2),
                date_col DATE,
                timestamp_col TIMESTAMP,
                json_col JSON,
                uuid_col UUID,
                blob_col BLOB
            );
        "#;

        let schema = provider.resolve_schema(sql, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Database").unwrap();

        if let TypeDefinition::Record(record) = &types.modules[0].types[0] {
            // Verify basic type mappings (accounting for option wrapper)
            let field_types: Vec<String> = record
                .fields
                .iter()
                .map(|(_, t)| t.to_string())
                .collect();

            // All nullable, so all should have "option"
            for ft in &field_types {
                assert!(ft.contains("option"));
            }
        }
    }

    #[test]
    fn test_postgresql_array_types() {
        let provider = SqlProvider::new();
        let sql = r#"
            CREATE TABLE array_test (
                id INT PRIMARY KEY,
                tags TEXT[],
                numbers INT[]
            );
        "#;

        let schema = provider.resolve_schema(sql, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Database").unwrap();

        if let TypeDefinition::Record(record) = &types.modules[0].types[0] {
            // tags should be a list of strings
            assert!(record.fields[1].1.to_string().contains("list"));

            // numbers should be a list of ints
            assert!(record.fields[2].1.to_string().contains("list"));
        }
    }
}
