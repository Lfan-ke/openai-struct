/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventOutputAudioBufferStopped : **WebRTC Only:** Emitted when the output audio buffer has been completely drained on the server, and no more audio is forthcoming. This event is emitted after the full response data has been sent to the client (`response.done`). [Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    #[serde(rename = "response_id")]
    pub response_id: String,
    /// The event type, must be `output_audio_buffer.stopped`.
    #[serde(rename = "type")]
    pub _type: String,
}
