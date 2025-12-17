//! OBI schema parser
//!
//! Parses OBI schema definitions from JSON format or generates
//! embedded schemas for built-in Hibana event types.

use crate::types::{ObiSchema, EventCategory};
use fusabi_type_providers::{ProviderError, ProviderResult};

/// Parse an OBI schema from JSON string
pub fn parse_obi_schema(json: &str) -> ProviderResult<ObiSchema> {
    serde_json::from_str(json)
        .map_err(|e| ProviderError::ParseError(format!("Invalid OBI JSON: {}", e)))
}

/// Parse an OBI schema from a source specifier
///
/// Supported formats:
/// - "embedded:syscall" - Built-in syscall events
/// - "embedded:network" - Built-in network events
/// - "embedded:file" - Built-in file events
/// - "embedded:process" - Built-in process events
/// - "embedded:all" - All built-in events
/// - JSON string starting with '{'
/// - File path (with or without "file://" prefix)
pub fn parse_from_source(source: &str) -> ProviderResult<ObiSchema> {
    // Handle embedded schemas
    if let Some(category_str) = source.strip_prefix("embedded:") {
        let category = match category_str.to_lowercase().as_str() {
            "syscall" => EventCategory::Syscall,
            "network" => EventCategory::Network,
            "file" => EventCategory::File,
            "process" => EventCategory::Process,
            "security" => EventCategory::Security,
            "all" | "custom" => EventCategory::Custom,
            _ => {
                return Err(ProviderError::ParseError(format!(
                    "Unknown embedded category: {}. Valid options: syscall, network, file, process, security, all",
                    category_str
                )))
            }
        };

        return Ok(crate::types::embedded::get_schema(category));
    }

    // Handle inline JSON
    if source.trim().starts_with('{') {
        return parse_obi_schema(source);
    }

    // Handle file paths
    let path = source.strip_prefix("file://").unwrap_or(source);
    let json_str = std::fs::read_to_string(path)
        .map_err(|e| ProviderError::IoError(format!("Failed to read {}: {}", path, e)))?;

    parse_obi_schema(&json_str)
}

/// Validate an OBI schema for correctness
pub fn validate_schema(schema: &ObiSchema) -> ProviderResult<()> {
    // Check for struct/enum reference validity
    for (struct_name, obi_struct) in &schema.structs {
        for field in &obi_struct.fields {
            validate_type_reference(&field.field_type, schema, struct_name)?;
        }
    }

    Ok(())
}

/// Validate that type references point to valid structs/enums
fn validate_type_reference(
    obi_type: &crate::types::ObiType,
    schema: &ObiSchema,
    context: &str,
) -> ProviderResult<()> {
    use crate::types::ObiType;

    match obi_type {
        ObiType::Struct { name } => {
            if !schema.structs.contains_key(name) {
                return Err(ProviderError::ParseError(format!(
                    "Struct '{}' referenced in '{}' not found in schema",
                    name, context
                )));
            }
        }
        ObiType::Enum { name } => {
            if !schema.enums.contains_key(name) {
                return Err(ProviderError::ParseError(format!(
                    "Enum '{}' referenced in '{}' not found in schema",
                    name, context
                )));
            }
        }
        ObiType::Array { element_type, .. } | ObiType::List { element_type } => {
            validate_type_reference(element_type, schema, context)?;
        }
        ObiType::Option { inner_type } => {
            validate_type_reference(inner_type, schema, context)?;
        }
        ObiType::Primitive { .. } => {
            // Primitives are always valid
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_embedded_syscall() {
        let schema = parse_from_source("embedded:syscall").unwrap();
        assert_eq!(schema.mode, "embedded");
        assert!(schema.structs.contains_key("SyscallEvent"));
        assert_eq!(schema.structs.len(), 1);
    }

    #[test]
    fn test_parse_embedded_network() {
        let schema = parse_from_source("embedded:network").unwrap();
        assert!(schema.structs.contains_key("NetworkEvent"));
    }

    #[test]
    fn test_parse_embedded_file() {
        let schema = parse_from_source("embedded:file").unwrap();
        assert!(schema.structs.contains_key("FileEvent"));
    }

    #[test]
    fn test_parse_embedded_process() {
        let schema = parse_from_source("embedded:process").unwrap();
        assert!(schema.structs.contains_key("ProcessEvent"));
        assert!(schema.enums.contains_key("ProcessEventType"));
    }

    #[test]
    fn test_parse_embedded_all() {
        let schema = parse_from_source("embedded:all").unwrap();
        assert!(schema.structs.contains_key("SyscallEvent"));
        assert!(schema.structs.contains_key("NetworkEvent"));
        assert!(schema.structs.contains_key("FileEvent"));
        assert!(schema.structs.contains_key("ProcessEvent"));
    }

    #[test]
    fn test_parse_inline_json() {
        let json = r#"{
            "version": "1.0",
            "mode": "custom",
            "structs": {
                "CustomEvent": {
                    "name": "CustomEvent",
                    "fields": [
                        {
                            "name": "id",
                            "type": { "kind": "primitive", "type": "u64" }
                        }
                    ]
                }
            }
        }"#;

        let schema = parse_from_source(json).unwrap();
        assert_eq!(schema.mode, "custom");
        assert!(schema.structs.contains_key("CustomEvent"));
    }

    #[test]
    fn test_validate_schema_valid() {
        let schema = parse_from_source("embedded:process").unwrap();
        assert!(validate_schema(&schema).is_ok());
    }

    #[test]
    fn test_validate_schema_invalid_struct_ref() {
        let json = r#"{
            "version": "1.0",
            "structs": {
                "Event": {
                    "name": "Event",
                    "fields": [
                        {
                            "name": "data",
                            "type": { "kind": "struct", "name": "NonExistent" }
                        }
                    ]
                }
            }
        }"#;

        let schema = parse_obi_schema(json).unwrap();
        let result = validate_schema(&schema);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_schema_invalid_enum_ref() {
        let json = r#"{
            "version": "1.0",
            "structs": {
                "Event": {
                    "name": "Event",
                    "fields": [
                        {
                            "name": "kind",
                            "type": { "kind": "enum", "name": "NonExistent" }
                        }
                    ]
                }
            }
        }"#;

        let schema = parse_obi_schema(json).unwrap();
        let result = validate_schema(&schema);
        assert!(result.is_err());
    }
}
