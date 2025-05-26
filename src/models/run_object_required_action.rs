/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunObjectRequiredAction : Details on the action required to continue the run. Will be `null` if no action is required.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunObjectRequiredAction {
    #[serde(rename = "submit_tool_outputs")]
    pub submit_tool_outputs: crate::models::RunObjectRequiredActionSubmitToolOutputs,
    /// For now, this is always `submit_tool_outputs`.
    #[serde(rename = "type")]
    pub _type: String,
}
