/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventSessionCreated : Returned when a Session is created. Emitted automatically when a new  connection is established as the first server event. This event will contain  the default Session configuration.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventSessionCreated {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "session")]
    pub session: crate::models::RealtimeSession,
    /// The event type, must be `session.created`.
    #[serde(rename = "type")]
    pub _type: String,
}
