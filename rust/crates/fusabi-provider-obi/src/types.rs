//! OBI (Observability Binary Interface) type definitions
//!
//! This module defines the type system for eBPF event structures used in
//! the Hibana observability agent.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// OBI primitive types that map to eBPF/kernel types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObiPrimitiveType {
    /// Unsigned 8-bit integer (u8, __u8)
    U8,
    /// Unsigned 16-bit integer (u16, __u16)
    U16,
    /// Unsigned 32-bit integer (u32, __u32)
    U32,
    /// Unsigned 64-bit integer (u64, __u64)
    U64,
    /// Signed 8-bit integer (i8, __s8)
    I8,
    /// Signed 16-bit integer (i16, __s16)
    I16,
    /// Signed 32-bit integer (i32, __s32)
    I32,
    /// Signed 64-bit integer (i64, __s64)
    I64,
    /// Boolean (bool, _Bool)
    Bool,
    /// String (char array or pointer)
    String,
    /// IPv4 address (u32 in network byte order)
    Ipv4Addr,
    /// IPv6 address (16 bytes)
    Ipv6Addr,
    /// PID/TID type
    Pid,
    /// Timestamp (nanoseconds since boot)
    Timestamp,
}

/// OBI type representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ObiType {
    /// Primitive type
    Primitive {
        #[serde(rename = "type")]
        prim_type: ObiPrimitiveType,
    },
    /// Fixed-size array
    Array {
        #[serde(rename = "type")]
        element_type: Box<ObiType>,
        size: usize,
    },
    /// Dynamic array (list)
    List {
        #[serde(rename = "type")]
        element_type: Box<ObiType>,
    },
    /// Struct reference
    Struct {
        name: String,
    },
    /// Enum/Discriminated union
    Enum {
        name: String,
    },
    /// Optional/nullable type
    Option {
        #[serde(rename = "type")]
        inner_type: Box<ObiType>,
    },
}

/// Field definition in an OBI struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObiField {
    /// Field name
    pub name: String,
    /// Field type
    #[serde(rename = "type")]
    pub field_type: ObiType,
    /// Field description/documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Offset in bytes (for layout verification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

/// Struct definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObiStruct {
    /// Struct name
    pub name: String,
    /// Fields in declaration order
    pub fields: Vec<ObiField>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Total size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
}

