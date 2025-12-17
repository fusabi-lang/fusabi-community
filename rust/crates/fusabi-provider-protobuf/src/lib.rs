//! Protobuf Type Provider
//!
//! Generates Fusabi types from Protocol Buffer (.proto) definitions.
//!
//! # Example
//!
//! ```rust,ignore
//! use fusabi_provider_protobuf::ProtobufProvider;
//! use fusabi_type_providers::{TypeProvider, ProviderParams};
//!
//! let provider = ProtobufProvider::new();
//! let schema = provider.resolve_schema("schema.proto", &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "MyProto")?;
//! ```

mod parser;
mod types;

pub use types::{ProtoFile, Message, Enum, Field, FieldType, FieldLabel};

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, DuDef, VariantDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};
use std::collections::HashMap;

/// Protobuf type provider
pub struct ProtobufProvider {
    generator: TypeGenerator,
}

impl ProtobufProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Parse a .proto file from string content
    fn parse_proto(&self, content: &str) -> ProviderResult<ProtoFile> {
        parser::parse_proto(content)
    }

    /// Generate types from parsed proto file
    fn generate_from_proto(
        &self,
        proto: &ProtoFile,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        let mut result = GeneratedTypes::new();

        // Create a module for the package if present
        let module_path = if let Some(ref package) = proto.package {
            package.split('.').map(String::from).collect()
        } else {
            vec![namespace.to_string()]
        };

        let mut types_module = GeneratedModule::new(module_path);

        // Build maps for type resolution
        let message_map = proto.build_message_map();
        let enum_map = proto.build_enum_map();

        // Process top-level enums
        for enum_def in &proto.enums {
            types_module.types.push(self.enum_to_typedef(enum_def)?);
        }

        // Process top-level messages
        for message in &proto.messages {
            self.process_message(message, &mut types_module, &message_map, &enum_map)?;
        }

        if !types_module.types.is_empty() {
            result.modules.push(types_module);
        }

        Ok(result)
    }

    /// Process a message and its nested types
    fn process_message(
        &self,
        message: &Message,
        module: &mut GeneratedModule,
        message_map: &HashMap<String, &Message>,
        enum_map: &HashMap<String, &Enum>,
    ) -> ProviderResult<()> {
        // Add nested enums first
        for nested_enum in &message.nested_enums {
            module.types.push(self.enum_to_typedef(nested_enum)?);
        }

        // Add nested messages recursively
        for nested_message in &message.nested_messages {
            self.process_message(nested_message, module, message_map, enum_map)?;
        }

        // Add the message itself
        module.types.push(self.message_to_typedef(message, message_map, enum_map)?);

        Ok(())
    }

    /// Convert a protobuf message to a RecordDef
    fn message_to_typedef(
        &self,
        message: &Message,
        message_map: &HashMap<String, &Message>,
        enum_map: &HashMap<String, &Enum>,
    ) -> ProviderResult<TypeDefinition> {
        let mut fields = Vec::new();

        for field in &message.fields {
            let type_expr = self.field_type_to_type_expr(
                &field.field_type,
                &field.label,
                message_map,
                enum_map,
            )?;
            fields.push((field.name.clone(), type_expr));
        }

        Ok(TypeDefinition::Record(RecordDef {
            name: self.generator.naming.apply(&message.name),
            fields,
        }))
    }

    /// Convert a protobuf enum to a DuDef
    fn enum_to_typedef(&self, enum_def: &Enum) -> ProviderResult<TypeDefinition> {
        let variants = enum_def
            .values
            .iter()
            .map(|v| VariantDef::new_simple(self.generator.naming.apply(&v.name)))
            .collect();

        Ok(TypeDefinition::Du(DuDef {
            name: self.generator.naming.apply(&enum_def.name),
            variants,
        }))
    }

    /// Convert a protobuf field type to a Fusabi TypeExpr
    fn field_type_to_type_expr(
        &self,
        field_type: &FieldType,
        label: &FieldLabel,
        message_map: &HashMap<String, &Message>,
        enum_map: &HashMap<String, &Enum>,
    ) -> ProviderResult<TypeExpr> {
        let base_type = match field_type {
            FieldType::Double | FieldType::Float => TypeExpr::Named("float".to_string()),
            FieldType::Int32 | FieldType::SInt32 | FieldType::SFixed32 => {
                TypeExpr::Named("int".to_string())
            }
            FieldType::Int64 | FieldType::SInt64 | FieldType::SFixed64 => {
                TypeExpr::Named("int64".to_string())
            }
            FieldType::UInt32 | FieldType::Fixed32 => TypeExpr::Named("uint".to_string()),
            FieldType::UInt64 | FieldType::Fixed64 => TypeExpr::Named("uint64".to_string()),
            FieldType::Bool => TypeExpr::Named("bool".to_string()),
            FieldType::String => TypeExpr::Named("string".to_string()),
            FieldType::Bytes => TypeExpr::Named("bytes".to_string()),
            FieldType::Message(type_name) => {
                // Check if it's a known message type
                if message_map.contains_key(type_name) {
                    TypeExpr::Named(self.generator.naming.apply(type_name))
                } else {
                    // Could be a fully qualified name or external reference
                    // For now, use the type name as-is
                    TypeExpr::Named(self.generator.naming.apply(type_name))
                }
            }
            FieldType::Enum(type_name) => {
                // Check if it's a known enum type
                if enum_map.contains_key(type_name) {
                    TypeExpr::Named(self.generator.naming.apply(type_name))
                } else {
                    // External enum reference
                    TypeExpr::Named(self.generator.naming.apply(type_name))
                }
            }
            FieldType::Map(key_type, value_type) => {
                let key_expr = self.field_type_to_type_expr(
                    key_type,
                    &FieldLabel::Required,
                    message_map,
                    enum_map,
                )?;
                let value_expr = self.field_type_to_type_expr(
                    value_type,
                    &FieldLabel::Required,
                    message_map,
                    enum_map,
                )?;
                TypeExpr::Named(format!("Map<{}, {}>", key_expr, value_expr))
            }
        };

        // Apply label modifiers
        match label {
            FieldLabel::Optional => {
                // Wrap in Option for optional fields
                Ok(TypeExpr::Named(format!("{} option", base_type)))
            }
            FieldLabel::Required => Ok(base_type),
            FieldLabel::Repeated => {
                // Wrap in list for repeated fields
                Ok(TypeExpr::Named(format!("{} list", base_type)))
            }
        }
    }
}

