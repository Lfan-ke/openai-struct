/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseWebSearchCallInProgressEvent : Emitted when a web search call is initiated.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWebSearchCallInProgressEvent {
  /// Unique ID for the output item associated with the web search call.
  #[serde(rename = "item_id")]
  pub item_id: String,
  /// The index of the output item that the web search call is associated with.
  #[serde(rename = "output_index")]
  pub output_index: i32,
  /// The type of the event. Always `response.web_search_call.in_progress`.
  #[serde(rename = "type")]
  pub _type: String
}
