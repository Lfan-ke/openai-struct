/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ComputerToolCallSafetyCheck : A pending safety check for the computer call.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolCallSafetyCheck {
    /// The type of the pending safety check.
    #[serde(rename = "code")]
    pub code: String,
    /// The ID of the pending safety check.
    #[serde(rename = "id")]
    pub id: String,
    /// Details about the pending safety check.
    #[serde(rename = "message")]
    pub message: String,
}
