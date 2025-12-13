//! MCP (Model Context Protocol) Type Provider
//!
//! Generates Fusabi types from MCP server specifications including tools,
//! resources, prompts, and protocol messages.
//!
//! # Features
//!
//! - Tool definitions with input schemas
//! - Resource definitions with URI templates
//! - Prompt definitions with arguments
//! - Full MCP protocol message types
//! - Content types (text, image, resource)
//! - Embedded mode with built-in MCP types
//!
//! # Example
//!
//! ```rust,ignore
//! use fusabi_provider_mcp::McpProvider;
//! use fusabi_type_providers::{TypeProvider, ProviderParams};
//!
//! let provider = McpProvider::new();
//! let schema = provider.resolve_schema("mcp-schema.json", &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "MyMcpServer")?;
//! ```
//!
//! # Embedded Mode
//!
//! ```rust,ignore
//! let params = ProviderParams::default().with("mode", "embedded");
//! let schema = provider.resolve_schema("", &params)?;
//! let types = provider.generate_types(&schema, "Mcp")?;
//! ```

mod parser;
mod types;

pub use types::{
    ContentType, JsonSchemaObject, JsonSchemaProperty, McpSchema, MessageType, PromptArgument,
    PromptDefinition, ResourceDefinition, ToolDefinition, TypeDefinition, TypeKind,
    EMBEDDED_MCP_TYPES,
};

use fusabi_type_providers::{
    DuDef, GeneratedModule, GeneratedTypes, NamingStrategy, ProviderError, ProviderParams,
    ProviderResult, RecordDef, Schema, TypeExpr, TypeGenerator, TypeProvider,
    TypeDefinition as FusabiTypeDef, VariantDef,
};

/// MCP type provider
pub struct McpProvider {
    generator: TypeGenerator,
}

