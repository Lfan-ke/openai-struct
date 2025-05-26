/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFailedEvent : An event that is emitted when a response fails.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFailedEvent {
    /// The response that failed.
    #[serde(rename = "response")]
    pub response: crate::models::Response,
    /// The type of the event. Always `response.failed`.
    #[serde(rename = "type")]
    pub _type: String,
}