/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObiEnumVariant {
    /// Variant name
    pub name: String,
    /// Variant value (discriminator)
    pub value: i64,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObiEnum {
    /// Enum name
    pub name: String,
    /// Variants
    pub variants: Vec<ObiEnumVariant>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Underlying type (default: i32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_type: Option<ObiPrimitiveType>,
}

/// Event category for built-in Hibana events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventCategory {
    /// System call events
    Syscall,
    /// Network events (socket, TCP/UDP, etc.)
    Network,
    /// File system events
    File,
    /// Process events (fork, exec, exit)
    Process,
    /// Security events (capability, permission checks)
    Security,
    /// Custom/user-defined events
    Custom,
}

/// Complete OBI schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObiSchema {
    /// Schema version
    #[serde(default = "default_version")]
    pub version: String,
    /// Mode: "embedded" for built-in types, "custom" for user-defined
    #[serde(default = "default_mode")]
    pub mode: String,
    /// Event category (for embedded mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<EventCategory>,
    /// Struct definitions
    #[serde(default)]
    pub structs: HashMap<String, ObiStruct>,
    /// Enum definitions
    #[serde(default)]
    pub enums: HashMap<String, ObiEnum>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn default_version() -> String {
    "1.0".to_string()
}

fn default_mode() -> String {
    "embedded".to_string()
}

impl ObiSchema {
    /// Create a new empty OBI schema
    pub fn new() -> Self {
        Self {
            version: default_version(),
            mode: default_mode(),
            category: None,
            structs: HashMap::new(),
            enums: HashMap::new(),
            description: None,
        }
    }

    /// Check if schema is in embedded mode
    pub fn is_embedded(&self) -> bool {
        self.mode == "embedded"
    }
}

impl Default for ObiSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in embedded event types for Hibana
pub mod embedded {
    use super::*;

    /// Create SyscallEvent struct definition
    pub fn syscall_event() -> ObiStruct {
        ObiStruct {
            name: "SyscallEvent".to_string(),
            description: Some("System call event from eBPF tracepoint".to_string()),
            size: Some(40), // 5 fields * 8 bytes
            fields: vec![
                ObiField {
                    name: "pid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Pid,
                    },
                    description: Some("Process ID".to_string()),
                    offset: Some(0),
                },
                ObiField {
                    name: "tid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Pid,
                    },
                    description: Some("Thread ID".to_string()),
                    offset: Some(4),
                },
                ObiField {
                    name: "syscall_nr".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U64,
                    },
                    description: Some("Syscall number".to_string()),
                    offset: Some(8),
                },
                ObiField {
                    name: "ret".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::I64,
                    },
                    description: Some("Return value".to_string()),
                    offset: Some(16),
                },
                ObiField {
                    name: "timestamp".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Timestamp,
                    },
                    description: Some("Event timestamp (ns)".to_string()),
                    offset: Some(24),
                },
            ],
        }
    }

    /// Create NetworkEvent struct definition
    pub fn network_event() -> ObiStruct {
        ObiStruct {
            name: "NetworkEvent".to_string(),
            description: Some("Network event from socket/TCP/UDP tracing".to_string()),
            size: Some(32),
            fields: vec![
                ObiField {
                    name: "pid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Pid,
                    },
                    description: Some("Process ID".to_string()),
                    offset: Some(0),
                },
                ObiField {
                    name: "saddr".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Ipv4Addr,
                    },
                    description: Some("Source IP address".to_string()),
                    offset: Some(4),
                },
                ObiField {
                    name: "daddr".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Ipv4Addr,
                    },
                    description: Some("Destination IP address".to_string()),
                    offset: Some(8),
                },
                ObiField {
                    name: "sport".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U16,
                    },
                    description: Some("Source port".to_string()),
                    offset: Some(12),
                },
                ObiField {
                    name: "dport".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U16,
                    },
                    description: Some("Destination port".to_string()),
                    offset: Some(14),
                },
                ObiField {
                    name: "protocol".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U8,
                    },
                    description: Some("Protocol (IPPROTO_TCP=6, IPPROTO_UDP=17)".to_string()),
                    offset: Some(16),
                },
            ],
        }
    }

    /// Create FileEvent struct definition
    pub fn file_event() -> ObiStruct {
        ObiStruct {
            name: "FileEvent".to_string(),
            description: Some("File system event from VFS hooks".to_string()),
            size: Some(280), // Approximate with string
            fields: vec![
                ObiField {
                    name: "pid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Pid,
                    },
                    description: Some("Process ID".to_string()),
                    offset: Some(0),
                },
                ObiField {
                    name: "filename".to_string(),
                    field_type: ObiType::Array {
                        element_type: Box::new(ObiType::Primitive {
                            prim_type: ObiPrimitiveType::U8,
                        }),
                        size: 256,
                    },
                    description: Some("File path (up to 256 chars)".to_string()),
                    offset: Some(8),
                },
                ObiField {
                    name: "flags".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U32,
                    },
                    description: Some("Open flags (O_RDONLY, O_WRONLY, etc.)".to_string()),
                    offset: Some(264),
                },
                ObiField {
                    name: "mode".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U32,
                    },
                    description: Some("File mode/permissions".to_string()),
                    offset: Some(268),
                },
            ],
        }
    }

    /// Create ProcessEvent struct definition
    pub fn process_event() -> ObiStruct {
        ObiStruct {
            name: "ProcessEvent".to_string(),
            description: Some("Process lifecycle event".to_string()),
            size: Some(32),
            fields: vec![
                ObiField {
                    name: "pid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Pid,
                    },
                    description: Some("Process ID".to_string()),
                    offset: Some(0),
                },
                ObiField {
                    name: "ppid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Pid,
                    },
                    description: Some("Parent process ID".to_string()),
                    offset: Some(4),
                },
                ObiField {
                    name: "uid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U32,
                    },
                    description: Some("User ID".to_string()),
                    offset: Some(8),
                },
                ObiField {
                    name: "gid".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::U32,
                    },
                    description: Some("Group ID".to_string()),
                    offset: Some(12),
                },
                ObiField {
                    name: "event_type".to_string(),
                    field_type: ObiType::Enum {
                        name: "ProcessEventType".to_string(),
                    },
                    description: Some("Event type (fork, exec, exit)".to_string()),
                    offset: Some(16),
                },
                ObiField {
                    name: "exit_code".to_string(),
                    field_type: ObiType::Option {
                        inner_type: Box::new(ObiType::Primitive {
                            prim_type: ObiPrimitiveType::I32,
                        }),
                    },
                    description: Some("Exit code (only for exit events)".to_string()),
                    offset: Some(20),
                },
                ObiField {
                    name: "timestamp".to_string(),
                    field_type: ObiType::Primitive {
                        prim_type: ObiPrimitiveType::Timestamp,
                    },
                    description: Some("Event timestamp (ns)".to_string()),
                    offset: Some(24),
                },
            ],
        }
    }

    /// Create ProcessEventType enum
    pub fn process_event_type_enum() -> ObiEnum {
        ObiEnum {
            name: "ProcessEventType".to_string(),
            description: Some("Type of process event".to_string()),
            underlying_type: Some(ObiPrimitiveType::U32),
            variants: vec![
                ObiEnumVariant {
                    name: "Fork".to_string(),
                    value: 1,
                    description: Some("Process forked".to_string()),
                },
                ObiEnumVariant {
                    name: "Exec".to_string(),
                    value: 2,
                    description: Some("Process executed new program".to_string()),
                },
                ObiEnumVariant {
                    name: "Exit".to_string(),
                    value: 3,
                    description: Some("Process exited".to_string()),
                },
            ],
        }
    }

    /// Get embedded schema for a specific category
    pub fn get_schema(category: EventCategory) -> ObiSchema {
        let mut schema = ObiSchema::new();
        schema.mode = "embedded".to_string();
        schema.category = Some(category.clone());

        match category {
            EventCategory::Syscall => {
                schema.structs.insert(
                    "SyscallEvent".to_string(),
                    syscall_event(),
                );
                schema.description = Some("Embedded syscall event types for Hibana".to_string());
            }
            EventCategory::Network => {
                schema.structs.insert(
                    "NetworkEvent".to_string(),
                    network_event(),
                );
                schema.description = Some("Embedded network event types for Hibana".to_string());
            }
            EventCategory::File => {
                schema.structs.insert(
                    "FileEvent".to_string(),
                    file_event(),
                );
                schema.description = Some("Embedded file event types for Hibana".to_string());
            }
            EventCategory::Process => {
                schema.structs.insert(
                    "ProcessEvent".to_string(),
                    process_event(),
                );
                schema.enums.insert(
                    "ProcessEventType".to_string(),
                    process_event_type_enum(),
                );
                schema.description = Some("Embedded process event types for Hibana".to_string());
            }
            _ => {
                // For custom or other categories, include all event types
                schema.structs.insert("SyscallEvent".to_string(), syscall_event());
                schema.structs.insert("NetworkEvent".to_string(), network_event());
                schema.structs.insert("FileEvent".to_string(), file_event());
                schema.structs.insert("ProcessEvent".to_string(), process_event());
                schema.enums.insert("ProcessEventType".to_string(), process_event_type_enum());
                schema.description = Some("All embedded event types for Hibana".to_string());
            }
        }

        schema
    }
}
