/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestUserMessage : Messages sent by an end user, containing prompts or additional context information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestUserMessage {
    /// The contents of the user message.
    #[serde(rename = "content")]
    pub content: Value,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The role of the messages author, in this case `user`.
    #[serde(rename = "role")]
    pub role: String,
}
