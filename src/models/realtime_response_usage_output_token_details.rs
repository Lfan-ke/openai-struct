/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeResponseUsageOutputTokenDetails : Details about the output tokens used in the Response.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeResponseUsageOutputTokenDetails {
    /// The number of audio tokens used in the Response.
    #[serde(rename = "audio_tokens")]
    pub audio_tokens: Option<i32>,
    /// The number of text tokens used in the Response.
    #[serde(rename = "text_tokens")]
    pub text_tokens: Option<i32>,
}
