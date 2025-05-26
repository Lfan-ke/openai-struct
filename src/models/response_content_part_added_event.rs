/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseContentPartAddedEvent : Emitted when a new content part is added.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContentPartAddedEvent {
    /// The index of the content part that was added.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The ID of the output item that the content part was added to.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item that the content part was added to.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The content part that was added.
    #[serde(rename = "part")]
    pub part: crate::models::OutputContent,
    /// The type of the event. Always `response.content_part.added`.
    #[serde(rename = "type")]
    pub _type: String,
}
