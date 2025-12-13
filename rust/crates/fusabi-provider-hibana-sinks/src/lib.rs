//! Hibana Sinks Type Provider
//!
//! Generates Fusabi types for Hibana observability agent data sinks.
//! Hibana is a Fusabi-powered observability agent that supports various
//! destinations for metrics, logs, and traces.

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};

/// Hibana Sinks type provider
pub struct HibanaSinksProvider {
    #[allow(dead_code)]
    generator: TypeGenerator,
}

impl HibanaSinksProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Generate metrics sink types
    fn generate_metrics_sinks(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Metrics".to_string()]);

        // Prometheus Remote Write sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "PrometheusRemoteWrite".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("headers".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("compressionEnabled".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // InfluxDB sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "InfluxDb".to_string(),
            fields: vec![
                ("url".to_string(), TypeExpr::Named("string".to_string())),
                ("database".to_string(), TypeExpr::Named("string".to_string())),
                ("org".to_string(), TypeExpr::Named("string option".to_string())),
                ("bucket".to_string(), TypeExpr::Named("string option".to_string())),
                ("token".to_string(), TypeExpr::Named("string option".to_string())),
                ("username".to_string(), TypeExpr::Named("string option".to_string())),
                ("password".to_string(), TypeExpr::Named("string option".to_string())),
                ("precision".to_string(), TypeExpr::Named("string option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // Datadog sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Datadog".to_string(),
            fields: vec![
                ("apiKey".to_string(), TypeExpr::Named("string".to_string())),
                ("site".to_string(), TypeExpr::Named("string option".to_string())),
                ("endpoint".to_string(), TypeExpr::Named("string option".to_string())),
                ("namespace".to_string(), TypeExpr::Named("string option".to_string())),
                ("tags".to_string(), TypeExpr::Named("List<string> option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        module
    }

    /// Generate logs sink types
    fn generate_logs_sinks(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Logs".to_string()]);

        // Elasticsearch sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Elasticsearch".to_string(),
            fields: vec![
                ("hosts".to_string(), TypeExpr::Named("List<string>".to_string())),
                ("index".to_string(), TypeExpr::Named("string".to_string())),
                ("auth".to_string(), TypeExpr::Named("ElasticsearchAuth option".to_string())),
                ("bulkSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("tlsVerify".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // Elasticsearch auth types
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "ElasticsearchAuth".to_string(),
            fields: vec![
                ("username".to_string(), TypeExpr::Named("string option".to_string())),
                ("password".to_string(), TypeExpr::Named("string option".to_string())),
                ("apiKey".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Loki sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Loki".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("labels".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("tenantId".to_string(), TypeExpr::Named("string option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("auth".to_string(), TypeExpr::Named("LokiAuth option".to_string())),
            ],
        }));

        // Loki auth types
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "LokiAuth".to_string(),
            fields: vec![
                ("username".to_string(), TypeExpr::Named("string option".to_string())),
                ("password".to_string(), TypeExpr::Named("string option".to_string())),
                ("bearerToken".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // S3 sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "S3".to_string(),
            fields: vec![
                ("bucket".to_string(), TypeExpr::Named("string".to_string())),
                ("region".to_string(), TypeExpr::Named("string".to_string())),
                ("prefix".to_string(), TypeExpr::Named("string option".to_string())),
                ("compression".to_string(), TypeExpr::Named("string option".to_string())),
                ("encoding".to_string(), TypeExpr::Named("string option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("accessKeyId".to_string(), TypeExpr::Named("string option".to_string())),
                ("secretAccessKey".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Splunk sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Splunk".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("token".to_string(), TypeExpr::Named("string".to_string())),
                ("index".to_string(), TypeExpr::Named("string option".to_string())),
                ("source".to_string(), TypeExpr::Named("string option".to_string())),
                ("sourceType".to_string(), TypeExpr::Named("string option".to_string())),
                ("host".to_string(), TypeExpr::Named("string option".to_string())),
                ("tlsVerify".to_string(), TypeExpr::Named("bool option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        module
    }

    /// Generate traces sink types
    fn generate_traces_sinks(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Traces".to_string()]);

        // OTLP sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Otlp".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("protocol".to_string(), TypeExpr::Named("string option".to_string())),
                ("headers".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("compression".to_string(), TypeExpr::Named("string option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("tlsVerify".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // Jaeger sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Jaeger".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("agentHost".to_string(), TypeExpr::Named("string option".to_string())),
                ("agentPort".to_string(), TypeExpr::Named("int option".to_string())),
                ("serviceName".to_string(), TypeExpr::Named("string".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("tags".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
            ],
        }));

        // Tempo sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Tempo".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("protocol".to_string(), TypeExpr::Named("string option".to_string())),
                ("auth".to_string(), TypeExpr::Named("TempoAuth option".to_string())),
                ("headers".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // Tempo auth types
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "TempoAuth".to_string(),
            fields: vec![
                ("username".to_string(), TypeExpr::Named("string option".to_string())),
                ("password".to_string(), TypeExpr::Named("string option".to_string())),
                ("bearerToken".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    /// Generate generic sink types
    fn generate_generic_sinks(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Generic".to_string()]);

        // HTTP sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Http".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("method".to_string(), TypeExpr::Named("string option".to_string())),
                ("headers".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("encoding".to_string(), TypeExpr::Named("string option".to_string())),
                ("compression".to_string(), TypeExpr::Named("string option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("tlsVerify".to_string(), TypeExpr::Named("bool option".to_string())),
                ("auth".to_string(), TypeExpr::Named("HttpAuth option".to_string())),
            ],
        }));

        // HTTP auth types
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "HttpAuth".to_string(),
            fields: vec![
                ("basic".to_string(), TypeExpr::Named("BasicAuth option".to_string())),
                ("bearer".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module.types.push(TypeDefinition::Record(RecordDef {
            name: "BasicAuth".to_string(),
            fields: vec![
                ("username".to_string(), TypeExpr::Named("string".to_string())),
                ("password".to_string(), TypeExpr::Named("string".to_string())),
            ],
        }));

        // Kafka sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Kafka".to_string(),
            fields: vec![
                ("brokers".to_string(), TypeExpr::Named("List<string>".to_string())),
                ("topic".to_string(), TypeExpr::Named("string".to_string())),
                ("compression".to_string(), TypeExpr::Named("string option".to_string())),
                ("encoding".to_string(), TypeExpr::Named("string option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("acks".to_string(), TypeExpr::Named("string option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("keyField".to_string(), TypeExpr::Named("string option".to_string())),
                ("auth".to_string(), TypeExpr::Named("KafkaAuth option".to_string())),
            ],
        }));

        // Kafka auth types
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "KafkaAuth".to_string(),
            fields: vec![
                ("saslMechanism".to_string(), TypeExpr::Named("string option".to_string())),
                ("saslUsername".to_string(), TypeExpr::Named("string option".to_string())),
                ("saslPassword".to_string(), TypeExpr::Named("string option".to_string())),
                ("tlsEnabled".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // File sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "File".to_string(),
            fields: vec![
                ("path".to_string(), TypeExpr::Named("string".to_string())),
                ("encoding".to_string(), TypeExpr::Named("string option".to_string())),
                ("compression".to_string(), TypeExpr::Named("string option".to_string())),
                ("maxSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("maxFiles".to_string(), TypeExpr::Named("int option".to_string())),
                ("rotateOnDate".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // Console sink
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Console".to_string(),
            fields: vec![
                ("encoding".to_string(), TypeExpr::Named("string option".to_string())),
                ("format".to_string(), TypeExpr::Named("string option".to_string())),
                ("target".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    /// Generate all embedded sink types
    fn generate_embedded_types(&self, namespace: &str) -> GeneratedTypes {
        let mut result = GeneratedTypes::new();
        result.modules.push(self.generate_metrics_sinks(namespace));
        result.modules.push(self.generate_logs_sinks(namespace));
        result.modules.push(self.generate_traces_sinks(namespace));
        result.modules.push(self.generate_generic_sinks(namespace));
        result
    }
}

impl Default for HibanaSinksProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for HibanaSinksProvider {
    fn name(&self) -> &str {
        "HibanaSinksProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        if source == "embedded" {
            return Ok(Schema::Custom("embedded".to_string()));
        }

        Err(ProviderError::InvalidSource(format!(
            "Hibana Sinks provider currently only supports 'embedded' source, got: {}",
            source
        )))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(s) if s == "embedded" => {
                Ok(self.generate_embedded_types(namespace))
            }
            _ => Err(ProviderError::ParseError("Expected Hibana Sinks schema".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_name() {
        let provider = HibanaSinksProvider::new();
        assert_eq!(provider.name(), "HibanaSinksProvider");
    }

    #[test]
    fn test_resolve_embedded_schema() {
        let provider = HibanaSinksProvider::new();
        let params = ProviderParams::default();
        let result = provider.resolve_schema("embedded", &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_invalid_schema() {
        let provider = HibanaSinksProvider::new();
        let params = ProviderParams::default();
        let result = provider.resolve_schema("invalid", &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_types() {
        let provider = HibanaSinksProvider::new();
        let schema = Schema::Custom("embedded".to_string());
        let result = provider.generate_types(&schema, "Hibana");
        assert!(result.is_ok());

        let types = result.unwrap();
        assert_eq!(types.modules.len(), 4); // Metrics, Logs, Traces, Generic
    }

    #[test]
    fn test_metrics_sinks_module() {
        let provider = HibanaSinksProvider::new();
        let module = provider.generate_metrics_sinks("Hibana");

        assert_eq!(module.path, vec!["Hibana", "Metrics"]);
        assert_eq!(module.types.len(), 3); // PrometheusRemoteWrite, InfluxDb, Datadog
    }

    #[test]
    fn test_logs_sinks_module() {
        let provider = HibanaSinksProvider::new();
        let module = provider.generate_logs_sinks("Hibana");

        assert_eq!(module.path, vec!["Hibana", "Logs"]);
        assert_eq!(module.types.len(), 6); // Elasticsearch, ElasticsearchAuth, Loki, LokiAuth, S3, Splunk
    }

    #[test]
    fn test_traces_sinks_module() {
        let provider = HibanaSinksProvider::new();
        let module = provider.generate_traces_sinks("Hibana");

        assert_eq!(module.path, vec!["Hibana", "Traces"]);
        assert_eq!(module.types.len(), 4); // Otlp, Jaeger, Tempo, TempoAuth
    }

    #[test]
    fn test_generic_sinks_module() {
        let provider = HibanaSinksProvider::new();
        let module = provider.generate_generic_sinks("Hibana");

        assert_eq!(module.path, vec!["Hibana", "Generic"]);
        assert_eq!(module.types.len(), 7); // Http, HttpAuth, BasicAuth, Kafka, KafkaAuth, File, Console
    }
}
