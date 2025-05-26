/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventConversationCreated : Returned when a conversation is created. Emitted right after session creation.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationCreated {
  #[serde(rename = "conversation")]
  pub conversation: crate::models::RealtimeServerEventConversationCreatedConversation,
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  /// The event type, must be `conversation.created`.
  #[serde(rename = "type")]
  pub _type: String
}
