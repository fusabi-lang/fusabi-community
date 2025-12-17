//! Regex Type Provider
//!
//! Generates Fusabi types from regex patterns with named capture groups.
//!
//! # Example
//!
//! ```rust,ignore
//! use fusabi_provider_regex::RegexProvider;
//! use fusabi_type_providers::{TypeProvider, ProviderParams};
//!
//! let provider = RegexProvider::new();
//! let pattern = r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})";
//! let schema = provider.resolve_schema(pattern, &ProviderParams::default())?;
//! let types = provider.generate_types(&schema, "Date")?;
//! // Generates: type Date = { year: string, month: string, day: string }
//! ```
//!
//! # Features
//!
//! - Named capture groups become record fields
//! - Optional groups (?) become optional fields
//! - Validates regex syntax at compile time
//! - All captured values are typed as strings

use fusabi_type_providers::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, TypeGenerator, NamingStrategy,
    RecordDef, TypeExpr, TypeDefinition,
    ProviderError, ProviderResult,
};
use regex::Regex;
use std::collections::HashMap;

/// Parsed regex pattern with capture group information
#[derive(Debug, Clone)]
pub struct RegexPattern {
    /// The original pattern string
    pub pattern: String,
    /// Named capture groups and their optional status
    pub named_groups: Vec<(String, bool)>,
}

/// Regex type provider
pub struct RegexProvider {
    generator: TypeGenerator,
}

impl RegexProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Parse a regex pattern and extract named capture groups
    fn parse_pattern(&self, pattern: &str) -> ProviderResult<RegexPattern> {
        // First validate the regex syntax
        Regex::new(pattern)
            .map_err(|e| ProviderError::ParseError(format!("Invalid regex pattern: {}", e)))?;

        // Extract named capture groups
        let named_groups = self.extract_named_groups(pattern)?;

        if named_groups.is_empty() {
            return Err(ProviderError::ParseError(
                "Regex pattern must contain at least one named capture group. \
                 Use (?P<name>...) syntax for named groups.".to_string()
            ));
        }

        Ok(RegexPattern {
            pattern: pattern.to_string(),
            named_groups,
        })
    }

    /// Extract named capture groups from pattern using regex introspection
    fn extract_named_groups(&self, pattern: &str) -> ProviderResult<Vec<(String, bool)>> {
        let re = Regex::new(pattern)
            .map_err(|e| ProviderError::ParseError(format!("Invalid regex: {}", e)))?;

        let mut groups = Vec::new();
        let mut seen_names = HashMap::new();

        // Iterate through capture group names
        for name in re.capture_names().flatten() {
            // Check for duplicate names
            if seen_names.contains_key(name) {
                return Err(ProviderError::ParseError(
                    format!("Duplicate named capture group: {}", name)
                ));
            }
            seen_names.insert(name.to_string(), ());

            // Determine if the group is optional
            let is_optional = self.is_group_optional(pattern, name);

            groups.push((name.to_string(), is_optional));
        }

        Ok(groups)
    }

    /// Determine if a named group is optional in the pattern
    /// This is a heuristic check looking for ? quantifiers after the group
    fn is_group_optional(&self, pattern: &str, group_name: &str) -> bool {
        // Look for the pattern (?P<name>...)?
        // This is a simplified heuristic - a full implementation would need
        // a proper regex AST parser

        let group_pattern = format!(r"\(\?P<{}>[^)]*\)\?", regex::escape(group_name));
        if let Ok(re) = Regex::new(&group_pattern) {
            if re.is_match(pattern) {
                return true;
            }
        }

        // Also check for the group being inside an optional non-capturing group
        // Pattern: (?:...(?P<name>...)...)?
        // This is more complex and would require proper parsing
        // For now, we'll do a simple check

        false
    }

    /// Generate Fusabi types from parsed regex pattern
    fn generate_from_pattern(
        &self,
        pattern: &RegexPattern,
        type_name: &str,
    ) -> ProviderResult<GeneratedTypes> {
        let mut result = GeneratedTypes::new();

        // Create fields from named groups
        let fields: Vec<(String, TypeExpr)> = pattern.named_groups.iter()
            .map(|(name, is_optional)| {
                // Keep field names as-is from the regex pattern
                let type_expr = if *is_optional {
                    TypeExpr::Named("string option".to_string())
                } else {
                    TypeExpr::Named("string".to_string())
                };
                (name.clone(), type_expr)
            })
            .collect();

        // Create the record type definition
        let record = RecordDef {
            name: self.generator.naming.apply(type_name),
            fields,
        };

        result.root_types.push(TypeDefinition::Record(record));
        Ok(result)
    }
}

