//! Protobuf file parser
//!
//! This is a simplified protobuf parser that handles the most common syntax.
//! It supports proto2 and proto3 syntax for messages, enums, and services.

use crate::types::{
    ProtoFile, Message, Field, FieldType, FieldLabel, Enum, EnumValue, Service, Method,
};
use fusabi_type_providers::{ProviderError, ProviderResult};

/// Parse a .proto file from string content
pub fn parse_proto(content: &str) -> ProviderResult<ProtoFile> {
    let mut parser = Parser::new(content);
    parser.parse_file()
}

/// Simple protobuf parser
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Keywords
    Package,
    Import,
    Message,
    Enum,
    Service,
    Rpc,
    Returns,
    Optional,
    Required,
    Repeated,
    Map,
    Stream,

    // Symbols
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftAngle,
    RightAngle,
    Semicolon,
    Equals,
    Comma,
    Dot,

    // Literals
    Identifier(String),
    Number(String),
    StringLiteral(String),

    // End of file
    Eof,
}

impl Parser {
    fn new(content: &str) -> Self {
        let tokens = tokenize(content);
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> ProviderResult<()> {
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(ProviderError::ParseError(format!(
                "Expected {:?}, got {:?}",
                expected,
                self.current()
            )))
        }
    }

    fn expect_identifier(&mut self) -> ProviderResult<String> {
        match self.current() {
            Token::Identifier(s) => {
                let result = s.clone();
                self.advance();
                Ok(result)
            }
            _ => Err(ProviderError::ParseError(format!(
                "Expected identifier, got {:?}",
                self.current()
            ))),
        }
    }

    fn expect_number(&mut self) -> ProviderResult<String> {
        match self.current() {
            Token::Number(s) => {
                let result = s.clone();
                self.advance();
                Ok(result)
            }
            _ => Err(ProviderError::ParseError(format!(
                "Expected number, got {:?}",
                self.current()
            ))),
        }
    }

    fn parse_file(&mut self) -> ProviderResult<ProtoFile> {
        let mut file = ProtoFile::new();

        // Skip syntax declaration if present
        while self.current() != &Token::Eof {
            if let Token::Identifier(s) = self.current() {
                if s == "syntax" {
                    self.advance();
                    self.expect(Token::Equals)?;
                    if let Token::StringLiteral(_) = self.current() {
                        self.advance();
                    }
                    self.expect(Token::Semicolon)?;
                    continue;
                }
            }

            match self.current() {
                Token::Package => {
                    self.advance();
                    file.package = Some(self.parse_qualified_name()?);
                    self.expect(Token::Semicolon)?;
                }
                Token::Import => {
                    self.advance();
                    if let Token::StringLiteral(s) = self.current() {
                        file.imports.push(s.clone());
                        self.advance();
                    }
                    self.expect(Token::Semicolon)?;
                }
                Token::Message => {
                    file.messages.push(self.parse_message()?);
                }
                Token::Enum => {
                    file.enums.push(self.parse_enum()?);
                }
                Token::Service => {
                    file.services.push(self.parse_service()?);
                }
                Token::Eof => break,
                _ => {
                    // Skip unknown tokens
                    self.advance();
                }
            }
        }

        Ok(file)
    }