impl McpProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Parse MCP schema from string
    fn parse_schema(&self, json: &str) -> ProviderResult<types::McpSchema> {
        parser::parse_mcp_schema(json)
    }

    /// Generate types from parsed MCP schema
    fn generate_from_schema(
        &self,
        schema: &types::McpSchema,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        let mut result = GeneratedTypes::new();

        // Generate tool types
        if !schema.tools.is_empty() {
            let mut tools_module = GeneratedModule::new(vec![namespace.to_string(), "tools".to_string()]);

            for tool in &schema.tools {
                if let Some(type_def) = self.generate_tool_type(tool)? {
                    tools_module.types.push(type_def);
                }
            }

            // Add tool union type
            if schema.tools.len() > 1 {
                let tool_union = self.generate_tool_union(&schema.tools)?;
                tools_module.types.push(tool_union);
            }

            result.modules.push(tools_module);
        }

        // Generate resource types
        if !schema.resources.is_empty() {
            let mut resources_module =
                GeneratedModule::new(vec![namespace.to_string(), "resources".to_string()]);

            for resource in &schema.resources {
                if let Some(type_def) = self.generate_resource_type(resource)? {
                    resources_module.types.push(type_def);
                }
            }

            result.modules.push(resources_module);
        }

        // Generate prompt types
        if !schema.prompts.is_empty() {
            let mut prompts_module =
                GeneratedModule::new(vec![namespace.to_string(), "prompts".to_string()]);

            for prompt in &schema.prompts {
                if let Some(type_def) = self.generate_prompt_type(prompt)? {
                    prompts_module.types.push(type_def);
                }
            }

            result.modules.push(prompts_module);
        }

        // Generate custom types
        if !schema.definitions.is_empty() {
            let mut defs_module =
                GeneratedModule::new(vec![namespace.to_string(), "definitions".to_string()]);

            for (name, type_def) in &schema.definitions {
                if let Some(fusabi_def) = self.generate_custom_type(name, type_def)? {
                    defs_module.types.push(fusabi_def);
                }
            }

            result.modules.push(defs_module);
        }

        Ok(result)
    }

    /// Generate embedded MCP protocol types
    fn generate_embedded_types(&self, namespace: &str) -> ProviderResult<GeneratedTypes> {
        let mut result = GeneratedTypes::new();

        // The embedded types are already in Fusabi syntax, so we can return them as a raw module
        // For now, we'll create a note that these should be included
        let mut protocol_module =
            GeneratedModule::new(vec![namespace.to_string(), "protocol".to_string()]);

        // Add a marker type to indicate embedded types should be included
        protocol_module.types.push(FusabiTypeDef::Record(RecordDef {
            name: "__EmbeddedMcpTypes".to_string(),
            fields: vec![("__marker".to_string(), TypeExpr::Named("unit".to_string()))],
        }));

        result.modules.push(protocol_module);
        Ok(result)
    }

    /// Generate type definition for a tool
    fn generate_tool_type(
        &self,
        tool: &types::ToolDefinition,
    ) -> ProviderResult<Option<FusabiTypeDef>> {
        let tool_name = self.generator.naming.apply(&tool.name);

        if let Some(input_schema) = &tool.input_schema {
            // Generate input type
            let input_type_name = format!("{}Input", tool_name);
            let fields = self.schema_object_to_fields(input_schema)?;

            Ok(Some(FusabiTypeDef::Record(RecordDef {
                name: input_type_name,
                fields,
            })))
        } else {
            // No input schema, create a simple marker type
            Ok(Some(FusabiTypeDef::Record(RecordDef {
                name: format!("{}Input", tool_name),
                fields: vec![],
            })))
        }
    }

    /// Generate union type for all tools
    fn generate_tool_union(&self, tools: &[types::ToolDefinition]) -> ProviderResult<FusabiTypeDef> {
        let variants = tools
            .iter()
            .map(|tool| {
                let tool_name = self.generator.naming.apply(&tool.name);
                let input_type = TypeExpr::Named(format!("{}Input", tool_name));
                VariantDef::new(tool_name, vec![input_type])
            })
            .collect();

        Ok(FusabiTypeDef::Du(DuDef {
            name: "ToolCall".to_string(),
            variants,
        }))
    }

    /// Generate type definition for a resource
    fn generate_resource_type(
        &self,
        resource: &types::ResourceDefinition,
    ) -> ProviderResult<Option<FusabiTypeDef>> {
        let resource_name = self.generator.naming.apply(&resource.name);

        let mut fields = vec![
            ("uri".to_string(), TypeExpr::Named("string".to_string())),
            ("name".to_string(), TypeExpr::Named("string".to_string())),
        ];

        if resource.description.is_some() {
            fields.push((
                "description".to_string(),
                TypeExpr::Named("string option".to_string()),
            ));
        }

        if resource.mime_type.is_some() {
            fields.push((
                "mimeType".to_string(),
                TypeExpr::Named("string option".to_string()),
            ));
        }

        Ok(Some(FusabiTypeDef::Record(RecordDef {
            name: format!("{}Resource", resource_name),
            fields,
        })))
    }

    /// Generate type definition for a prompt
    fn generate_prompt_type(
        &self,
        prompt: &types::PromptDefinition,
    ) -> ProviderResult<Option<FusabiTypeDef>> {
        let prompt_name = self.generator.naming.apply(&prompt.name);

        if prompt.arguments.is_empty() {
            // No arguments, simple marker type
            Ok(Some(FusabiTypeDef::Record(RecordDef {
                name: format!("{}Args", prompt_name),
                fields: vec![],
            })))
        } else {
            // Generate args type from arguments
            let fields = prompt
                .arguments
                .iter()
                .map(|arg| {
                    let type_expr = if arg.required {
                        TypeExpr::Named("string".to_string())
                    } else {
                        TypeExpr::Named("string option".to_string())
                    };
                    (arg.name.clone(), type_expr)
                })
                .collect();

            Ok(Some(FusabiTypeDef::Record(RecordDef {
                name: format!("{}Args", prompt_name),
                fields,
            })))
        }
    }

    /// Generate type definition for a custom type
    fn generate_custom_type(
        &self,
        name: &str,
        type_def: &types::TypeDefinition,
    ) -> ProviderResult<Option<FusabiTypeDef>> {
        let type_name = self.generator.naming.apply(name);

        match &type_def.kind {
            TypeKind::Object {
                properties,
                required,
            } => {
                let fields = self.properties_to_fields(properties, required)?;
                Ok(Some(FusabiTypeDef::Record(RecordDef {
                    name: type_name,
                    fields,
                })))
            }
            TypeKind::Enum { values } => {
                let variants = values
                    .iter()
                    .map(|v| VariantDef::new_simple(self.generator.naming.apply(v)))
                    .collect();
                Ok(Some(FusabiTypeDef::Du(DuDef {
                    name: type_name,
                    variants,
                })))
            }
            TypeKind::Union { variants } => {
                let fusabi_variants = variants
                    .iter()
                    .enumerate()
                    .map(|(i, variant)| {
                        let variant_name = if !variant.name.is_empty() {
                            self.generator.naming.apply(&variant.name)
                        } else {
                            format!("Variant{}", i)
                        };
                        // For union variants, we use the type itself
                        VariantDef::new_simple(variant_name)
                    })
                    .collect();
                Ok(Some(FusabiTypeDef::Du(DuDef {
                    name: type_name,
                    variants: fusabi_variants,
                })))
            }
        }
    }

    /// Convert JSON Schema object to record fields
    fn schema_object_to_fields(
        &self,
        schema: &types::JsonSchemaObject,
    ) -> ProviderResult<Vec<(String, TypeExpr)>> {
        self.properties_to_fields(&schema.properties, &schema.required)
    }

    /// Convert properties to record fields
    fn properties_to_fields(
        &self,
        properties: &std::collections::HashMap<String, types::JsonSchemaProperty>,
        required: &[String],
    ) -> ProviderResult<Vec<(String, TypeExpr)>> {
        let mut fields = Vec::new();

        for (prop_name, prop) in properties {
            let type_expr = self.property_to_type_expr(prop)?;
            let is_required = required.contains(prop_name);

            let final_type = if is_required {
                type_expr
            } else {
                TypeExpr::Named(format!("{} option", type_expr))
            };

            fields.push((prop_name.clone(), final_type));
        }

        Ok(fields)
    }

    /// Convert JSON Schema property to TypeExpr
    fn property_to_type_expr(
        &self,
        prop: &types::JsonSchemaProperty,
    ) -> ProviderResult<TypeExpr> {
        // Handle enum
        if !prop.enum_values.is_empty() {
            // For string enums, we use a union type
            let enum_str = prop
                .enum_values
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<_>>()
                .join(" | ");
            return Ok(TypeExpr::Named(enum_str));
        }

        match prop.property_type.as_str() {
            "string" => Ok(TypeExpr::Named("string".to_string())),
            "integer" => Ok(TypeExpr::Named("int".to_string())),
            "number" => Ok(TypeExpr::Named("float".to_string())),
            "boolean" => Ok(TypeExpr::Named("bool".to_string())),
            "null" => Ok(TypeExpr::Named("unit".to_string())),
            "array" => {
                if let Some(items) = &prop.items {
                    let item_type = self.property_to_type_expr(items)?;
                    Ok(TypeExpr::Named(format!("{} list", item_type)))
                } else {
                    Ok(TypeExpr::Named("any list".to_string()))
                }
            }
            "object" => {
                if prop.properties.is_empty() {
                    // Generic object/map
                    Ok(TypeExpr::Named("Map<string, any>".to_string()))
                } else {
                    // Nested object - would need inline record type support
                    Ok(TypeExpr::Named("Map<string, any>".to_string()))
                }
            }
            "any" | _ => Ok(TypeExpr::Named("any".to_string())),
        }
    }
}

