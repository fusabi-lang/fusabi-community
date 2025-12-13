//! GraphQL Type Provider
//!
//! Generates Fusabi types from GraphQL introspection schemas.

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, TypeGenerator, NamingStrategy,
    ProviderError, ProviderResult,
};

/// GraphQL type provider
pub struct GraphQLProvider {
    generator: TypeGenerator,
}

impl GraphQLProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }
}

impl Default for GraphQLProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for GraphQLProvider {
    fn name(&self) -> &str {
        "GraphQLProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        // Parse GraphQL introspection response
        let json_str = if source.starts_with('{') {
            source.to_string()
        } else if source.starts_with("file://") {
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        } else {
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        };

        let value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        Ok(Schema::Custom(serde_json::to_string(&value).unwrap()))
    }

    fn generate_types(&self, _schema: &Schema, _namespace: &str) -> ProviderResult<GeneratedTypes> {
        // TODO: Implement full GraphQL introspection parsing
        Ok(GeneratedTypes::new())
    }
}
