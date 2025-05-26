/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventTranscriptionSessionUpdated : Returned when a transcription session is updated with a `transcription_session.update` event, unless  there is an error.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  #[serde(rename = "session")]
  pub session: crate::models::RealtimeTranscriptionSessionCreateResponse,
  /// The event type, must be `transcription_session.updated`.
  #[serde(rename = "type")]
  pub _type: String
}
