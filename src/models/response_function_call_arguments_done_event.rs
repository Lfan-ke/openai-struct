/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFunctionCallArgumentsDoneEvent : Emitted when function-call arguments are finalized.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
  /// The function-call arguments.
  #[serde(rename = "arguments")]
  pub arguments: String,
  /// The ID of the item.
  #[serde(rename = "item_id")]
  pub item_id: String,
  /// The index of the output item.
  #[serde(rename = "output_index")]
  pub output_index: i32,
  #[serde(rename = "type")]
  pub _type: String
}
