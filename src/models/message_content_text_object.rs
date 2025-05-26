/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageContentTextObject : The text content that is part of a message.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageContentTextObject {
    #[serde(rename = "text")]
    pub text: crate::models::MessageContentTextObjectText,
    /// Always `text`.
    #[serde(rename = "type")]
    pub _type: String,
}
