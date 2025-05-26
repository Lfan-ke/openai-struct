/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventResponseDone : Returned when a Response is done streaming. Always emitted, no matter the  final state. The Response object included in the `response.done` event will  include all output Items in the Response but will omit the raw audio data.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseDone {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "response")]
    pub response: crate::models::RealtimeResponse,
    /// The event type, must be `response.done`.
    #[serde(rename = "type")]
    pub _type: String,
}