impl Default for RegexProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for RegexProvider {
    fn name(&self) -> &str {
        "RegexProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        // Parse the regex pattern to validate it early
        let _parsed = self.parse_pattern(source)?;

        // Store as a custom schema with the pattern string
        Ok(Schema::Custom(source.to_string()))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        match schema {
            Schema::Custom(pattern) => {
                let parsed = self.parse_pattern(pattern)?;
                self.generate_from_pattern(&parsed, namespace)
            }
            _ => Err(ProviderError::ParseError("Expected regex pattern".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_date_pattern() {
        let provider = RegexProvider::new();
        let pattern = r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})";

        let parsed = provider.parse_pattern(pattern).unwrap();
        assert_eq!(parsed.named_groups.len(), 3);
        assert_eq!(parsed.named_groups[0].0, "year");
        assert_eq!(parsed.named_groups[1].0, "month");
        assert_eq!(parsed.named_groups[2].0, "day");
    }

    #[test]
    fn test_generate_date_type() {
        let provider = RegexProvider::new();
        let pattern = r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})";

        let schema = provider.resolve_schema(pattern, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Date").unwrap();

        assert_eq!(types.root_types.len(), 1);
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "Date");
            assert_eq!(record.fields.len(), 3);
            assert_eq!(record.fields[0].0, "year");
            assert_eq!(record.fields[1].0, "month");
            assert_eq!(record.fields[2].0, "day");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_optional_group() {
        let provider = RegexProvider::new();
        // Pattern with optional time component
        let pattern = r"(?P<date>\d{4}-\d{2}-\d{2})(?P<time>T\d{2}:\d{2}:\d{2})?";

        let parsed = provider.parse_pattern(pattern).unwrap();
        assert_eq!(parsed.named_groups.len(), 2);
        assert_eq!(parsed.named_groups[0].0, "date");
        assert_eq!(parsed.named_groups[1].0, "time");
        assert!(!parsed.named_groups[0].1); // date is not optional
        assert!(parsed.named_groups[1].1);  // time is optional
    }

    #[test]
    fn test_email_pattern() {
        let provider = RegexProvider::new();
        let pattern = r"(?P<username>[a-zA-Z0-9._%+-]+)@(?P<domain>[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})";

        let schema = provider.resolve_schema(pattern, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "Email").unwrap();

        assert_eq!(types.root_types.len(), 1);
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "Email");
            assert_eq!(record.fields.len(), 2);
            assert_eq!(record.fields[0].0, "username");
            assert_eq!(record.fields[1].0, "domain");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_url_pattern() {
        let provider = RegexProvider::new();
        let pattern = r"(?P<protocol>https?)://(?P<host>[^/]+)(?P<path>/.*)?";

        let parsed = provider.parse_pattern(pattern).unwrap();
        assert_eq!(parsed.named_groups.len(), 3);
        assert_eq!(parsed.named_groups[0].0, "protocol");
        assert_eq!(parsed.named_groups[1].0, "host");
        assert_eq!(parsed.named_groups[2].0, "path");
        assert!(parsed.named_groups[2].1); // path is optional
    }

    #[test]
    fn test_no_named_groups_error() {
        let provider = RegexProvider::new();
        let pattern = r"\d{4}-\d{2}-\d{2}"; // No named groups

        let result = provider.parse_pattern(pattern);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("named capture group"));
    }

    #[test]
    fn test_invalid_regex_error() {
        let provider = RegexProvider::new();
        let pattern = r"(?P<invalid>[[["; // Invalid regex

        let result = provider.parse_pattern(pattern);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_group_names_error() {
        let provider = RegexProvider::new();
        // Regex crate doesn't allow duplicate group names, this will fail at regex compilation
        let pattern = r"(?P<name>\w+)|(?P<name>\d+)";

        let result = provider.parse_pattern(pattern);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_line_pattern() {
        let provider = RegexProvider::new();
        let pattern = r"\[(?P<timestamp>[^\]]+)\] (?P<level>INFO|WARN|ERROR) (?P<message>.+)";

        let schema = provider.resolve_schema(pattern, &ProviderParams::default()).unwrap();
        let types = provider.generate_types(&schema, "LogEntry").unwrap();

        assert_eq!(types.root_types.len(), 1);
        if let TypeDefinition::Record(record) = &types.root_types[0] {
            assert_eq!(record.name, "LogEntry");
            assert_eq!(record.fields.len(), 3);
            assert_eq!(record.fields[0].0, "timestamp");
            assert_eq!(record.fields[1].0, "level");
            assert_eq!(record.fields[2].0, "message");
        } else {
            panic!("Expected Record type definition");
        }
    }

    #[test]
    fn test_semantic_version_pattern() {
        let provider = RegexProvider::new();
        let pattern = r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?P<prerelease>-[a-zA-Z0-9.]+)?";

        let parsed = provider.parse_pattern(pattern).unwrap();
        assert_eq!(parsed.named_groups.len(), 4);
        assert!(!parsed.named_groups[0].1); // major is required
        assert!(!parsed.named_groups[1].1); // minor is required
        assert!(!parsed.named_groups[2].1); // patch is required
        assert!(parsed.named_groups[3].1);  // prerelease is optional
    }
}
