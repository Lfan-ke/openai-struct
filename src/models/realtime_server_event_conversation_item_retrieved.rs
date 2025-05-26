/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventConversationItemRetrieved : Returned when a conversation item is retrieved with `conversation.item.retrieve`.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemRetrieved {
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  #[serde(rename = "item")]
  pub item: crate::models::RealtimeConversationItem,
  /// The event type, must be `conversation.item.retrieved`.
  #[serde(rename = "type")]
  pub _type: String
}
