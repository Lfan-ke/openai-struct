/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ComputerCallOutputItemParam : The output of a computer tool call.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerCallOutputItemParam {
    #[serde(rename = "acknowledged_safety_checks")]
    pub acknowledged_safety_checks: Option<Value>,
    /// The ID of the computer tool call that produced the output.
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[serde(rename = "id")]
    pub id: Option<Value>,
    #[serde(rename = "output")]
    pub output: crate::models::ComputerScreenshotImage,
    #[serde(rename = "status")]
    pub status: Option<Value>,
    /// The type of the computer tool call output. Always `computer_call_output`.
    #[serde(rename = "type")]
    pub _type: String,
}
