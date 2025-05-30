/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageContentRefusalObject : The refusal content generated by the assistant.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageContentRefusalObject {
    #[serde(rename = "refusal")]
    pub refusal: String,
    /// Always `refusal`.
    #[serde(rename = "type")]
    pub _type: String,
}