impl Default for McpProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for McpProvider {
    fn name(&self) -> &str {
        "McpProvider"
    }

    fn resolve_schema(&self, source: &str, params: &ProviderParams) -> ProviderResult<Schema> {
        // Check for embedded mode
        if params.custom.get("mode") == Some(&"embedded".to_string()) || source.is_empty() {
            // Return embedded schema marker
            return Ok(Schema::Custom("embedded".to_string()));
        }

        // Load from file or parse inline JSON
        let json_str = if source.starts_with('{') || source.starts_with('[') {
            source.to_string()
        } else if source.starts_with("file://") {
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        } else {
            // Treat as file path
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        };

        let _value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        // Store the JSON in the source for later parsing
        Ok(Schema::Custom(json_str))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(content) => {
                if content == "embedded" {
                    // Generate embedded MCP types
                    self.generate_embedded_types(namespace)
                } else {
                    // Parse the JSON content
                    let parsed = self.parse_schema(content)?;
                    self.generate_from_schema(&parsed, namespace)
                }
            }
            _ => Err(ProviderError::ParseError(
                "Expected MCP schema".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tool_types() {
        let provider = McpProvider::new();
        let json = r#"{
            "tools": [
                {
                    "name": "get_weather",
                    "description": "Get current weather",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "location": {
                                "type": "string",
                                "description": "City name"
                            },
                            "units": {
                                "type": "string",
                                "enum": ["celsius", "fahrenheit"]
                            }
                        },
                        "required": ["location"]
                    }
                }
            ]
        }"#;

        let schema = provider
            .resolve_schema(json, &ProviderParams::default())
            .unwrap();
        let types = provider.generate_types(&schema, "Weather").unwrap();

        assert!(!types.modules.is_empty());
    }

    #[test]
    fn test_generate_resource_types() {
        let provider = McpProvider::new();
        let json = r#"{
            "resources": [
                {
                    "uri": "file:///data.json",
                    "name": "data",
                    "description": "Data file",
                    "mimeType": "application/json"
                }
            ]
        }"#;

        let schema = provider
            .resolve_schema(json, &ProviderParams::default())
            .unwrap();
        let types = provider.generate_types(&schema, "Resources").unwrap();

        assert!(!types.modules.is_empty());
    }

    #[test]
    fn test_generate_prompt_types() {
        let provider = McpProvider::new();
        let json = r#"{
            "prompts": [
                {
                    "name": "summarize",
                    "description": "Summarize text",
                    "arguments": [
                        {
                            "name": "text",
                            "description": "Text to summarize",
                            "required": true
                        },
                        {
                            "name": "maxLength",
                            "description": "Max length",
                            "required": false
                        }
                    ]
                }
            ]
        }"#;

        let schema = provider
            .resolve_schema(json, &ProviderParams::default())
            .unwrap();
        let types = provider.generate_types(&schema, "Prompts").unwrap();

        assert!(!types.modules.is_empty());
    }

    #[test]
    fn test_embedded_mode() {
        let provider = McpProvider::new();
        let params = ProviderParams::default();
        // Simulate embedded mode by passing empty source
        let schema = provider.resolve_schema("", &params).unwrap();
        let types = provider.generate_types(&schema, "Mcp").unwrap();

        assert!(!types.modules.is_empty());
    }
}