impl Default for ProtobufProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for ProtobufProvider {
    fn name(&self) -> &str {
        "ProtobufProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        // Load proto file from path or inline content
        // Check if source looks like inline proto content (contains proto keywords)
        let looks_like_proto = source.contains("syntax") || source.contains("package")
            || source.contains("message ") || source.contains("enum ") || source.contains("service ");

        let proto_content = if looks_like_proto {
            // Inline proto content
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

        // Parse the proto file to validate it
        let _proto_file = self.parse_proto(&proto_content)?;

        // Store the actual proto content directly in the Schema
        // This way we don't need to re-read files or handle paths again
        Ok(Schema::Custom(proto_content))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(proto_content) => {
                // Parse the proto content
                let proto = self.parse_proto(proto_content)?;
                self.generate_from_proto(&proto, namespace)
            }
            _ => Err(ProviderError::ParseError(
                "Expected Protobuf schema".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_message() {
        let provider = ProtobufProvider::new();
        let proto = r#"
            syntax = "proto3";
            package example;

            message Person {
                string name = 1;
                int32 age = 2;
                repeated string emails = 3;
            }
        "#;

        let schema = provider.resolve_schema(proto, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Example").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];
        assert!(!module.types.is_empty());
    }

    #[test]
    fn test_generate_enum() {
        let provider = ProtobufProvider::new();
        let proto = r#"
            syntax = "proto3";

            enum Status {
                UNKNOWN = 0;
                ACTIVE = 1;
                INACTIVE = 2;
            }
        "#;

        let schema = provider.resolve_schema(proto, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Test").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];
        assert_eq!(module.types.len(), 1);

        if let TypeDefinition::Du(du) = &module.types[0] {
            assert_eq!(du.variants.len(), 3);
        } else {
            panic!("Expected DU type");
        }
    }

    #[test]
    fn test_generate_nested_message() {
        let provider = ProtobufProvider::new();
        let proto = r#"
            syntax = "proto3";

            message Outer {
                message Inner {
                    string value = 1;
                }
                Inner inner = 1;
                int32 count = 2;
            }
        "#;

        let schema = provider.resolve_schema(proto, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Test").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];
        // Should have Inner and Outer
        assert_eq!(module.types.len(), 2);
    }

    #[test]
    fn test_generate_map_field() {
        let provider = ProtobufProvider::new();
        let proto = r#"
            syntax = "proto3";

            message Config {
                map<string, int32> settings = 1;
            }
        "#;

        let schema = provider.resolve_schema(proto, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Test").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];
        assert_eq!(module.types.len(), 1);
    }

    #[test]
    fn test_field_labels() {
        let provider = ProtobufProvider::new();
        let proto = r#"
            syntax = "proto3";

            message TestMessage {
                string required_field = 1;
                optional string optional_field = 2;
                repeated string repeated_field = 3;
            }
        "#;

        let schema = provider.resolve_schema(proto, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Test").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];
        assert_eq!(module.types.len(), 1);

        if let TypeDefinition::Record(record) = &module.types[0] {
            assert_eq!(record.fields.len(), 3);
        } else {
            panic!("Expected Record type");
        }
    }

    #[test]
    fn test_comprehensive_proto() {
        let provider = ProtobufProvider::new();
        let proto = r#"
            syntax = "proto3";
            package example.v1;

            enum Status {
                UNKNOWN = 0;
                ACTIVE = 1;
            }

            message User {
                string id = 1;
                string name = 2;
                Status status = 3;
                repeated string tags = 4;
                map<string, int32> scores = 5;

                message Address {
                    string street = 1;
                    string city = 2;
                }

                optional Address address = 6;
            }

            message GetUserRequest {
                string user_id = 1;
            }

            service UserService {
                rpc GetUser(GetUserRequest) returns (User);
            }
        "#;

        let schema = provider.resolve_schema(proto, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Example").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];

        // Check module path matches package
        assert_eq!(module.path, vec!["example", "v1"]);

        // Should have: Status enum, Address, User, GetUserRequest (service is not converted to types)
        assert!(module.types.len() >= 4);

        // Verify we have the Status enum
        let has_enum = module.types.iter().any(|t| {
            matches!(t, TypeDefinition::Du(du) if du.name == "Status")
        });
        assert!(has_enum, "Should have Status enum");

        // Verify we have the User record
        let has_user = module.types.iter().any(|t| {
            matches!(t, TypeDefinition::Record(r) if r.name == "User")
        });
        assert!(has_user, "Should have User record");
    }
}
