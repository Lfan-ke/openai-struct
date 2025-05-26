/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeResponseStatusDetailsError : A description of the error that caused the response to fail,  populated when the `status` is `failed`.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeResponseStatusDetailsError {
    /// Error code, if any.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// The type of error.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
