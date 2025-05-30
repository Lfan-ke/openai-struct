/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeClientEventInputAudioBufferClear : Send this event to clear the audio bytes in the buffer. The server will  respond with an `input_audio_buffer.cleared` event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeClientEventInputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// The event type, must be `input_audio_buffer.clear`.
    #[serde(rename = "type")]
    pub _type: String,
}
