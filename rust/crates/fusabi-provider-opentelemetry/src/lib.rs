//! OpenTelemetry Type Provider
//!
//! Generates Fusabi types from OpenTelemetry semantic conventions.

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};

/// OpenTelemetry type provider
pub struct OpenTelemetryProvider {
    generator: TypeGenerator,
}

impl OpenTelemetryProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    fn generate_http_types(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Http".to_string()]);

        // HTTP Client span attributes
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Client".to_string(),
            fields: vec![
                ("requestMethod".to_string(), TypeExpr::Named("string".to_string())),
                ("requestUrl".to_string(), TypeExpr::Named("string option".to_string())),
                ("responseStatusCode".to_string(), TypeExpr::Named("int option".to_string())),
                ("networkProtocolName".to_string(), TypeExpr::Named("string option".to_string())),
                ("networkProtocolVersion".to_string(), TypeExpr::Named("string option".to_string())),
                ("serverAddress".to_string(), TypeExpr::Named("string option".to_string())),
                ("serverPort".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // HTTP Server span attributes
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Server".to_string(),
            fields: vec![
                ("requestMethod".to_string(), TypeExpr::Named("string".to_string())),
                ("route".to_string(), TypeExpr::Named("string option".to_string())),
                ("responseStatusCode".to_string(), TypeExpr::Named("int option".to_string())),
                ("scheme".to_string(), TypeExpr::Named("string option".to_string())),
                ("target".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    fn generate_db_types(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Db".to_string()]);

        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Client".to_string(),
            fields: vec![
                ("system".to_string(), TypeExpr::Named("string".to_string())),
                ("statement".to_string(), TypeExpr::Named("string option".to_string())),
                ("operation".to_string(), TypeExpr::Named("string option".to_string())),
                ("name".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    fn generate_embedded_types(&self, namespace: &str) -> GeneratedTypes {
        let mut result = GeneratedTypes::new();
        result.modules.push(self.generate_http_types(namespace));
        result.modules.push(self.generate_db_types(namespace));
        result
    }
}

impl Default for OpenTelemetryProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for OpenTelemetryProvider {
    fn name(&self) -> &str {
        "OpenTelemetryProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        if source == "embedded" {
            return Ok(Schema::Custom("embedded".to_string()));
        }

        Err(ProviderError::InvalidSource(format!(
            "OpenTelemetry provider currently only supports 'embedded' source, got: {}",
            source
        )))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(s) if s == "embedded" => {
                Ok(self.generate_embedded_types(namespace))
            }
            _ => Err(ProviderError::ParseError("Expected OpenTelemetry schema".to_string())),
        }
    }
}