    fn parse_message(&mut self) -> ProviderResult<Message> {
        self.expect(Token::Message)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut message = Message::new(name);

        while self.current() != &Token::RightBrace && self.current() != &Token::Eof {
            match self.current() {
                Token::Message => {
                    message.nested_messages.push(self.parse_message()?);
                }
                Token::Enum => {
                    message.nested_enums.push(self.parse_enum()?);
                }
                Token::Optional | Token::Required | Token::Repeated => {
                    message.fields.push(self.parse_field()?);
                }
                Token::Map => {
                    message.fields.push(self.parse_map_field()?);
                }
                Token::Identifier(_) => {
                    // Proto3 field (no label)
                    message.fields.push(self.parse_field()?);
                }
                _ => {
                    // Skip unknown tokens
                    self.advance();
                }
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(message)
    }

    fn parse_field(&mut self) -> ProviderResult<Field> {
        // Parse optional label
        let label = match self.current() {
            Token::Optional => {
                self.advance();
                FieldLabel::Optional
            }
            Token::Required => {
                self.advance();
                FieldLabel::Required
            }
            Token::Repeated => {
                self.advance();
                FieldLabel::Repeated
            }
            _ => FieldLabel::Optional, // Proto3 default
        };

        // Parse field type
        let type_name = self.expect_identifier()?;
        let field_type = FieldType::from_str(&type_name);

        // Parse field name
        let name = self.expect_identifier()?;

        // Parse field number
        self.expect(Token::Equals)?;
        let number_str = self.expect_number()?;
        let number: u32 = number_str.parse().map_err(|_| {
            ProviderError::ParseError(format!("Invalid field number: {}", number_str))
        })?;

        self.expect(Token::Semicolon)?;

        Ok(Field {
            name,
            field_type,
            number,
            label,
        })
    }

    fn parse_map_field(&mut self) -> ProviderResult<Field> {
        self.expect(Token::Map)?;
        self.expect(Token::LeftAngle)?;

        // Parse key type
        let key_type_name = self.expect_identifier()?;
        let key_type = FieldType::from_str(&key_type_name);

        self.expect(Token::Comma)?;

        // Parse value type
        let value_type_name = self.expect_identifier()?;
        let value_type = FieldType::from_str(&value_type_name);

        self.expect(Token::RightAngle)?;

        // Parse field name
        let name = self.expect_identifier()?;

        // Parse field number
        self.expect(Token::Equals)?;
        let number_str = self.expect_number()?;
        let number: u32 = number_str.parse().map_err(|_| {
            ProviderError::ParseError(format!("Invalid field number: {}", number_str))
        })?;

        self.expect(Token::Semicolon)?;

        Ok(Field {
            name,
            field_type: FieldType::Map(Box::new(key_type), Box::new(value_type)),
            number,
            label: FieldLabel::Repeated, // Maps are always repeated
        })
    }

    fn parse_enum(&mut self) -> ProviderResult<Enum> {
        self.expect(Token::Enum)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut enum_def = Enum::new(name);

        while self.current() != &Token::RightBrace && self.current() != &Token::Eof {
            if let Token::Identifier(value_name) = self.current() {
                let value_name = value_name.clone();
                self.advance();
                self.expect(Token::Equals)?;
                let number_str = self.expect_number()?;
                let number: i32 = number_str.parse().map_err(|_| {
                    ProviderError::ParseError(format!("Invalid enum number: {}", number_str))
                })?;
                self.expect(Token::Semicolon)?;

                enum_def.values.push(EnumValue { name: value_name, number });
            } else {
                self.advance();
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(enum_def)
    }

    fn parse_service(&mut self) -> ProviderResult<Service> {
        self.expect(Token::Service)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut service = Service {
            name,
            methods: Vec::new(),
        };

        while self.current() != &Token::RightBrace && self.current() != &Token::Eof {
            if self.current() == &Token::Rpc {
                service.methods.push(self.parse_method()?);
            } else {
                self.advance();
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(service)
    }

    fn parse_method(&mut self) -> ProviderResult<Method> {
        self.expect(Token::Rpc)?;
        let name = self.expect_identifier()?;

        self.expect(Token::LeftParen)?;
        let client_streaming = if self.current() == &Token::Stream {
            self.advance();
            true
        } else {
            false
        };
        let input_type = self.expect_identifier()?;
        self.expect(Token::RightParen)?;

        self.expect(Token::Returns)?;
        self.expect(Token::LeftParen)?;
        let server_streaming = if self.current() == &Token::Stream {
            self.advance();
            true
        } else {
            false
        };
        let output_type = self.expect_identifier()?;
        self.expect(Token::RightParen)?;

        // Skip method body if present
        if self.current() == &Token::LeftBrace {
            self.advance();
            let mut depth = 1;
            while depth > 0 && self.current() != &Token::Eof {
                match self.current() {
                    Token::LeftBrace => depth += 1,
                    Token::RightBrace => depth -= 1,
                    _ => {}
                }
                self.advance();
            }
        } else {
            self.expect(Token::Semicolon)?;
        }

        Ok(Method {
            name,
            input_type,
            output_type,
            client_streaming,
            server_streaming,
        })
    }

    fn parse_qualified_name(&mut self) -> ProviderResult<String> {
        let mut parts = vec![self.expect_identifier()?];
        while self.current() == &Token::Dot {
            self.advance();
            parts.push(self.expect_identifier()?);
        }
        Ok(parts.join("."))
    }
}

/// Tokenize a protobuf file
fn tokenize(content: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = content.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    // Line comment
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\n' {
                            break;
                        }
                    }
                } else if chars.peek() == Some(&'*') {
                    // Block comment
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                }
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '<' => {
                tokens.push(Token::LeftAngle);
                chars.next();
            }
            '>' => {
                tokens.push(Token::RightAngle);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
            }
            '"' => {
                chars.next();
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == '"' {
                        break;
                    }
                    if c == '\\' {
                        if let Some(&next) = chars.peek() {
                            chars.next();
                            string.push(next);
                        }
                    } else {
                        string.push(c);
                    }
                }
                tokens.push(Token::StringLiteral(string));
            }
            '0'..='9' | '-' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '-' || c == '.' {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check for keywords
                let token = match ident.as_str() {
                    "package" => Token::Package,
                    "import" => Token::Import,
                    "message" => Token::Message,
                    "enum" => Token::Enum,
                    "service" => Token::Service,
                    "rpc" => Token::Rpc,
                    "returns" => Token::Returns,
                    "optional" => Token::Optional,
                    "required" => Token::Required,
                    "repeated" => Token::Repeated,
                    "map" => Token::Map,
                    "stream" => Token::Stream,
                    _ => Token::Identifier(ident),
                };
                tokens.push(token);
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens.push(Token::Eof);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_message() {
        let proto = r#"
            syntax = "proto3";
            package example;

            message Person {
                string name = 1;
                int32 age = 2;
            }
        "#;

        let file = parse_proto(proto).unwrap();
        assert_eq!(file.package, Some("example".to_string()));
        assert_eq!(file.messages.len(), 1);
        assert_eq!(file.messages[0].name, "Person");
        assert_eq!(file.messages[0].fields.len(), 2);
    }

    #[test]
    fn test_parse_enum() {
        let proto = r#"
            enum Status {
                UNKNOWN = 0;
                ACTIVE = 1;
                INACTIVE = 2;
            }
        "#;

        let file = parse_proto(proto).unwrap();
        assert_eq!(file.enums.len(), 1);
        assert_eq!(file.enums[0].name, "Status");
        assert_eq!(file.enums[0].values.len(), 3);
    }

    #[test]
    fn test_parse_nested_message() {
        let proto = r#"
            message Outer {
                message Inner {
                    string value = 1;
                }
                Inner inner = 1;
            }
        "#;

        let file = parse_proto(proto).unwrap();
        assert_eq!(file.messages.len(), 1);
        assert_eq!(file.messages[0].nested_messages.len(), 1);
    }
}
