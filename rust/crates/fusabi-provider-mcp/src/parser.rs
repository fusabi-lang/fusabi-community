//! MCP schema parser
//!
//! Parses MCP server manifests and configurations into structured types.

use crate::types::{
    JsonSchemaObject, JsonSchemaProperty, McpSchema, PromptArgument, PromptDefinition,
    ResourceDefinition, ToolDefinition, TypeDefinition, TypeKind,
};
use fusabi_type_providers::{ProviderError, ProviderResult};
use std::collections::HashMap;

/// Parse an MCP schema from a JSON string
pub fn parse_mcp_schema(json: &str) -> ProviderResult<McpSchema> {
    let value: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| ProviderError::ParseError(format!("Invalid JSON: {}", e)))?;

    parse_schema_value(&value)
}

/// Parse an MCP schema from a serde_json::Value
pub fn parse_schema_value(value: &serde_json::Value) -> ProviderResult<McpSchema> {
    let obj = value
        .as_object()
        .ok_or_else(|| ProviderError::ParseError("MCP schema must be an object".to_string()))?;

    let mut schema = McpSchema::default();

    // Parse tools
    if let Some(tools) = obj.get("tools").and_then(|v| v.as_array()) {
        schema.tools = tools
            .iter()
            .map(parse_tool_definition)
            .collect::<ProviderResult<_>>()?;
    }

    // Parse resources
    if let Some(resources) = obj.get("resources").and_then(|v| v.as_array()) {
        schema.resources = resources
            .iter()
            .map(parse_resource_definition)
            .collect::<ProviderResult<_>>()?;
    }

    // Parse prompts
    if let Some(prompts) = obj.get("prompts").and_then(|v| v.as_array()) {
        schema.prompts = prompts
            .iter()
            .map(parse_prompt_definition)
            .collect::<ProviderResult<_>>()?;
    }

    // Parse definitions/types
    if let Some(defs) = obj.get("definitions").and_then(|v| v.as_object()) {
        for (name, def_value) in defs {
            let type_def = parse_type_definition(name, def_value)?;
            schema.definitions.insert(name.clone(), type_def);
        }
    }

    Ok(schema)
}

/// Parse a tool definition
fn parse_tool_definition(value: &serde_json::Value) -> ProviderResult<ToolDefinition> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("Tool definition must be an object".to_string())
    })?;

    let name = obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ProviderError::ParseError("Tool must have a name".to_string()))?
        .to_string();

    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);

    let input_schema = obj
        .get("inputSchema")
        .map(parse_json_schema_object)
        .transpose()?;

    Ok(ToolDefinition {
        name,
        description,
        input_schema,
    })
}

/// Parse a resource definition
fn parse_resource_definition(value: &serde_json::Value) -> ProviderResult<ResourceDefinition> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("Resource definition must be an object".to_string())
    })?;

    let uri = obj
        .get("uri")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ProviderError::ParseError("Resource must have a uri".to_string()))?
        .to_string();

    let name = obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ProviderError::ParseError("Resource must have a name".to_string()))?
        .to_string();

    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);

    let mime_type = obj
        .get("mimeType")
        .and_then(|v| v.as_str())
        .map(String::from);

    Ok(ResourceDefinition {
        uri,
        name,
        description,
        mime_type,
    })
}

/// Parse a prompt definition
fn parse_prompt_definition(value: &serde_json::Value) -> ProviderResult<PromptDefinition> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("Prompt definition must be an object".to_string())
    })?;

    let name = obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ProviderError::ParseError("Prompt must have a name".to_string()))?
        .to_string();

    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);

    let arguments = if let Some(args) = obj.get("arguments").and_then(|v| v.as_array()) {
        args.iter()
            .map(parse_prompt_argument)
            .collect::<ProviderResult<_>>()?
    } else {
        Vec::new()
    };

    Ok(PromptDefinition {
        name,
        description,
        arguments,
    })
}

/// Parse a prompt argument
fn parse_prompt_argument(value: &serde_json::Value) -> ProviderResult<PromptArgument> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("Prompt argument must be an object".to_string())
    })?;

    let name = obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ProviderError::ParseError("Argument must have a name".to_string()))?
        .to_string();

    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);

    let required = obj
        .get("required")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    Ok(PromptArgument {
        name,
        description,
        required,
    })
}

