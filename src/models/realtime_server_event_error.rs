/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventError : Returned when an error occurs, which could be a client problem or a server  problem. Most errors are recoverable and the session will stay open, we  recommend to implementors to monitor and log error messages by default.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventError {
  #[serde(rename = "error")]
  pub error: crate::models::RealtimeServerEventErrorError,
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  /// The event type, must be `error`.
  #[serde(rename = "type")]
  pub _type: String
}
