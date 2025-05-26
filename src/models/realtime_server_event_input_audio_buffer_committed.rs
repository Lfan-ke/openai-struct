/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventInputAudioBufferCommitted : Returned when an input audio buffer is committed, either by the client or  automatically in server VAD mode. The `item_id` property is the ID of the user message item that will be created, thus a `conversation.item.created` event  will also be sent to the client.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    /// The unique ID of the server event.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The ID of the user message item that will be created.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The ID of the preceding item after which the new item will be inserted.
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: String,
    /// The event type, must be `input_audio_buffer.committed`.
    #[serde(rename = "type")]
    pub _type: String,
}
