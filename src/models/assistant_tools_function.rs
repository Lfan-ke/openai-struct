/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantToolsFunction {
    #[serde(rename = "function")]
    pub function: crate::models::FunctionObject,
    /// The type of tool being defined: `function`
    #[serde(rename = "type")]
    pub _type: String,
}
