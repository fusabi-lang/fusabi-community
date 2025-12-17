//! TOML type definitions

use std::collections::HashMap;
use toml::Value;

/// Inferred TOML type
#[derive(Debug, Clone, PartialEq)]
pub enum TomlType {
    String,
    Integer,
    Float,
    Boolean,
    Datetime,
    Array(Box<TomlType>),
    Table,
}

/// Parsed TOML schema representation
#[derive(Debug, Clone)]
pub struct TomlSchema {
    /// The root value
    pub root: TomlValue,
}

/// A TOML value with inferred type information
#[derive(Debug, Clone)]
pub struct TomlValue {
    /// The inferred type
    pub value_type: TomlType,
    /// For tables: field name -> field value
    pub fields: HashMap<String, TomlValue>,
    /// For arrays: the element type and whether all elements have same type
    pub array_element_type: Option<Box<TomlType>>,
    /// Original TOML value for reference
    pub original: Value,
}

impl TomlValue {
    /// Create a new TomlValue from a TOML Value
    pub fn from_value(value: Value) -> Self {
        let value_type = Self::infer_type(&value);
        let mut fields = HashMap::new();
        let mut array_element_type = None;

        match &value {
            Value::Table(table) => {
                for (key, val) in table {
                    fields.insert(key.clone(), TomlValue::from_value(val.clone()));
                }
            }
            Value::Array(arr) => {
                if !arr.is_empty() {
                    // Infer the common type from array elements
                    let elem_type = Self::infer_array_type(arr);
                    array_element_type = Some(Box::new(elem_type));
                }
            }
            _ => {}
        }

        TomlValue {
            value_type,
            fields,
            array_element_type,
            original: value,
        }
    }

    /// Infer the type from a TOML Value
    fn infer_type(value: &Value) -> TomlType {
        match value {
            Value::String(_) => TomlType::String,
            Value::Integer(_) => TomlType::Integer,
            Value::Float(_) => TomlType::Float,
            Value::Boolean(_) => TomlType::Boolean,
            Value::Datetime(_) => TomlType::Datetime,
            Value::Array(arr) => {
                let elem_type = if arr.is_empty() {
                    TomlType::String // default to string for empty arrays
                } else {
                    Self::infer_array_type(arr)
                };
                TomlType::Array(Box::new(elem_type))
            }
            Value::Table(_) => TomlType::Table,
        }
    }

    /// Infer the common type from an array of values
    fn infer_array_type(arr: &[Value]) -> TomlType {
        if arr.is_empty() {
            return TomlType::String;
        }

        // Check if all elements have the same type
        let first_type = Self::infer_type(&arr[0]);
        let all_same = arr.iter().all(|v| Self::infer_type(v) == first_type);

        if all_same {
            first_type
        } else {
            // Mixed types - we'll need to handle this as a union or any type
            // For now, default to the first element's type
            first_type
        }
    }

    /// Check if this is a table (record)
    pub fn is_table(&self) -> bool {
        matches!(self.value_type, TomlType::Table)
    }

    /// Check if this is an array
    pub fn is_array(&self) -> bool {
        matches!(self.value_type, TomlType::Array(_))
    }
}
