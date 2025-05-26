/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ToolChoiceFunction : Use this option to force the model to call a specific function.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    #[serde(rename = "name")]
    pub name: String,
    /// For function calling, the type is always `function`.
    #[serde(rename = "type")]
    pub _type: String,
}
