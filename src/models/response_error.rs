/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseError : An error object returned when the model fails to generate a Response.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    #[serde(rename = "code")]
    pub code: crate::models::ResponseErrorCode,
    /// A human-readable description of the error.
    #[serde(rename = "message")]
    pub message: String,
}
