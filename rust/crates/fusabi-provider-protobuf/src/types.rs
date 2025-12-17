//! Protobuf AST type definitions

use std::collections::HashMap;

/// Represents a complete .proto file
#[derive(Debug, Clone, Default)]
pub struct ProtoFile {
    /// Package name (e.g., "example.v1")
    pub package: Option<String>,
    /// Import statements
    pub imports: Vec<String>,
    /// Top-level message definitions
    pub messages: Vec<Message>,
    /// Top-level enum definitions
    pub enums: Vec<Enum>,
    /// Service definitions
    pub services: Vec<Service>,
}

/// Protobuf message definition
#[derive(Debug, Clone)]
pub struct Message {
    /// Message name
    pub name: String,
    /// Message fields
    pub fields: Vec<Field>,
    /// Nested messages
    pub nested_messages: Vec<Message>,
    /// Nested enums
    pub nested_enums: Vec<Enum>,
}

/// Protobuf field definition
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: FieldType,
    /// Field number (protobuf tag)
    pub number: u32,
    /// Field label (optional, required, repeated)
    pub label: FieldLabel,
}

/// Field label indicating cardinality
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldLabel {
    /// Optional field (proto3 default, or explicit "optional" in proto2)
    Optional,
    /// Required field (proto2 only)
    Required,
    /// Repeated field (list)
    Repeated,
}

/// Protobuf field type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    // Scalar types
    Double,
    Float,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Bool,
    String,
    Bytes,

    // Message type (reference to another message)
    Message(String),

    // Enum type (reference to an enum)
    Enum(String),

    // Map type (key_type, value_type)
    Map(Box<FieldType>, Box<FieldType>),
}

/// Protobuf enum definition
#[derive(Debug, Clone)]
pub struct Enum {
    /// Enum name
    pub name: String,
    /// Enum values
    pub values: Vec<EnumValue>,
}

/// Protobuf enum value
#[derive(Debug, Clone)]
pub struct EnumValue {
    /// Value name
    pub name: String,
    /// Value number
    pub number: i32,
}

/// Protobuf service definition
#[derive(Debug, Clone)]
pub struct Service {
    /// Service name
    pub name: String,
    /// RPC methods
    pub methods: Vec<Method>,
}

/// Protobuf RPC method
#[derive(Debug, Clone)]
pub struct Method {
    /// Method name
    pub name: String,
    /// Input message type
    pub input_type: String,
    /// Output message type
    pub output_type: String,
    /// Whether input is a stream
    pub client_streaming: bool,
    /// Whether output is a stream
    pub server_streaming: bool,
}

impl ProtoFile {
    /// Create a new empty ProtoFile
    pub fn new() -> Self {
        Self::default()
    }

    /// Get all message definitions including nested ones
    pub fn all_messages(&self) -> Vec<&Message> {
        let mut result = Vec::new();
        for msg in &self.messages {
            result.push(msg);
            result.extend(msg.all_nested_messages());
        }
        result
    }

    /// Get all enum definitions including nested ones
    pub fn all_enums(&self) -> Vec<&Enum> {
        let mut result = Vec::new();
        for enum_def in &self.enums {
            result.push(enum_def);
        }
        for msg in &self.messages {
            result.extend(msg.all_nested_enums());
        }
        result
    }

    /// Build a map of type name to message for resolution
    pub fn build_message_map(&self) -> HashMap<String, &Message> {
        let mut map = HashMap::new();
        for msg in self.all_messages() {
            map.insert(msg.name.clone(), msg);
        }
        map
    }

    /// Build a map of type name to enum for resolution
    pub fn build_enum_map(&self) -> HashMap<String, &Enum> {
        let mut map = HashMap::new();
        for enum_def in self.all_enums() {
            map.insert(enum_def.name.clone(), enum_def);
        }
        map
    }
}

impl Message {
    /// Create a new message
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
        }
    }

    /// Get all nested messages recursively
    pub fn all_nested_messages(&self) -> Vec<&Message> {
        let mut result = Vec::new();
        for nested in &self.nested_messages {
            result.push(nested);
            result.extend(nested.all_nested_messages());
        }
        result
    }

    /// Get all nested enums recursively
    pub fn all_nested_enums(&self) -> Vec<&Enum> {
        let mut result = Vec::new();
        for nested in &self.nested_enums {
            result.push(nested);
        }
        for msg in &self.nested_messages {
            result.extend(msg.all_nested_enums());
        }
        result
    }
}

impl Enum {
    /// Create a new enum
    pub fn new(name: String) -> Self {
        Self {
            name,
            values: Vec::new(),
        }
    }
}

impl FieldType {
    /// Parse a field type from a string
    pub fn from_str(s: &str) -> Self {
        match s {
            "double" => FieldType::Double,
            "float" => FieldType::Float,
            "int32" => FieldType::Int32,
            "int64" => FieldType::Int64,
            "uint32" => FieldType::UInt32,
            "uint64" => FieldType::UInt64,
            "sint32" => FieldType::SInt32,
            "sint64" => FieldType::SInt64,
            "fixed32" => FieldType::Fixed32,
            "fixed64" => FieldType::Fixed64,
            "sfixed32" => FieldType::SFixed32,
            "sfixed64" => FieldType::SFixed64,
            "bool" => FieldType::Bool,
            "string" => FieldType::String,
            "bytes" => FieldType::Bytes,
            _ => {
                // Assume it's a message or enum type
                // Parser will need to resolve this later
                FieldType::Message(s.to_string())
            }
        }
    }

    /// Check if this is a scalar type
    pub fn is_scalar(&self) -> bool {
        !matches!(self, FieldType::Message(_) | FieldType::Enum(_) | FieldType::Map(_, _))
    }
}
