/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventConversationItemInputAudioTranscriptionFailed : Returned when input audio transcription is configured, and a transcription  request for a user message failed. These events are separate from other  `error` events so that the client can identify the related Item.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
  /// The index of the content part containing the audio.
  #[serde(rename = "content_index")]
  pub content_index: i32,
  #[serde(rename = "error")]
  pub error: crate::models::RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  /// The ID of the user message item.
  #[serde(rename = "item_id")]
  pub item_id: String,
  /// The event type, must be `conversation.item.input_audio_transcription.failed`.
  #[serde(rename = "type")]
  pub _type: String
}
