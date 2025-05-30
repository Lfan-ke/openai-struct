/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventConversationItemCreated : Returned when a conversation item is created. There are several scenarios that produce this event:   - The server is generating a Response, which if successful will produce      either one or two Items, which will be of type `message`      (role `assistant`) or type `function_call`.   - The input audio buffer has been committed, either by the client or the      server (in `server_vad` mode). The server will take the content of the      input audio buffer and add it to a new user message Item.   - The client has sent a `conversation.item.create` event to add a new Item      to the Conversation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemCreated {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "item")]
    pub item: crate::models::RealtimeConversationItem,
    /// The ID of the preceding item in the Conversation context, allows the  client to understand the order of the conversation.
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: String,
    /// The event type, must be `conversation.item.created`.
    #[serde(rename = "type")]
    pub _type: String,
}
