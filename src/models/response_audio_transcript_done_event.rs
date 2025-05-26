/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseAudioTranscriptDoneEvent : Emitted when the full audio transcript is completed.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAudioTranscriptDoneEvent {
  /// The type of the event. Always `response.audio.transcript.done`.
  #[serde(rename = "type")]
  pub _type: String
}
