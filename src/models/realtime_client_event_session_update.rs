/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeClientEventSessionUpdate : Send this event to update the session’s default configuration. The client may send this event at any time to update any field, except for `voice`. However, note that once a session has been initialized with a particular `model`, it can’t be changed to another model using `session.update`.  When the server receives a `session.update`, it will respond with a `session.updated` event showing the full, effective configuration. Only the fields that are present are updated. To clear a field like `instructions`, pass an empty string.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeClientEventSessionUpdate {
  /// Optional client-generated ID used to identify this event.
  #[serde(rename = "event_id")]
  pub event_id: Option<String>,
  #[serde(rename = "session")]
  pub session: crate::models::RealtimeSessionCreateRequest,
  /// The event type, must be `session.update`.
  #[serde(rename = "type")]
  pub _type: String
}