/// Parse a JSON Schema object (for tool input schemas)
fn parse_json_schema_object(value: &serde_json::Value) -> ProviderResult<JsonSchemaObject> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("JSON Schema must be an object".to_string())
    })?;

    let schema_type = obj.get("type").and_then(|v| v.as_str()).map(String::from);

    let properties = if let Some(props) = obj.get("properties").and_then(|v| v.as_object()) {
        props
            .iter()
            .map(|(k, v)| parse_json_schema_property(v).map(|prop| (k.clone(), prop)))
            .collect::<ProviderResult<_>>()?
    } else {
        HashMap::new()
    };

    let required = if let Some(req) = obj.get("required").and_then(|v| v.as_array()) {
        req.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    } else {
        Vec::new()
    };

    let additional_properties = obj
        .get("additionalProperties")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    Ok(JsonSchemaObject {
        schema_type,
        properties,
        required,
        additional_properties,
    })
}

/// Parse a JSON Schema property
fn parse_json_schema_property(value: &serde_json::Value) -> ProviderResult<JsonSchemaProperty> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("JSON Schema property must be an object".to_string())
    })?;

    let property_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("any")
        .to_string();

    let description = obj
        .get("description")
        .and_then(|v| v.as_str())
        .map(String::from);

    let enum_values = if let Some(enum_val) = obj.get("enum").and_then(|v| v.as_array()) {
        enum_val.clone()
    } else {
        Vec::new()
    };

    let items = obj
        .get("items")
        .map(|v| parse_json_schema_property(v).map(Box::new))
        .transpose()?;

    let properties = if let Some(props) = obj.get("properties").and_then(|v| v.as_object()) {
        props
            .iter()
            .map(|(k, v)| parse_json_schema_property(v).map(|prop| (k.clone(), prop)))
            .collect::<ProviderResult<_>>()?
    } else {
        HashMap::new()
    };

    let default = obj.get("default").cloned();

    Ok(JsonSchemaProperty {
        property_type,
        description,
        enum_values,
        items,
        properties,
        default,
    })
}

/// Parse a type definition
fn parse_type_definition(
    name: &str,
    value: &serde_json::Value,
) -> ProviderResult<TypeDefinition> {
    let obj = value.as_object().ok_or_else(|| {
        ProviderError::ParseError("Type definition must be an object".to_string())
    })?;

    let type_str = obj.get("type").and_then(|v| v.as_str());

    let kind = match type_str {
        Some("object") => {
            let properties = if let Some(props) = obj.get("properties").and_then(|v| v.as_object())
            {
                props
                    .iter()
                    .map(|(k, v)| parse_json_schema_property(v).map(|prop| (k.clone(), prop)))
                    .collect::<ProviderResult<_>>()?
            } else {
                HashMap::new()
            };

            let required = if let Some(req) = obj.get("required").and_then(|v| v.as_array()) {
                req.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            } else {
                Vec::new()
            };

            TypeKind::Object {
                properties,
                required,
            }
        }
        Some("string") if obj.contains_key("enum") => {
            let values = obj
                .get("enum")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            TypeKind::Enum { values }
        }
        _ if obj.contains_key("oneOf") => {
            let variants = obj
                .get("oneOf")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .enumerate()
                        .map(|(i, v)| {
                            let variant_name = format!("{}Variant{}", name, i);
                            parse_type_definition(&variant_name, v)
                        })
                        .collect::<ProviderResult<_>>()
                })
                .transpose()?
                .unwrap_or_default();

            TypeKind::Union { variants }
        }
        _ => {
            // Default to object type
            TypeKind::Object {
                properties: HashMap::new(),
                required: Vec::new(),
            }
        }
    };

    Ok(TypeDefinition {
        name: name.to_string(),
        kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tool_definition() {
        let json = r#"{
            "name": "get_weather",
            "description": "Get the current weather",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state"
                    }
                },
                "required": ["location"]
            }
        }"#;

        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let tool = parse_tool_definition(&value).unwrap();

        assert_eq!(tool.name, "get_weather");
        assert!(tool.description.is_some());
        assert!(tool.input_schema.is_some());
    }

    #[test]
    fn test_parse_resource_definition() {
        let json = r#"{
            "uri": "file:///path/to/resource",
            "name": "example_resource",
            "description": "An example resource",
            "mimeType": "text/plain"
        }"#;

        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let resource = parse_resource_definition(&value).unwrap();

        assert_eq!(resource.uri, "file:///path/to/resource");
        assert_eq!(resource.name, "example_resource");
        assert_eq!(resource.mime_type, Some("text/plain".to_string()));
    }

    #[test]
    fn test_parse_prompt_definition() {
        let json = r#"{
            "name": "summarize",
            "description": "Summarize text",
            "arguments": [
                {
                    "name": "text",
                    "description": "The text to summarize",
                    "required": true
                }
            ]
        }"#;

        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let prompt = parse_prompt_definition(&value).unwrap();

        assert_eq!(prompt.name, "summarize");
        assert_eq!(prompt.arguments.len(), 1);
        assert_eq!(prompt.arguments[0].name, "text");
        assert!(prompt.arguments[0].required);
    }
}
