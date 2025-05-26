/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventRateLimitsUpdated : Emitted at the beginning of a Response to indicate the updated rate limits.  When a Response is created some tokens will be \"reserved\" for the output  tokens, the rate limits shown here reflect that reservation, which is then  adjusted accordingly once the Response is completed.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventRateLimitsUpdated {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// List of rate limit information.
    #[serde(rename = "rate_limits")]
    pub rate_limits: Vec<crate::models::RealtimeServerEventRateLimitsUpdatedRateLimits>,
    /// The event type, must be `rate_limits.updated`.
    #[serde(rename = "type")]
    pub _type: String,
}
