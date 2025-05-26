/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeClientEventConversationItemRetrieve : Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD. The server will respond with a `conversation.item.retrieved` event,  unless the item does not exist in the conversation history, in which case the  server will respond with an error.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeClientEventConversationItemRetrieve {
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// The ID of the item to retrieve.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The event type, must be `conversation.item.retrieve`.
    #[serde(rename = "type")]
    pub _type: String,
}
