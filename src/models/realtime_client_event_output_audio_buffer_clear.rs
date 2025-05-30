/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeClientEventOutputAudioBufferClear : **WebRTC Only:** Emit to cut off the current audio response. This will trigger the server to stop generating audio and emit a `output_audio_buffer.cleared` event. This  event should be preceded by a `response.cancel` client event to stop the  generation of the current response. [Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeClientEventOutputAudioBufferClear {
    /// The unique ID of the client event used for error handling.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// The event type, must be `output_audio_buffer.clear`.
    #[serde(rename = "type")]
    pub _type: String,
}
