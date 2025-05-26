/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunObjectLastError : The last error associated with this run. Will be `null` if there are no errors.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunObjectLastError {
    /// One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`.
    #[serde(rename = "code")]
    pub code: String,
    /// A human-readable description of the error.
    #[serde(rename = "message")]
    pub message: String,
}
