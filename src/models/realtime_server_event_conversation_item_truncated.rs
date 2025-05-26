/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventConversationItemTruncated : Returned when an earlier assistant audio message item is truncated by the  client with a `conversation.item.truncate` event. This event is used to  synchronize the server's understanding of the audio with the client's playback.  This action will truncate the audio and remove the server-side text transcript  to ensure there is no text in the context that hasn't been heard by the user.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemTruncated {
    /// The duration up to which the audio was truncated, in milliseconds.
    #[serde(rename = "audio_end_ms")]
    pub audio_end_ms: i32,
    /// The index of the content part that was truncated.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the assistant message item that was truncated.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The event type, must be `conversation.item.truncated`.
    #[serde(rename = "type")]
    pub _type: String,
}
