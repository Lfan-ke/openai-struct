/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeSessionCreateResponseClientSecret : Ephemeral key returned by the API.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseClientSecret {
    /// Timestamp for when the token expires. Currently, all tokens expire after one minute.
    #[serde(rename = "expires_at")]
    pub expires_at: i32,
    /// Ephemeral key usable in client environments to authenticate connections to the Realtime API. Use this in client-side environments rather than a standard API token, which should only be used server-side.
    #[serde(rename = "value")]
    pub value: String,
}
