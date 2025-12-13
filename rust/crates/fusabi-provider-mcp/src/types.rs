//! MCP (Model Context Protocol) type definitions
//!
//! Defines the structure of MCP schemas including tools, resources, prompts,
//! and protocol messages.

use std::collections::HashMap;

/// MCP schema type representing the complete specification
#[derive(Debug, Clone, Default)]
pub struct McpSchema {
    /// Tool definitions
    pub tools: Vec<ToolDefinition>,
    /// Resource definitions
    pub resources: Vec<ResourceDefinition>,
    /// Prompt definitions
    pub prompts: Vec<PromptDefinition>,
    /// Custom type definitions
    pub definitions: HashMap<String, TypeDefinition>,
}

/// MCP tool definition
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: Option<String>,
    /// Input schema (JSON Schema)
    pub input_schema: Option<JsonSchemaObject>,
}

/// MCP resource definition
#[derive(Debug, Clone)]
pub struct ResourceDefinition {
    /// Resource URI template
    pub uri: String,
    /// Resource name
    pub name: String,
    /// Resource description
    pub description: Option<String>,
    /// MIME type
    pub mime_type: Option<String>,
}

/// MCP prompt definition
#[derive(Debug, Clone)]
pub struct PromptDefinition {
    /// Prompt name
    pub name: String,
    /// Prompt description
    pub description: Option<String>,
    /// Prompt arguments
    pub arguments: Vec<PromptArgument>,
}

/// MCP prompt argument
#[derive(Debug, Clone)]
pub struct PromptArgument {
    /// Argument name
    pub name: String,
    /// Argument description
    pub description: Option<String>,
    /// Whether the argument is required
    pub required: bool,
}

/// JSON Schema object for tool input schemas
#[derive(Debug, Clone, Default)]
pub struct JsonSchemaObject {
    /// Schema type
    pub schema_type: Option<String>,
    /// Properties
    pub properties: HashMap<String, JsonSchemaProperty>,
    /// Required properties
    pub required: Vec<String>,
    /// Additional properties allowed
    pub additional_properties: bool,
}

/// JSON Schema property
#[derive(Debug, Clone)]
pub struct JsonSchemaProperty {
    /// Property type
    pub property_type: String,
    /// Property description
    pub description: Option<String>,
    /// Enum values
    pub enum_values: Vec<serde_json::Value>,
    /// Items schema (for arrays)
    pub items: Option<Box<JsonSchemaProperty>>,
    /// Nested properties (for objects)
    pub properties: HashMap<String, JsonSchemaProperty>,
    /// Default value
    pub default: Option<serde_json::Value>,
}

/// Custom type definition in MCP schema
#[derive(Debug, Clone)]
pub struct TypeDefinition {
    /// Type name
    pub name: String,
    /// Type kind
    pub kind: TypeKind,
}

/// Kind of type definition
#[derive(Debug, Clone)]
pub enum TypeKind {
    /// Object/record type
    Object {
        properties: HashMap<String, JsonSchemaProperty>,
        required: Vec<String>,
    },
    /// Enum/discriminated union
    Enum { values: Vec<String> },
    /// Union type (oneOf)
    Union { variants: Vec<TypeDefinition> },
}

/// MCP content type (for responses)
#[derive(Debug, Clone)]
pub enum ContentType {
    /// Text content
    Text { text: String },
    /// Image content
    Image {
        url: String,
        mime_type: String,
    },
    /// Resource content
    Resource { uri: String },
}

/// MCP protocol message types
#[derive(Debug, Clone)]
pub enum MessageType {
    /// Request message
    Request,
    /// Response message
    Response,
    /// Notification message
    Notification,
}

/// Built-in MCP protocol types for embedded mode
pub const EMBEDDED_MCP_TYPES: &str = r#"
// MCP Protocol Core Types

type JsonRpcVersion = "2.0"

type RequestId = string | int

// Base protocol messages
type JsonRpcRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: string,
  params: any option
}

type JsonRpcResponse = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  result: any option,
  error: JsonRpcError option
}

type JsonRpcNotification = {
  jsonrpc: JsonRpcVersion,
  method: string,
  params: any option
}

type JsonRpcError = {
  code: int,
  message: string,
  data: any option
}

// MCP Content types
type TextContent = {
  type: "text",
  text: string
}

type ImageContent = {
  type: "image",
  data: string,
  mimeType: string
}

type ResourceContent = {
  type: "resource",
  resource: EmbeddedResource
}

type Content =
  | TextContent
  | ImageContent
  | ResourceContent

// MCP Tool types
type Tool = {
  name: string,
  description: string option,
  inputSchema: any
}

type ToolCall = {
  name: string,
  arguments: any
}

type ToolResult = {
  content: Content list,
  isError: bool option
}

// MCP Resource types
type Resource = {
  uri: string,
  name: string,
  description: string option,
  mimeType: string option
}

type EmbeddedResource = {
  uri: string,
  mimeType: string option,
  text: string option,
  blob: string option
}

type ResourceContents = {
  uri: string,
  mimeType: string option,
  text: string option,
  blob: string option
}

// MCP Prompt types
type Prompt = {
  name: string,
  description: string option,
  arguments: PromptArgument list option
}

type PromptArgument = {
  name: string,
  description: string option,
  required: bool option
}

type PromptMessage = {
  role: "user" | "assistant",
  content: Content
}

type GetPromptResult = {
  description: string option,
  messages: PromptMessage list
}

// MCP Server capabilities
type ServerCapabilities = {
  experimental: any option,
  logging: any option,
  prompts: PromptsCapability option,
  resources: ResourcesCapability option,
  tools: ToolsCapability option
}

type PromptsCapability = {
  listChanged: bool option
}

type ResourcesCapability = {
  subscribe: bool option,
  listChanged: bool option
}

type ToolsCapability = {
  listChanged: bool option
}

// MCP Client capabilities
type ClientCapabilities = {
  experimental: any option,
  sampling: any option,
  roots: RootsCapability option
}

type RootsCapability = {
  listChanged: bool option
}

// MCP Initialize
type InitializeRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "initialize",
  params: InitializeParams
}

type InitializeParams = {
  protocolVersion: string,
  capabilities: ClientCapabilities,
  clientInfo: Implementation
}

type Implementation = {
  name: string,
  version: string
}

type InitializeResult = {
  protocolVersion: string,
  capabilities: ServerCapabilities,
  serverInfo: Implementation
}

// MCP List operations
type ListToolsRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "tools/list",
  params: any option
}

type ListToolsResult = {
  tools: Tool list
}

type ListResourcesRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "resources/list",
  params: any option
}

type ListResourcesResult = {
  resources: Resource list
}

type ListPromptsRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "prompts/list",
  params: any option
}

type ListPromptsResult = {
  prompts: Prompt list
}

// MCP Call operations
type CallToolRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "tools/call",
  params: CallToolParams
}

type CallToolParams = {
  name: string,
  arguments: any option
}

type ReadResourceRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "resources/read",
  params: ReadResourceParams
}

type ReadResourceParams = {
  uri: string
}

type ReadResourceResult = {
  contents: ResourceContents list
}

type GetPromptRequest = {
  jsonrpc: JsonRpcVersion,
  id: RequestId,
  method: "prompts/get",
  params: GetPromptParams
}

type GetPromptParams = {
  name: string,
  arguments: any option
}
"#;
