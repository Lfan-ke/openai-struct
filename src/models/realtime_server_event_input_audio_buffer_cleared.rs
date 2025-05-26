/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventInputAudioBufferCleared : Returned when the input audio buffer is cleared by the client with a  `input_audio_buffer.clear` event.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventInputAudioBufferCleared {
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  /// The event type, must be `input_audio_buffer.cleared`.
  #[serde(rename = "type")]
  pub _type: String
}
