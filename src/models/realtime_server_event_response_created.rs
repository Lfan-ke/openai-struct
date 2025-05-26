/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventResponseCreated : Returned when a new Response is created. The first event of response creation, where the response is in an initial state of `in_progress`.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseCreated {
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  #[serde(rename = "response")]
  pub response: crate::models::RealtimeResponse,
  /// The event type, must be `response.created`.
  #[serde(rename = "type")]
  pub _type: String
}
