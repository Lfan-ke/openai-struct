/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFileSearchCallCompletedEvent : Emitted when a file search call is completed (results found).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFileSearchCallCompletedEvent {
    /// The ID of the output item that the file search call is initiated.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item that the file search call is initiated.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The type of the event. Always `response.file_search_call.completed`.
    #[serde(rename = "type")]
    pub _type: String,
}
