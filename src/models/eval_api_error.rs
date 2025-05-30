/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub EvalApiError : An object representing an error response from the Eval API.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalApiError {
    /// The error code.
    #[serde(rename = "code")]
    pub code: String,
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
}
