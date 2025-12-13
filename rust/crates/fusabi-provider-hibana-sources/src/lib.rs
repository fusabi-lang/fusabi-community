//! Hibana Sources Type Provider
//!
//! Generates Fusabi types for Hibana observability agent data sources.
//! Hibana is a Fusabi-powered observability agent that collects metrics, logs, traces, and events.

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};

/// Hibana Sources type provider
pub struct HibanaSourcesProvider {
    generator: TypeGenerator,
}

impl HibanaSourcesProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    fn generate_metrics_sources(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Metrics".to_string()]);

        // Prometheus scrape source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "PrometheusScrape".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("interval".to_string(), TypeExpr::Named("int".to_string())),
                ("labels".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("scrapeProtocol".to_string(), TypeExpr::Named("string option".to_string())),
                ("honorLabels".to_string(), TypeExpr::Named("bool option".to_string())),
                ("tlsConfig".to_string(), TypeExpr::Named("TlsConfig option".to_string())),
            ],
        }));

        // StatsD source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "StatsDSource".to_string(),
            fields: vec![
                ("address".to_string(), TypeExpr::Named("string".to_string())),
                ("port".to_string(), TypeExpr::Named("int".to_string())),
                ("protocol".to_string(), TypeExpr::Named("string option".to_string())),
                ("metricsPrefix".to_string(), TypeExpr::Named("string option".to_string())),
                ("parseMetricTags".to_string(), TypeExpr::Named("bool option".to_string())),
                ("aggregationInterval".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // System metrics source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "SystemMetrics".to_string(),
            fields: vec![
                ("interval".to_string(), TypeExpr::Named("int".to_string())),
                ("collectCpu".to_string(), TypeExpr::Named("bool option".to_string())),
                ("collectMemory".to_string(), TypeExpr::Named("bool option".to_string())),
                ("collectDisk".to_string(), TypeExpr::Named("bool option".to_string())),
                ("collectNetwork".to_string(), TypeExpr::Named("bool option".to_string())),
                ("collectProcesses".to_string(), TypeExpr::Named("bool option".to_string())),
                ("namespacePrefix".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Host metrics source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "HostMetrics".to_string(),
            fields: vec![
                ("interval".to_string(), TypeExpr::Named("int".to_string())),
                ("rootPath".to_string(), TypeExpr::Named("string option".to_string())),
                ("collectors".to_string(), TypeExpr::Named("list<string>".to_string())),
                ("filters".to_string(), TypeExpr::Named("Map<string, list<string>> option".to_string())),
            ],
        }));

        module
    }

    fn generate_logs_sources(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Logs".to_string()]);

        // File log source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "FileLog".to_string(),
            fields: vec![
                ("path".to_string(), TypeExpr::Named("string".to_string())),
                ("encoding".to_string(), TypeExpr::Named("string option".to_string())),
                ("multiline".to_string(), TypeExpr::Named("MultilineConfig option".to_string())),
                ("includeMetadata".to_string(), TypeExpr::Named("bool option".to_string())),
                ("startPosition".to_string(), TypeExpr::Named("string option".to_string())),
                ("glob".to_string(), TypeExpr::Named("bool option".to_string())),
                ("exclude".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("maxLineBytes".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // Multiline configuration
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "MultilineConfig".to_string(),
            fields: vec![
                ("pattern".to_string(), TypeExpr::Named("string".to_string())),
                ("negate".to_string(), TypeExpr::Named("bool option".to_string())),
                ("match".to_string(), TypeExpr::Named("string option".to_string())),
                ("maxLines".to_string(), TypeExpr::Named("int option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // Syslog source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Syslog".to_string(),
            fields: vec![
                ("address".to_string(), TypeExpr::Named("string".to_string())),
                ("port".to_string(), TypeExpr::Named("int".to_string())),
                ("protocol".to_string(), TypeExpr::Named("string option".to_string())),
                ("mode".to_string(), TypeExpr::Named("string option".to_string())),
                ("maxMessageSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("frameDelimiter".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Journald source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Journald".to_string(),
            fields: vec![
                ("currentBootOnly".to_string(), TypeExpr::Named("bool option".to_string())),
                ("units".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("includeKernel".to_string(), TypeExpr::Named("bool option".to_string())),
                ("batchSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("sinceNow".to_string(), TypeExpr::Named("bool option".to_string())),
                ("journalDirectory".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Docker log source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Docker".to_string(),
            fields: vec![
                ("dockerHost".to_string(), TypeExpr::Named("string option".to_string())),
                ("includeContainers".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("excludeContainers".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("includeLabels".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("excludeLabels".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("partialEventMarkerField".to_string(), TypeExpr::Named("string option".to_string())),
                ("autoPartialMerge".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // Kubernetes logs source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "KubernetesLogs".to_string(),
            fields: vec![
                ("namespaces".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("excludeNamespaces".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("labelSelector".to_string(), TypeExpr::Named("string option".to_string())),
                ("fieldSelector".to_string(), TypeExpr::Named("string option".to_string())),
                ("annotationFields".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("selfNodeName".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    fn generate_traces_sources(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Traces".to_string()]);

        // OTLP trace source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "OtlpTrace".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("protocol".to_string(), TypeExpr::Named("string".to_string())),
                ("headers".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
                ("timeout".to_string(), TypeExpr::Named("int option".to_string())),
                ("compression".to_string(), TypeExpr::Named("string option".to_string())),
                ("tlsConfig".to_string(), TypeExpr::Named("TlsConfig option".to_string())),
                ("retryConfig".to_string(), TypeExpr::Named("RetryConfig option".to_string())),
            ],
        }));

        // Jaeger trace source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Jaeger".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("protocol".to_string(), TypeExpr::Named("string option".to_string())),
                ("agentHost".to_string(), TypeExpr::Named("string option".to_string())),
                ("agentPort".to_string(), TypeExpr::Named("int option".to_string())),
                ("sampler".to_string(), TypeExpr::Named("SamplerConfig option".to_string())),
                ("tags".to_string(), TypeExpr::Named("Map<string, string> option".to_string())),
            ],
        }));

        // Zipkin trace source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Zipkin".to_string(),
            fields: vec![
                ("endpoint".to_string(), TypeExpr::Named("string".to_string())),
                ("port".to_string(), TypeExpr::Named("int".to_string())),
                ("collectorEndpoint".to_string(), TypeExpr::Named("string option".to_string())),
                ("maxPayloadSize".to_string(), TypeExpr::Named("int option".to_string())),
                ("v2Format".to_string(), TypeExpr::Named("bool option".to_string())),
            ],
        }));

        // Sampler configuration
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "SamplerConfig".to_string(),
            fields: vec![
                ("samplerType".to_string(), TypeExpr::Named("string".to_string())),
                ("param".to_string(), TypeExpr::Named("float option".to_string())),
                ("samplingServerUrl".to_string(), TypeExpr::Named("string option".to_string())),
                ("maxOperations".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        module
    }

    fn generate_events_sources(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Events".to_string()]);

        // eBPF source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "EbpfSource".to_string(),
            fields: vec![
                ("programPath".to_string(), TypeExpr::Named("string".to_string())),
                ("programType".to_string(), TypeExpr::Named("string".to_string())),
                ("attachPoint".to_string(), TypeExpr::Named("string option".to_string())),
                ("mapNames".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("pollInterval".to_string(), TypeExpr::Named("int option".to_string())),
                ("kernelVersion".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Audit log source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Audit".to_string(),
            fields: vec![
                ("socketPath".to_string(), TypeExpr::Named("string option".to_string())),
                ("auditdPath".to_string(), TypeExpr::Named("string option".to_string())),
                ("rules".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("resolveIds".to_string(), TypeExpr::Named("bool option".to_string())),
                ("bufferSize".to_string(), TypeExpr::Named("int option".to_string())),
            ],
        }));

        // CloudWatch events source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "CloudWatch".to_string(),
            fields: vec![
                ("region".to_string(), TypeExpr::Named("string".to_string())),
                ("logGroupName".to_string(), TypeExpr::Named("string option".to_string())),
                ("logStreamName".to_string(), TypeExpr::Named("string option".to_string())),
                ("filterPattern".to_string(), TypeExpr::Named("string option".to_string())),
                ("startTime".to_string(), TypeExpr::Named("int option".to_string())),
                ("pollInterval".to_string(), TypeExpr::Named("int option".to_string())),
                ("awsProfile".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // EventBridge source
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "EventBridge".to_string(),
            fields: vec![
                ("region".to_string(), TypeExpr::Named("string".to_string())),
                ("eventBusName".to_string(), TypeExpr::Named("string option".to_string())),
                ("ruleNames".to_string(), TypeExpr::Named("list<string> option".to_string())),
                ("eventPattern".to_string(), TypeExpr::Named("string option".to_string())),
                ("awsProfile".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    fn generate_common_types(&self, namespace: &str) -> GeneratedModule {
        let mut module = GeneratedModule::new(vec![namespace.to_string(), "Common".to_string()]);

        // TLS configuration
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "TlsConfig".to_string(),
            fields: vec![
                ("caFile".to_string(), TypeExpr::Named("string option".to_string())),
                ("certFile".to_string(), TypeExpr::Named("string option".to_string())),
                ("keyFile".to_string(), TypeExpr::Named("string option".to_string())),
                ("insecureSkipVerify".to_string(), TypeExpr::Named("bool option".to_string())),
                ("serverName".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Retry configuration
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "RetryConfig".to_string(),
            fields: vec![
                ("enabled".to_string(), TypeExpr::Named("bool".to_string())),
                ("initialInterval".to_string(), TypeExpr::Named("int option".to_string())),
                ("maxInterval".to_string(), TypeExpr::Named("int option".to_string())),
                ("maxElapsedTime".to_string(), TypeExpr::Named("int option".to_string())),
                ("multiplier".to_string(), TypeExpr::Named("float option".to_string())),
            ],
        }));

        // Buffer configuration
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "BufferConfig".to_string(),
            fields: vec![
                ("maxSize".to_string(), TypeExpr::Named("int".to_string())),
                ("flushInterval".to_string(), TypeExpr::Named("int option".to_string())),
                ("persistent".to_string(), TypeExpr::Named("bool option".to_string())),
                ("persistPath".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        // Authentication configuration
        module.types.push(TypeDefinition::Record(RecordDef {
            name: "AuthConfig".to_string(),
            fields: vec![
                ("authType".to_string(), TypeExpr::Named("string".to_string())),
                ("username".to_string(), TypeExpr::Named("string option".to_string())),
                ("password".to_string(), TypeExpr::Named("string option".to_string())),
                ("bearerToken".to_string(), TypeExpr::Named("string option".to_string())),
                ("apiKey".to_string(), TypeExpr::Named("string option".to_string())),
                ("apiKeyHeader".to_string(), TypeExpr::Named("string option".to_string())),
            ],
        }));

        module
    }

    fn generate_embedded_types(&self, namespace: &str) -> GeneratedTypes {
        let mut result = GeneratedTypes::new();

        // Add common types first (used by other modules)
        result.modules.push(self.generate_common_types(namespace));

        // Add source-specific types
        result.modules.push(self.generate_metrics_sources(namespace));
        result.modules.push(self.generate_logs_sources(namespace));
        result.modules.push(self.generate_traces_sources(namespace));
        result.modules.push(self.generate_events_sources(namespace));

        result
    }
}

impl Default for HibanaSourcesProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for HibanaSourcesProvider {
    fn name(&self) -> &str {
        "HibanaSourcesProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        if source == "embedded" {
            return Ok(Schema::Custom("embedded".to_string()));
        }

        Err(ProviderError::InvalidSource(format!(
            "Hibana Sources provider currently only supports 'embedded' source, got: {}",
            source
        )))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(s) if s == "embedded" => {
                Ok(self.generate_embedded_types(namespace))
            }
            _ => Err(ProviderError::ParseError("Expected Hibana Sources schema".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_name() {
        let provider = HibanaSourcesProvider::new();
        assert_eq!(provider.name(), "HibanaSourcesProvider");
    }

    #[test]
    fn test_resolve_embedded_schema() {
        let provider = HibanaSourcesProvider::new();
        let params = ProviderParams::default();
        let result = provider.resolve_schema("embedded", &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_invalid_source() {
        let provider = HibanaSourcesProvider::new();
        let params = ProviderParams::default();
        let result = provider.resolve_schema("invalid", &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_embedded_types() {
        let provider = HibanaSourcesProvider::new();
        let schema = Schema::Custom("embedded".to_string());
        let result = provider.generate_types(&schema, "HibanaSources");
        assert!(result.is_ok());

        let types = result.unwrap();
        // Should have 5 modules: Common, Metrics, Logs, Traces, Events
        assert_eq!(types.modules.len(), 5);
    }

    #[test]
    fn test_metrics_sources_module() {
        let provider = HibanaSourcesProvider::new();
        let module = provider.generate_metrics_sources("HibanaSources");

        // Should have 4 metric source types
        assert_eq!(module.types.len(), 4);

        // Check for PrometheusScrape type
        let has_prometheus = module.types.iter().any(|t| {
            if let TypeDefinition::Record(r) = t {
                r.name == "PrometheusScrape"
            } else {
                false
            }
        });
        assert!(has_prometheus);
    }

    #[test]
    fn test_logs_sources_module() {
        let provider = HibanaSourcesProvider::new();
        let module = provider.generate_logs_sources("HibanaSources");

        // Should have 6 types (including MultilineConfig)
        assert_eq!(module.types.len(), 6);

        // Check for FileLog type
        let has_file_log = module.types.iter().any(|t| {
            if let TypeDefinition::Record(r) = t {
                r.name == "FileLog"
            } else {
                false
            }
        });
        assert!(has_file_log);
    }

    #[test]
    fn test_traces_sources_module() {
        let provider = HibanaSourcesProvider::new();
        let module = provider.generate_traces_sources("HibanaSources");

        // Should have 4 types (including SamplerConfig)
        assert_eq!(module.types.len(), 4);

        // Check for OtlpTrace type
        let has_otlp = module.types.iter().any(|t| {
            if let TypeDefinition::Record(r) = t {
                r.name == "OtlpTrace"
            } else {
                false
            }
        });
        assert!(has_otlp);
    }

    #[test]
    fn test_events_sources_module() {
        let provider = HibanaSourcesProvider::new();
        let module = provider.generate_events_sources("HibanaSources");

        // Should have 4 event source types
        assert_eq!(module.types.len(), 4);

        // Check for EbpfSource type
        let has_ebpf = module.types.iter().any(|t| {
            if let TypeDefinition::Record(r) = t {
                r.name == "EbpfSource"
            } else {
                false
            }
        });
        assert!(has_ebpf);
    }

    #[test]
    fn test_common_types_module() {
        let provider = HibanaSourcesProvider::new();
        let module = provider.generate_common_types("HibanaSources");

        // Should have 4 common configuration types
        assert_eq!(module.types.len(), 4);

        // Check for TlsConfig type
        let has_tls = module.types.iter().any(|t| {
            if let TypeDefinition::Record(r) = t {
                r.name == "TlsConfig"
            } else {
                false
            }
        });
        assert!(has_tls);
    }
}
