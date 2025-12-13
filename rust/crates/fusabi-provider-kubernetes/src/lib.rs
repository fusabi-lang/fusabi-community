//! Kubernetes Type Provider
//!
//! Generates Fusabi types from Kubernetes OpenAPI schemas.

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};

/// Kubernetes type provider
pub struct KubernetesProvider {
    generator: TypeGenerator,
}

impl KubernetesProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    fn generate_core_types(&self, namespace: &str) -> GeneratedTypes {
        let mut result = GeneratedTypes::new();
        let mut core_module = GeneratedModule::new(vec![namespace.to_string(), "Core".to_string()]);

        // Add ObjectMeta type
        core_module.types.push(TypeDefinition::Record(RecordDef {
            name: "ObjectMeta".to_string(),
            fields: vec![
                ("name".to_string(), TypeExpr::Named("string".to_string())),
                ("namespace".to_string(), TypeExpr::Named("string option".to_string())),
                ("labels".to_string(), TypeExpr::Named("Map<string, string>".to_string())),
                ("annotations".to_string(), TypeExpr::Named("Map<string, string>".to_string())),
            ],
        }));

        // Add TypeMeta
        core_module.types.push(TypeDefinition::Record(RecordDef {
            name: "TypeMeta".to_string(),
            fields: vec![
                ("apiVersion".to_string(), TypeExpr::Named("string".to_string())),
                ("kind".to_string(), TypeExpr::Named("string".to_string())),
            ],
        }));

        result.modules.push(core_module);
        result
    }
}

impl Default for KubernetesProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for KubernetesProvider {
    fn name(&self) -> &str {
        "KubernetesProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        // For now, support "embedded" mode with built-in types
        if source == "embedded" {
            return Ok(Schema::Custom("embedded".to_string()));
        }

        // Support file:// or http:// URLs for OpenAPI specs
        Err(ProviderError::InvalidSource(format!(
            "Kubernetes provider currently only supports 'embedded' source, got: {}",
            source
        )))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(s) if s == "embedded" => {
                Ok(self.generate_core_types(namespace))
            }
            Schema::OpenApi(_) => {
                // TODO: Parse OpenAPI spec for full K8s types
                Ok(self.generate_core_types(namespace))
            }
            _ => Err(ProviderError::ParseError("Expected Kubernetes schema".to_string())),
        }
    }
}
