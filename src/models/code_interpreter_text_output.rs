/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CodeInterpreterTextOutput : The output of a code interpreter tool call that is text.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeInterpreterTextOutput {
    /// The logs of the code interpreter tool call.
    #[serde(rename = "logs")]
    pub logs: String,
    /// The type of the code interpreter text output. Always `logs`.
    #[serde(rename = "type")]
    pub _type: String,
}
