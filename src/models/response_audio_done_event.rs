/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseAudioDoneEvent : Emitted when the audio response is complete.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAudioDoneEvent {
    /// The type of the event. Always `response.audio.done`.
    #[serde(rename = "type")]
    pub _type: String,
}
