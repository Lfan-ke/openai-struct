/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeClientEventTranscriptionSessionUpdate : Send this event to update a transcription session.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[serde(rename = "session")]
    pub session: crate::models::RealtimeTranscriptionSessionCreateRequest,
    /// The event type, must be `transcription_session.update`.
    #[serde(rename = "type")]
    pub _type: String,
}
