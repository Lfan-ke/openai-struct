/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestUserMessage : Messages sent by an end user, containing prompts or additional context information.

#[allow(unused_imports)]
use serde_json::Value;

/// # on openapi.yaml
///
/// ```yaml
/// ChatCompletionRequestUserMessage:
///   type: object
///   title: User message
///   description: |
///     Messages sent by an end user, containing prompts or additional context
///     information.
///   properties:
///     content:
///       description: |
///         The contents of the user message.
///       oneOf:
///         - type: string
///           description: The text contents of the message.
///           title: Text content
///         - type: array
///           description:
///             An array of content parts with a defined type. Supported options
///             differ based on the [model](/docs/models) being used to generate
///             the response. Can contain text, image, or audio inputs.
///           title: Array of content parts
///           items:
///             $ref: "#/components/schemas/ChatCompletionRequestUserMessageContentPart"
///           minItems: 1
///     role:
///       type: string
///       enum:
///         - user
///       description: The role of the messages author, in this case `user`.
///       x-stainless-const: true
///     name:
///       type: string
///       description: An optional name for the participant. Provides the model
///         information to differentiate between participants of the same role.
///   required:
///     - content
///     - role
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestUserMessage {
    /// The contents of the user message.
    #[serde(rename = "content")]
    pub content: Value,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
