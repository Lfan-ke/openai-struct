/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventResponseContentPartAdded : Returned when a new content part is added to an assistant message item during response generation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseContentPartAdded {
    /// The index of the content part in the item's content array.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the item to which the content part was added.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item in the response.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    #[serde(rename = "part")]
    pub part: crate::models::RealtimeServerEventResponseContentPartAddedPart,
    /// The ID of the response.
    #[serde(rename = "response_id")]
    pub response_id: String,
    /// The event type, must be `response.content_part.added`.
    #[serde(rename = "type")]
    pub _type: String,
}
