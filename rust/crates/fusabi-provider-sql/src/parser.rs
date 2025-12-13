//! SQL DDL parser

use crate::types::{Column, Constraint, SqlSchema, SqlType, Table, TableConstraint};
use fusabi_type_providers::{ProviderError, ProviderResult};

/// Parse SQL DDL statements into a SqlSchema
pub fn parse_sql_ddl(sql: &str) -> ProviderResult<SqlSchema> {
    let mut schema = SqlSchema::new();

    // Split into individual statements
    let statements = split_statements(sql);

    for stmt in statements {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }

        // Parse CREATE TABLE statements
        if stmt.to_uppercase().starts_with("CREATE TABLE") {
            let table = parse_create_table(stmt)?;
            schema.add_table(table);
        }
        // Ignore other statements for now (CREATE INDEX, ALTER TABLE, etc.)
    }

    Ok(schema)
}

/// Split SQL into individual statements
fn split_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut string_char = ' ';
    let mut paren_depth = 0;

    for ch in sql.chars() {
        match ch {
            '\'' | '"' => {
                if in_string && ch == string_char {
                    in_string = false;
                } else if !in_string {
                    in_string = true;
                    string_char = ch;
                }
                current.push(ch);
            }
            '(' if !in_string => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' if !in_string => {
                paren_depth -= 1;
                current.push(ch);
            }
            ';' if !in_string && paren_depth == 0 => {
                if !current.trim().is_empty() {
                    statements.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    // Add last statement if it doesn't end with semicolon
    if !current.trim().is_empty() {
        statements.push(current.trim().to_string());
    }

    statements
}

/// Parse a CREATE TABLE statement
fn parse_create_table(stmt: &str) -> ProviderResult<Table> {
    // Remove CREATE TABLE prefix
    let stmt_upper = stmt.to_uppercase();
    let start_idx = if let Some(idx) = stmt_upper.find("CREATE TABLE") {
        idx + "CREATE TABLE".len()
    } else {
        return Err(ProviderError::ParseError("Invalid CREATE TABLE statement".to_string()));
    };

    let rest = stmt[start_idx..].trim();

    // Handle IF NOT EXISTS
    let rest = if rest.to_uppercase().starts_with("IF NOT EXISTS") {
        rest["IF NOT EXISTS".len()..].trim()
    } else {
        rest
    };

    // Extract table name
    let (table_name, rest) = extract_table_name(rest)?;

    // Find column definitions (between parentheses)
    let (columns_str, _rest) = extract_parentheses_content(rest)?;

    // Parse column and table constraint definitions
    let (columns, table_constraints) = parse_table_definitions(&columns_str)?;

    let mut table = Table::new(table_name);
    table.columns = columns;
    table.table_constraints = table_constraints;

    Ok(table)
}

/// Extract table name from statement
fn extract_table_name(s: &str) -> ProviderResult<(String, &str)> {
    let s = s.trim();

    // Handle quoted table names
    if s.starts_with('"') || s.starts_with('`') {
        let quote_char = s.chars().next().unwrap();
        let end_idx = s[1..].find(quote_char)
            .ok_or_else(|| ProviderError::ParseError("Unclosed quoted table name".to_string()))?;
        let table_name = s[1..end_idx+1].to_string();
        let rest = &s[end_idx+2..];
        return Ok((table_name, rest));
    }

    // Handle unquoted table names
    let end_idx = s.find(|c: char| c.is_whitespace() || c == '(')
        .unwrap_or(s.len());
    let table_name = s[..end_idx].to_string();
    let rest = &s[end_idx..];

    Ok((table_name, rest))
}

/// Extract content between parentheses
fn extract_parentheses_content(s: &str) -> ProviderResult<(String, &str)> {
    let s = s.trim();
    if !s.starts_with('(') {
        return Err(ProviderError::ParseError("Expected opening parenthesis".to_string()));
    }

    let mut depth = 0;
    let mut end_idx = 0;

    for (i, ch) in s.chars().enumerate() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end_idx = i;
                    break;
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(ProviderError::ParseError("Unmatched parentheses".to_string()));
    }

    let content = s[1..end_idx].to_string();
    let rest = &s[end_idx+1..];

    Ok((content, rest))
}

/// Parse table definitions (columns and constraints)
fn parse_table_definitions(s: &str) -> ProviderResult<(Vec<Column>, Vec<TableConstraint>)> {
    let mut columns = Vec::new();
    let mut table_constraints = Vec::new();

    // Split by commas (handling nested parentheses)
    let definitions = split_by_comma(s);

    for def in definitions {
        let def = def.trim();
        if def.is_empty() {
            continue;
        }

        let def_upper = def.to_uppercase();

        // Check if it's a table constraint
        if def_upper.starts_with("PRIMARY KEY") {
            let cols = extract_constraint_columns(&def["PRIMARY KEY".len()..])?;
            table_constraints.push(TableConstraint::PrimaryKey(cols));
        } else if def_upper.starts_with("UNIQUE") {
            let rest = &def["UNIQUE".len()..];
            let cols = extract_constraint_columns(rest)?;
            table_constraints.push(TableConstraint::Unique(cols));
        } else if def_upper.starts_with("FOREIGN KEY") {
            // Skip for now - complex to parse
            continue;
        } else if def_upper.starts_with("CONSTRAINT") {
            // Skip named constraints for now
            continue;
        } else if def_upper.starts_with("CHECK") {
            let check_expr = def["CHECK".len()..].trim().to_string();
            table_constraints.push(TableConstraint::Check(check_expr));
        } else {
            // It's a column definition
            let column = parse_column_definition(def)?;
            columns.push(column);
        }
    }

    Ok((columns, table_constraints))
}

/// Extract column names from constraint definition
fn extract_constraint_columns(s: &str) -> ProviderResult<Vec<String>> {
    let s = s.trim();
    if !s.starts_with('(') {
        return Err(ProviderError::ParseError("Expected column list in parentheses".to_string()));
    }

    let (content, _) = extract_parentheses_content(s)?;
    let columns = split_by_comma(&content)
        .into_iter()
        .map(|c| c.trim().trim_matches('"').trim_matches('`').to_string())
        .collect();

    Ok(columns)
}

/// Split string by commas, respecting nested parentheses
fn split_by_comma(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = ' ';

    for ch in s.chars() {
        match ch {
            '\'' | '"' => {
                if in_string && ch == string_char {
                    in_string = false;
                } else if !in_string {
                    in_string = true;
                    string_char = ch;
                }
                current.push(ch);
            }
            '(' if !in_string => {
                depth += 1;
                current.push(ch);
            }
            ')' if !in_string => {
                depth -= 1;
                current.push(ch);
            }
            ',' if !in_string && depth == 0 => {
                parts.push(current.clone());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        parts.push(current);
    }

    parts
}

/// Parse a column definition
fn parse_column_definition(def: &str) -> ProviderResult<Column> {
    let def = def.trim();
    let parts: Vec<&str> = def.split_whitespace().collect();

    if parts.is_empty() {
        return Err(ProviderError::ParseError("Empty column definition".to_string()));
    }

    // First part is column name
    let column_name = parts[0].trim_matches('"').trim_matches('`').to_string();

    // Second part is data type
    if parts.len() < 2 {
        return Err(ProviderError::ParseError(format!("Missing type for column {}", column_name)));
    }

    // Handle types with parameters (e.g., VARCHAR(255))
    let type_str = if parts[1].contains('(') {
        // Find the closing paren
        let mut type_parts = vec![parts[1]];
        let mut i = 2;
        while i < parts.len() && !type_parts.last().unwrap().contains(')') {
            type_parts.push(parts[i]);
            i += 1;
        }
        type_parts.join(" ")
    } else {
        parts[1].to_string()
    };

    let sql_type = SqlType::from_str(&type_str);
    let mut column = Column::new(column_name, sql_type);

    // Parse constraints
    let remaining = &parts[2..].join(" ").to_uppercase();
    parse_column_constraints(remaining, &mut column);

    Ok(column)
}

/// Parse column constraints
fn parse_column_constraints(s: &str, column: &mut Column) {
    let s_upper = s.to_uppercase();

    if s_upper.contains("PRIMARY KEY") || s_upper.contains("PRIMARYKEY") {
        column.constraints.push(Constraint::PrimaryKey);
    }

    if s_upper.contains("NOT NULL") {
        column.constraints.push(Constraint::NotNull);
    } else if s_upper.contains("NULL") && !s_upper.contains("NOT NULL") {
        column.constraints.push(Constraint::Null);
    }

    if s_upper.contains("UNIQUE") {
        column.constraints.push(Constraint::Unique);
    }

    if s_upper.contains("AUTO_INCREMENT") || s_upper.contains("AUTOINCREMENT") {
        column.constraints.push(Constraint::AutoIncrement);
    }

    // Parse DEFAULT
    if let Some(idx) = s_upper.find("DEFAULT") {
        let default_part = &s[idx + "DEFAULT".len()..].trim();
        // Extract default value (simplified - doesn't handle complex expressions)
        let default_value = default_part
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches('\'')
            .trim_matches('"')
            .to_string();
        if !default_value.is_empty() {
            column.constraints.push(Constraint::Default(default_value));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_table() {
        let sql = r#"
            CREATE TABLE users (
                id INT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email TEXT
            );
        "#;

        let schema = parse_sql_ddl(sql).unwrap();
        assert_eq!(schema.tables.len(), 1);

        let table = schema.tables.get("users").unwrap();
        assert_eq!(table.name, "users");
        assert_eq!(table.columns.len(), 3);

        let id_col = &table.columns[0];
        assert_eq!(id_col.name, "id");
        assert!(id_col.is_primary_key());

        let name_col = &table.columns[1];
        assert_eq!(name_col.name, "name");
        assert!(!name_col.is_nullable());
    }

    #[test]
    fn test_parse_multiple_tables() {
        let sql = r#"
            CREATE TABLE users (
                id INT PRIMARY KEY
            );

            CREATE TABLE posts (
                id INT PRIMARY KEY,
                user_id INT NOT NULL
            );
        "#;

        let schema = parse_sql_ddl(sql).unwrap();
        assert_eq!(schema.tables.len(), 2);
        assert!(schema.tables.contains_key("users"));
        assert!(schema.tables.contains_key("posts"));
    }

    #[test]
    fn test_parse_table_with_default() {
        let sql = r#"
            CREATE TABLE settings (
                theme VARCHAR(50) DEFAULT 'light',
                notifications BOOLEAN DEFAULT true
            );
        "#;

        let schema = parse_sql_ddl(sql).unwrap();
        let table = schema.tables.get("settings").unwrap();

        assert!(table.columns[0].has_default());
        assert!(table.columns[1].has_default());
    }

    #[test]
    fn test_split_statements() {
        let sql = "CREATE TABLE a (id INT); CREATE TABLE b (id INT);";
        let stmts = split_statements(sql);
        assert_eq!(stmts.len(), 2);
    }

    #[test]
    fn test_split_by_comma() {
        let s = "id INT, name VARCHAR(255), data JSON";
        let parts = split_by_comma(s);
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].trim(), "id INT");
        assert_eq!(parts[1].trim(), "name VARCHAR(255)");
        assert_eq!(parts[2].trim(), "data JSON");
    }
}
