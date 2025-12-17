//! OBI/eBPF Type Provider
//!
//! Generates Fusabi types from OBI (Observability Binary Interface) schemas
//! for eBPF-based observability applications like the Hibana agent.
//!
//! # Features
//!
//! - Built-in embedded event types for common eBPF use cases
//! - Support for custom eBPF event structures
//! - Type-safe mapping from kernel/eBPF types to Fusabi types
//! - Comprehensive event categories: syscall, network, file, process, security
//!
//! # Example
//!
//! ```rust,ignore
//! use fusabi_provider_obi::ObiProvider;
//! use fusabi_type_providers::{TypeProvider, ProviderParams};
//!
//! let provider = ObiProvider::new();
//!
//! // Use embedded syscall events
//! let schema = provider.resolve_schema("embedded:syscall", &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "Syscall")?;
//!
//! // Or load custom schema from file
//! let schema = provider.resolve_schema("my_events.json", &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "MyEvents")?;
//! ```

mod parser;
mod types;

pub use types::{
    ObiSchema, ObiStruct, ObiEnum, ObiField, ObiEnumVariant,
    ObiType, ObiPrimitiveType, EventCategory,
};

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, DuDef, VariantDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};

/// OBI type provider for eBPF event structures
pub struct ObiProvider {
    generator: TypeGenerator,
}

impl ObiProvider {
    /// Create a new OBI provider
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Generate types from an OBI schema
    fn generate_from_schema(
        &self,
        schema: &ObiSchema,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        // Validate schema first
        parser::validate_schema(schema)?;

        let mut result = GeneratedTypes::new();

        // Create a module for the namespace if we have definitions
        if !schema.structs.is_empty() || !schema.enums.is_empty() {
            let mut module = GeneratedModule::new(vec![namespace.to_string()]);

            // Generate enum definitions first (they may be referenced by structs)
            for (_enum_name, obi_enum) in &schema.enums {
                let type_def = self.enum_to_typedef(obi_enum)?;
                module.types.push(type_def);
            }

            // Generate struct definitions
            for (_struct_name, obi_struct) in &schema.structs {
                let type_def = self.struct_to_typedef(obi_struct)?;

                // For embedded mode, add structs as root types
                if schema.is_embedded() {
                    result.root_types.push(type_def.clone());
                }

                module.types.push(type_def);
            }

            if !module.types.is_empty() {
                result.modules.push(module);
            }
        }

        Ok(result)
    }

    /// Convert an OBI struct to a Fusabi RecordDef
    fn struct_to_typedef(&self, obi_struct: &ObiStruct) -> ProviderResult<TypeDefinition> {
        let mut fields = Vec::new();

        for field in &obi_struct.fields {
            let type_expr = self.obi_type_to_type_expr(&field.field_type)?;
            fields.push((field.name.clone(), type_expr));
        }

        Ok(TypeDefinition::Record(RecordDef {
            name: self.generator.naming.apply(&obi_struct.name),
            fields,
        }))
    }

    /// Convert an OBI enum to a Fusabi DuDef
    fn enum_to_typedef(&self, obi_enum: &ObiEnum) -> ProviderResult<TypeDefinition> {
        let variants = obi_enum
            .variants
            .iter()
            .map(|v| VariantDef::new_simple(self.generator.naming.apply(&v.name)))
            .collect();

        Ok(TypeDefinition::Du(DuDef {
            name: self.generator.naming.apply(&obi_enum.name),
            variants,
        }))
    }

    /// Convert an OBI type to a Fusabi TypeExpr
    fn obi_type_to_type_expr(&self, obi_type: &ObiType) -> ProviderResult<TypeExpr> {
        match obi_type {
            ObiType::Primitive { prim_type } => {
                Ok(TypeExpr::Named(self.primitive_to_fusabi_type(prim_type)))
            }
            ObiType::Array { element_type, size: _ } => {
                let elem_expr = self.obi_type_to_type_expr(element_type)?;
                // For fixed arrays, we use list for now
                // TODO: Consider adding array type to Fusabi
                Ok(TypeExpr::Named(format!("{} list", elem_expr)))
            }
            ObiType::List { element_type } => {
                let elem_expr = self.obi_type_to_type_expr(element_type)?;
                Ok(TypeExpr::Named(format!("{} list", elem_expr)))
            }
            ObiType::Struct { name } => {
                Ok(TypeExpr::Named(self.generator.naming.apply(name)))
            }
            ObiType::Enum { name } => {
                Ok(TypeExpr::Named(self.generator.naming.apply(name)))
            }
            ObiType::Option { inner_type } => {
                let inner_expr = self.obi_type_to_type_expr(inner_type)?;
                Ok(TypeExpr::Named(format!("{} option", inner_expr)))
            }
        }
    }

    /// Map OBI primitive types to Fusabi type names
    fn primitive_to_fusabi_type(&self, prim_type: &ObiPrimitiveType) -> String {
        match prim_type {
            ObiPrimitiveType::U8 => "int",
            ObiPrimitiveType::U16 => "int",
            ObiPrimitiveType::U32 => "int",
            ObiPrimitiveType::U64 => "int",
            ObiPrimitiveType::I8 => "int",
            ObiPrimitiveType::I16 => "int",
            ObiPrimitiveType::I32 => "int",
            ObiPrimitiveType::I64 => "int",
            ObiPrimitiveType::Bool => "bool",
            ObiPrimitiveType::String => "string",
            ObiPrimitiveType::Ipv4Addr => "string", // Can be represented as dotted decimal
            ObiPrimitiveType::Ipv6Addr => "string", // Can be represented as colon-hex
            ObiPrimitiveType::Pid => "int",
            ObiPrimitiveType::Timestamp => "int", // Nanoseconds as integer
        }
        .to_string()
    }
}

