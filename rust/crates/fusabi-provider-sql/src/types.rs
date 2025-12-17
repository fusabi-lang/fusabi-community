//! SQL DDL type definitions

use std::collections::HashMap;

/// SQL data type
#[derive(Debug, Clone, PartialEq)]
pub enum SqlType {
    // Integer types
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    Serial,
    BigSerial,

    // Floating point types
    Real,
    Double,
    Float,
    Decimal { precision: Option<u32>, scale: Option<u32> },
    Numeric { precision: Option<u32>, scale: Option<u32> },

    // String types
    Char { length: Option<u32> },
    VarChar { length: Option<u32> },
    Text,

    // Boolean
    Boolean,

    // Date/Time types
    Date,
    Time,
    Timestamp,
    TimestampTz,

    // Binary types
    Blob,
    Bytea,

    // JSON types
    Json,
    JsonB,

    // UUID
    Uuid,

    // Array type (PostgreSQL)
    Array { element_type: Box<SqlType> },

    // Custom/Unknown types
    Custom(String),
}

impl SqlType {
    /// Parse SQL type from string
    pub fn from_str(s: &str) -> Self {
        let s_upper = s.to_uppercase();
        let s_trimmed = s_upper.trim();

        // Handle types with parameters
        if let Some(idx) = s_trimmed.find('(') {
            let base_type = &s_trimmed[..idx];
            let params = &s_trimmed[idx+1..s_trimmed.len()-1];

            return match base_type {
                "CHAR" | "CHARACTER" => {
                    SqlType::Char { length: params.parse().ok() }
                }
                "VARCHAR" | "CHARACTER VARYING" => {
                    SqlType::VarChar { length: params.parse().ok() }
                }
                "DECIMAL" | "DEC" => {
                    let parts: Vec<&str> = params.split(',').collect();
                    SqlType::Decimal {
                        precision: parts.get(0).and_then(|p| p.trim().parse().ok()),
                        scale: parts.get(1).and_then(|s| s.trim().parse().ok()),
                    }
                }
                "NUMERIC" => {
                    let parts: Vec<&str> = params.split(',').collect();
                    SqlType::Numeric {
                        precision: parts.get(0).and_then(|p| p.trim().parse().ok()),
                        scale: parts.get(1).and_then(|s| s.trim().parse().ok()),
                    }
                }
                "FLOAT" => SqlType::Float,
                _ => SqlType::Custom(s.to_string()),
            };
        }

        // Handle array types (PostgreSQL syntax)
        if s_trimmed.ends_with("[]") {
            let element_type_str = &s_trimmed[..s_trimmed.len()-2];
            return SqlType::Array {
                element_type: Box::new(SqlType::from_str(element_type_str)),
            };
        }

        // Simple types without parameters
        match s_trimmed {
            // Integer types
            "TINYINT" | "INT1" => SqlType::TinyInt,
            "SMALLINT" | "INT2" => SqlType::SmallInt,
            "INT" | "INTEGER" | "INT4" => SqlType::Int,
            "BIGINT" | "INT8" => SqlType::BigInt,
            "SERIAL" => SqlType::Serial,
            "BIGSERIAL" => SqlType::BigSerial,

            // Floating point
            "REAL" | "FLOAT4" => SqlType::Real,
            "DOUBLE" | "DOUBLE PRECISION" | "FLOAT8" => SqlType::Double,
            "FLOAT" => SqlType::Float,
            "DECIMAL" => SqlType::Decimal { precision: None, scale: None },
            "NUMERIC" => SqlType::Numeric { precision: None, scale: None },

            // String types
            "CHAR" | "CHARACTER" => SqlType::Char { length: None },
            "VARCHAR" | "CHARACTER VARYING" => SqlType::VarChar { length: None },
            "TEXT" | "LONGTEXT" | "MEDIUMTEXT" | "TINYTEXT" => SqlType::Text,

            // Boolean
            "BOOLEAN" | "BOOL" => SqlType::Boolean,

            // Date/Time
            "DATE" => SqlType::Date,
            "TIME" => SqlType::Time,
            "TIMESTAMP" | "DATETIME" => SqlType::Timestamp,
            "TIMESTAMPTZ" | "TIMESTAMP WITH TIME ZONE" => SqlType::TimestampTz,

            // Binary
            "BLOB" | "BINARY" | "VARBINARY" => SqlType::Blob,
            "BYTEA" => SqlType::Bytea,

            // JSON
            "JSON" => SqlType::Json,
            "JSONB" => SqlType::JsonB,

            // UUID
            "UUID" => SqlType::Uuid,

            // Default to custom type
            _ => SqlType::Custom(s.to_string()),
        }
    }
}

/// Column constraint
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    NotNull,
    Null,
    PrimaryKey,
    Unique,
    AutoIncrement,
    Default(String),
    ForeignKey { table: String, column: String },
    Check(String),
}

/// SQL column definition
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub sql_type: SqlType,
    pub constraints: Vec<Constraint>,
}

impl Column {
    pub fn new(name: String, sql_type: SqlType) -> Self {
        Self {
            name,
            sql_type,
            constraints: Vec::new(),
        }
    }

    pub fn is_nullable(&self) -> bool {
        !self.constraints.contains(&Constraint::NotNull)
            && !self.constraints.contains(&Constraint::PrimaryKey)
    }

    pub fn is_primary_key(&self) -> bool {
        self.constraints.contains(&Constraint::PrimaryKey)
    }

    pub fn has_default(&self) -> bool {
        self.constraints.iter().any(|c| matches!(c, Constraint::Default(_)))
    }
}

/// SQL table definition
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub table_constraints: Vec<TableConstraint>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
            table_constraints: Vec::new(),
        }
    }
}

/// Table-level constraints
#[derive(Debug, Clone, PartialEq)]
pub enum TableConstraint {
    PrimaryKey(Vec<String>),
    Unique(Vec<String>),
    ForeignKey {
        columns: Vec<String>,
        referenced_table: String,
        referenced_columns: Vec<String>,
    },
    Check(String),
}

/// SQL database dialect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SqlDialect {
    PostgreSQL,
    MySQL,
    SQLite,
    Generic,
}

/// Parsed SQL schema
#[derive(Debug, Clone, Default)]
pub struct SqlSchema {
    pub tables: HashMap<String, Table>,
    pub dialect: Option<SqlDialect>,
}

impl SqlSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.insert(table.name.clone(), table);
    }
}
