//! TOML Configuration Type Provider
//!
//! Generates Fusabi types from TOML configuration files by inferring types from values.
//!
//! # Example
//!
//! ```rust,ignore
//! use fusabi_provider_toml::TomlProvider;
//! use fusabi_type_providers::{TypeProvider, ProviderParams};
//!
//! let provider = TomlProvider::new();
//! let schema = provider.resolve_schema("config.toml", &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "Config")?;
//! ```

mod parser;
mod types;

pub use types::{TomlType, TomlValue};

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};


/// TOML configuration type provider
pub struct TomlProvider {
    generator: TypeGenerator,
}

impl TomlProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Parse TOML from string
    fn parse_toml(&self, toml_str: &str) -> ProviderResult<types::TomlSchema> {
        parser::parse_toml(toml_str)
    }

    /// Generate types from parsed TOML schema
    fn generate_from_toml(
        &self,
        schema: &types::TomlSchema,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        let mut result = GeneratedTypes::new();

        // Generate the root type from the TOML root table
        if schema.root.is_table() {
            // Collect all nested table types first
            let mut nested_types = Vec::new();
            self.collect_nested_types(&schema.root, namespace, &mut nested_types)?;

            // Generate the root record
            let fields = self.table_to_fields(&schema.root, namespace)?;
            let root_record = TypeDefinition::Record(RecordDef {
                name: self.generator.naming.apply(namespace),
                fields,
            });

            result.root_types.push(root_record);

            // Add nested types to a module if any were found
            if !nested_types.is_empty() {
                let mut module = GeneratedModule::new(vec![namespace.to_string()]);
                module.types.extend(nested_types);
                result.modules.push(module);
            }
        }

        Ok(result)
    }

    /// Collect nested table types that should become separate type definitions
    fn collect_nested_types(
        &self,
        value: &types::TomlValue,
        parent_name: &str,
        types: &mut Vec<TypeDefinition>,
    ) -> ProviderResult<()> {
        if value.is_table() {
            for (field_name, field_value) in &value.fields {
                if field_value.is_table() {
                    // Create a type for this nested table
                    let type_name = format!("{}{}", parent_name, self.generator.naming.apply(field_name));
                    let fields = self.table_to_fields(field_value, &type_name)?;

                    types.push(TypeDefinition::Record(RecordDef {
                        name: type_name.clone(),
                        fields,
                    }));

                    // Recursively collect deeper nested types
                    self.collect_nested_types(field_value, &type_name, types)?;
                } else if let types::TomlType::Array(elem_type) = &field_value.value_type {
                    // Check if array contains tables
                    if let types::TomlType::Table = **elem_type {
                        // Get the first array element to infer structure
                        if let toml::Value::Array(arr) = &field_value.original {
                            if let Some(toml::Value::Table(_)) = arr.first() {
                                // Create a type for the array element
                                let type_name = format!("{}{}Item", parent_name, self.generator.naming.apply(field_name));

                                // Use the first element as template
                                let template_value = types::TomlValue::from_value(arr[0].clone());
                                let fields = self.table_to_fields(&template_value, &type_name)?;

                                types.push(TypeDefinition::Record(RecordDef {
                                    name: type_name,
                                    fields,
                                }));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Convert a TOML table to record fields
    fn table_to_fields(
        &self,
        value: &types::TomlValue,
        parent_name: &str,
    ) -> ProviderResult<Vec<(String, TypeExpr)>> {
        let mut fields = Vec::new();

        for (field_name, field_value) in &value.fields {
            let type_expr = self.value_to_type_expr(field_value, field_name, parent_name)?;
            fields.push((field_name.clone(), type_expr));
        }

        Ok(fields)
    }

    /// Convert a TOML value to a TypeExpr
    fn value_to_type_expr(
        &self,
        value: &types::TomlValue,
        field_name: &str,
        parent_name: &str,
    ) -> ProviderResult<TypeExpr> {
        match &value.value_type {
            types::TomlType::String => Ok(TypeExpr::Named("string".to_string())),
            types::TomlType::Integer => Ok(TypeExpr::Named("int".to_string())),
            types::TomlType::Float => Ok(TypeExpr::Named("float".to_string())),
            types::TomlType::Boolean => Ok(TypeExpr::Named("bool".to_string())),
            types::TomlType::Datetime => Ok(TypeExpr::Named("string".to_string())), // TOML datetime as string
            types::TomlType::Array(elem_type) => {
                let elem_type_expr = self.array_elem_to_type_expr(elem_type, field_name, parent_name)?;
                Ok(TypeExpr::Named(format!("{} list", elem_type_expr)))
            }
            types::TomlType::Table => {
                // Reference to a nested type
                let type_name = format!("{}{}", parent_name, self.generator.naming.apply(field_name));
                Ok(TypeExpr::Named(type_name))
            }
        }
    }

    /// Convert array element type to TypeExpr
    fn array_elem_to_type_expr(
        &self,
        elem_type: &types::TomlType,
        field_name: &str,
        parent_name: &str,
    ) -> ProviderResult<TypeExpr> {
        match elem_type {
            types::TomlType::String => Ok(TypeExpr::Named("string".to_string())),
            types::TomlType::Integer => Ok(TypeExpr::Named("int".to_string())),
            types::TomlType::Float => Ok(TypeExpr::Named("float".to_string())),
            types::TomlType::Boolean => Ok(TypeExpr::Named("bool".to_string())),
            types::TomlType::Datetime => Ok(TypeExpr::Named("string".to_string())),
            types::TomlType::Table => {
                // Array of tables - reference the item type
                let type_name = format!("{}{}Item", parent_name, self.generator.naming.apply(field_name));
                Ok(TypeExpr::Named(type_name))
            }
            types::TomlType::Array(inner) => {
                // Nested array
                let inner_expr = self.array_elem_to_type_expr(inner, field_name, parent_name)?;
                Ok(TypeExpr::Named(format!("{} list", inner_expr)))
            }
        }
    }
}

impl Default for TomlProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for TomlProvider {
    fn name(&self) -> &str {
        "TomlProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        // Source can be inline TOML or file path
        let toml_str = if source.contains('=') || source.contains('[') {
            // Looks like inline TOML
            source.to_string()
        } else if source.starts_with("file://") {
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        } else {
            // Treat as file path without prefix
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        };

        // Validate that it parses as TOML
        let _value: toml::Value = toml::from_str(&toml_str)
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        // Store the TOML string directly in Schema::Custom
        Ok(Schema::Custom(toml_str))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        let toml_str = match schema {
            Schema::Custom(s) => s,
            _ => return Err(ProviderError::ParseError("Expected TOML Schema".to_string())),
        };

        let parsed = self.parse_toml(toml_str)?;
        self.generate_from_toml(&parsed, namespace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_config() {
        let provider = TomlProvider::new();
        let toml = r#"
            name = "myapp"
            version = "1.0.0"
            port = 8080
            debug = true
        "#;

        let schema = provider.resolve_schema(toml, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Config").unwrap();

        assert!(!types.root_types.is_empty());
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "Config");
            assert_eq!(record.fields.len(), 4);
        } else {
            panic!("Expected Record type");
        }
    }

    #[test]
    fn test_generate_nested_tables() {
        let provider = TomlProvider::new();
        let toml = r#"
            [database]
            host = "localhost"
            port = 5432

            [server]
            host = "0.0.0.0"
            port = 8080
        "#;

        let schema = provider.resolve_schema(toml, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Config").unwrap();

        assert!(!types.root_types.is_empty());
        // Should have nested types for database and server
        assert!(!types.modules.is_empty());
    }

    #[test]
    fn test_generate_arrays() {
        let provider = TomlProvider::new();
        let toml = r#"
            ports = [8080, 8081, 8082]
            tags = ["rust", "toml", "config"]
        "#;

        let schema = provider.resolve_schema(toml, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Config").unwrap();

        assert!(!types.root_types.is_empty());
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert!(record.fields.len() >= 2);
        }
    }

    #[test]
    fn test_generate_array_of_tables() {
        let provider = TomlProvider::new();
        let toml = r#"
            [[servers]]
            host = "localhost"
            port = 8080

            [[servers]]
            host = "0.0.0.0"
            port = 8081
        "#;

        let schema = provider.resolve_schema(toml, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Config").unwrap();

        assert!(!types.root_types.is_empty());
        // Should have a nested type for the server items
        assert!(!types.modules.is_empty());
    }

    #[test]
    fn test_datetime_type() {
        let provider = TomlProvider::new();
        let toml = r#"
            created_at = 1979-05-27T07:32:00Z
        "#;

        let schema = provider.resolve_schema(toml, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Config").unwrap();

        assert!(!types.root_types.is_empty());
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            // Datetime should be mapped to string
            assert!(record.fields.iter().any(|(name, _)| name == "created_at"));
        }
    }
}
