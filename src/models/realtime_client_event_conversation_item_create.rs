/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeClientEventConversationItemCreate : Add a new Item to the Conversation's context, including messages, function  calls, and function call responses. This event can be used both to populate a  \"history\" of the conversation and to add new items mid-stream, but has the  current limitation that it cannot populate assistant audio messages.  If successful, the server will respond with a `conversation.item.created`  event, otherwise an `error` event will be sent.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeClientEventConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[serde(rename = "item")]
    pub item: crate::models::RealtimeConversationItem,
    /// The ID of the preceding item after which the new item will be inserted.  If not set, the new item will be appended to the end of the conversation. If set to `root`, the new item will be added to the beginning of the conversation. If set to an existing ID, it allows an item to be inserted mid-conversation. If the ID cannot be found, an error will be returned and the item will not be added.
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: Option<String>,
    /// The event type, must be `conversation.item.create`.
    #[serde(rename = "type")]
    pub _type: String,
}
