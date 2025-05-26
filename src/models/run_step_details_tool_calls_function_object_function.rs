/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunStepDetailsToolCallsFunctionObjectFunction : The definition of the function that was called.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
    /// The arguments passed to the function.
    #[serde(rename = "arguments")]
    pub arguments: String,
    /// The name of the function.
    #[serde(rename = "name")]
    pub name: String,
    /// The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet.
    #[serde(rename = "output")]
    pub output: String,
}
