//! TOML parser

use crate::types::{TomlSchema, TomlValue};
use fusabi_type_providers::{ProviderError, ProviderResult};

/// Parse a TOML configuration from a TOML string
pub fn parse_toml(toml_str: &str) -> ProviderResult<TomlSchema> {
    let value: toml::Value = toml::from_str(toml_str)
        .map_err(|e| ProviderError::ParseError(format!("Invalid TOML: {}", e)))?;

    Ok(TomlSchema {
        root: TomlValue::from_value(value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let toml = r#"
            name = "test"
            port = 8080
            enabled = true
        "#;

        let schema = parse_toml(toml).unwrap();
        assert!(schema.root.is_table());
        assert_eq!(schema.root.fields.len(), 3);
    }

    #[test]
    fn test_parse_nested_tables() {
        let toml = r#"
            [database]
            host = "localhost"
            port = 5432

            [server]
            host = "0.0.0.0"
            port = 8080
        "#;

        let schema = parse_toml(toml).unwrap();
        assert!(schema.root.is_table());
        assert!(schema.root.fields.contains_key("database"));
        assert!(schema.root.fields.contains_key("server"));
    }

    #[test]
    fn test_parse_arrays() {
        let toml = r#"
            ports = [8080, 8081, 8082]
            tags = ["rust", "toml", "config"]
        "#;

        let schema = parse_toml(toml).unwrap();
        assert!(schema.root.fields.contains_key("ports"));
        assert!(schema.root.fields.contains_key("tags"));
    }

    #[test]
    fn test_parse_inline_tables() {
        let toml = r#"
            server = { host = "localhost", port = 8080 }
        "#;

        let schema = parse_toml(toml).unwrap();
        assert!(schema.root.fields.contains_key("server"));
        let server = &schema.root.fields["server"];
        assert!(server.is_table());
    }

    #[test]
    fn test_parse_array_of_tables() {
        let toml = r#"
            [[servers]]
            host = "localhost"
            port = 8080

            [[servers]]
            host = "0.0.0.0"
            port = 8081
        "#;

        let schema = parse_toml(toml).unwrap();
        assert!(schema.root.fields.contains_key("servers"));
        assert!(schema.root.fields["servers"].is_array());
    }
}