impl Default for ObiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for ObiProvider {
    fn name(&self) -> &str {
        "ObiProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        let obi_schema = parser::parse_from_source(source)?;

        // Validate the schema
        parser::validate_schema(&obi_schema)?;

        // Convert to JSON for Schema::JsonSchema variant
        let json_value = serde_json::to_value(&obi_schema)
            .map_err(|e| ProviderError::ParseError(format!("Failed to serialize OBI schema: {}", e)))?;

        Ok(Schema::JsonSchema(json_value))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::JsonSchema(value) => {
                // Deserialize back to ObiSchema
                let obi_schema: ObiSchema = serde_json::from_value(value.clone())
                    .map_err(|e| ProviderError::ParseError(format!("Invalid OBI schema: {}", e)))?;

                self.generate_from_schema(&obi_schema, namespace)
            }
            _ => Err(ProviderError::ParseError("Expected OBI schema (JSON format)".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_syscall_event() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:syscall", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Syscall").unwrap();

        assert!(!types.root_types.is_empty());
        // Should have SyscallEvent as root type
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "SyscallEvent");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_generate_network_event() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:network", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Network").unwrap();

        assert!(!types.root_types.is_empty());
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "NetworkEvent");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_generate_file_event() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:file", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "File").unwrap();

        assert!(!types.root_types.is_empty());
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "FileEvent");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_generate_process_event() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:process", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Process").unwrap();

        // Should have both ProcessEvent struct and ProcessEventType enum
        assert!(!types.root_types.is_empty());
        assert!(!types.modules.is_empty());

        // Check for ProcessEvent
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "ProcessEvent");
        } else {
            panic!("Expected Record type definition");
        }

        // Check for enum in module types
        let has_enum = types.modules[0].types.iter().any(|t| {
            matches!(t, TypeDefinition::Du(du) if du.name == "ProcessEventType")
        });
        assert!(has_enum);
    }

    #[test]
    fn test_generate_all_events() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:all", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Events").unwrap();

        // Should have all event types
        assert!(types.root_types.len() >= 4);

        let event_names: Vec<String> = types.root_types.iter()
            .filter_map(|t| {
                if let TypeDefinition::Record(record) = t {
                    Some(record.name.clone())
                } else {
                    None
                }
            })
            .collect();

        assert!(event_names.contains(&"SyscallEvent".to_string()));
        assert!(event_names.contains(&"NetworkEvent".to_string()));
        assert!(event_names.contains(&"FileEvent".to_string()));
        assert!(event_names.contains(&"ProcessEvent".to_string()));
    }

    #[test]
    fn test_generate_custom_schema() {
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
                        },
                        {
                            "name": "name",
                            "type": { "kind": "primitive", "type": "string" }
                        },
                        {
                            "name": "active",
                            "type": { "kind": "primitive", "type": "bool" }
                        }
                    ]
                }
            }
        }"#;

        let provider = ObiProvider::new();
        let schema = provider.resolve_schema(json, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Custom").unwrap();

        assert!(!types.modules.is_empty());
        assert!(types.modules[0].types.iter().any(|t| {
            matches!(t, TypeDefinition::Record(record) if record.name == "CustomEvent")
        }));
    }

    #[test]
    fn test_type_with_enum() {
        let json = r#"{
            "version": "1.0",
            "mode": "custom",
            "enums": {
                "Status": {
                    "name": "Status",
                    "variants": [
                        { "name": "Active", "value": 1 },
                        { "name": "Inactive", "value": 2 }
                    ]
                }
            },
            "structs": {
                "Event": {
                    "name": "Event",
                    "fields": [
                        {
                            "name": "status",
                            "type": { "kind": "enum", "name": "Status" }
                        }
                    ]
                }
            }
        }"#;

        let provider = ObiProvider::new();
        let schema = provider.resolve_schema(json, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Test").unwrap();

        assert!(!types.modules.is_empty());
        let module = &types.modules[0];

        // Should have both enum and struct
        assert!(module.types.iter().any(|t| {
            matches!(t, TypeDefinition::Du(du) if du.name == "Status")
        }));
        assert!(module.types.iter().any(|t| {
            matches!(t, TypeDefinition::Record(record) if record.name == "Event")
        }));
    }

    #[test]
    fn test_syscall_event_fields() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:syscall", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Syscall").unwrap();

        // Get the SyscallEvent record
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "SyscallEvent");
            assert_eq!(record.fields.len(), 5);

            // Check field names
            let field_names: Vec<&str> = record.fields.iter()
                .map(|(name, _)| name.as_str())
                .collect();

            assert!(field_names.contains(&"pid"));
            assert!(field_names.contains(&"tid"));
            assert!(field_names.contains(&"syscall_nr"));
            assert!(field_names.contains(&"ret"));
            assert!(field_names.contains(&"timestamp"));
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_network_event_fields() {
        let provider = ObiProvider::new();
        let schema = provider.resolve_schema("embedded:network", &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Network").unwrap();

        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "NetworkEvent");

            let field_names: Vec<&str> = record.fields.iter()
                .map(|(name, _)| name.as_str())
                .collect();

            assert!(field_names.contains(&"pid"));
            assert!(field_names.contains(&"saddr"));
            assert!(field_names.contains(&"daddr"));
            assert!(field_names.contains(&"sport"));
            assert!(field_names.contains(&"dport"));
            assert!(field_names.contains(&"protocol"));
        } else {
            panic!("Expected Record type definition");
        }
    }
}
