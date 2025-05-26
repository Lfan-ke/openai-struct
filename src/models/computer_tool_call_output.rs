/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ComputerToolCallOutput : The output of a computer tool call.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolCallOutput {
    /// The safety checks reported by the API that have been acknowledged by the  developer.
    #[serde(rename = "acknowledged_safety_checks")]
    pub acknowledged_safety_checks: Option<Vec<crate::models::ComputerToolCallSafetyCheck>>,
    /// The ID of the computer tool call that produced the output.
    #[serde(rename = "call_id")]
    pub call_id: String,
    /// The ID of the computer tool call output.
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "output")]
    pub output: crate::models::ComputerScreenshotImage,
    /// The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The type of the computer tool call output. Always `computer_call_output`.
    #[serde(rename = "type")]
    pub _type: String,
}
