/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ComputerToolCall : A tool call to a computer use tool. See the  [computer use guide](/docs/guides/tools-computer-use) for more information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolCall {
    #[serde(rename = "action")]
    pub action: crate::models::ComputerAction,
    /// An identifier used when responding to the tool call with output.
    #[serde(rename = "call_id")]
    pub call_id: String,
    /// The unique ID of the computer call.
    #[serde(rename = "id")]
    pub id: String,
    /// The pending safety checks for the computer call.
    #[serde(rename = "pending_safety_checks")]
    pub pending_safety_checks: Vec<crate::models::ComputerToolCallSafetyCheck>,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of the computer call. Always `computer_call`.
    #[serde(rename = "type")]
    pub _type: String,
}
