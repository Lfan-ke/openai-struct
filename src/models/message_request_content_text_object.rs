/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageRequestContentTextObject : The text content that is part of a message.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRequestContentTextObject {
    /// Text content to be sent to the model
    #[serde(rename = "text")]
    pub text: String,
    /// Always `text`.
    #[serde(rename = "type")]
    pub _type: String,
}
