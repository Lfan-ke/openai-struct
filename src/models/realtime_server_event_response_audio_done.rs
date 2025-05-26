/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventResponseAudioDone : Returned when the model-generated audio is done. Also emitted when a Response is interrupted, incomplete, or cancelled.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseAudioDone {
    /// The index of the content part in the item's content array.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the item.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item in the response.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The ID of the response.
    #[serde(rename = "response_id")]
    pub response_id: String,
    /// The event type, must be `response.audio.done`.
    #[serde(rename = "type")]
    pub _type: String,
}
