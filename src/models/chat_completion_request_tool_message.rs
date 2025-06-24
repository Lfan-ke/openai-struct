/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

/// # on openapi.yaml
///
/// ```yaml
/// ChatCompletionRequestToolMessage:
///   type: object
///   title: Tool message
///   properties:
///     role:
///       type: string
///       enum:
///         - tool
///       description: The role of the messages author, in this case `tool`.
///       x-stainless-const: true
///     content:
///       oneOf:
///         - type: string
///           description: The contents of the tool message.
///           title: Text content
///         - type: array
///           description:
///             An array of content parts with a defined type. For tool messages,
///             only type `text` is supported.
///           title: Array of content parts
///           items:
///             $ref: "#/components/schemas/ChatCompletionRequestToolMessageContentPart"
///           minItems: 1
///       description: The contents of the tool message.
///     tool_call_id:
///       type: string
///       description: Tool call that this message is responding to.
///   required:
///     - role
///     - content
///     - tool_call_id
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestToolMessage {
    /// The contents of the tool message.
    #[serde(rename = "content")]
    pub content: Value,
    /// Tool call that this message is responding to.
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: String,
}
