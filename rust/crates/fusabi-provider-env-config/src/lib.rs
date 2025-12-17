//! Environment Configuration Type Provider
//!
//! Generates Fusabi types from .env file definitions.

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};

/// Environment configuration type provider
pub struct EnvConfigProvider {
    generator: TypeGenerator,
}

impl EnvConfigProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    fn parse_env_file(&self, content: &str) -> Vec<(String, String)> {
        content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn infer_type(&self, value: &str) -> TypeExpr {
        // Try to infer type from value
        if value.parse::<i64>().is_ok() {
            TypeExpr::Named("int".to_string())
        } else if value.parse::<f64>().is_ok() {
            TypeExpr::Named("float".to_string())
        } else if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
            TypeExpr::Named("bool".to_string())
        } else {
            TypeExpr::Named("string".to_string())
        }
    }
}

impl Default for EnvConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for EnvConfigProvider {
    fn name(&self) -> &str {
        "EnvConfigProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        let content = if source.starts_with("file://") {
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        } else if source.contains('=') {
            // Inline env content
            source.to_string()
        } else {
            // Treat as file path
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(e.to_string()))?
        };

        Ok(Schema::Custom(content))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        let content = match schema {
            Schema::Custom(s) => s,
            _ => return Err(ProviderError::ParseError("Expected env config".to_string())),
        };

        let vars = self.parse_env_file(content);
        let fields: Vec<(String, TypeExpr)> = vars
            .into_iter()
            .map(|(name, value)| {
                let type_expr = self.infer_type(&value);
                (self.generator.naming.apply(&name.to_lowercase()), type_expr)
            })
            .collect();

        let mut result = GeneratedTypes::new();
        let mut module = GeneratedModule::new(vec![namespace.to_string()]);

        module.types.push(TypeDefinition::Record(RecordDef {
            name: "Config".to_string(),
            fields,
        }));

        result.modules.push(module);
        Ok(result)
    }
}
