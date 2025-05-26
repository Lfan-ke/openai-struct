/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFunctionCallArgumentsDeltaEvent : Emitted when there is a partial function-call arguments delta.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
  /// The function-call arguments delta that is added.
  #[serde(rename = "delta")]
  pub delta: String,
  /// The ID of the output item that the function-call arguments delta is added to.
  #[serde(rename = "item_id")]
  pub item_id: String,
  /// The index of the output item that the function-call arguments delta is added to.
  #[serde(rename = "output_index")]
  pub output_index: i32,
  /// The type of the event. Always `response.function_call_arguments.delta`.
  #[serde(rename = "type")]
  pub _type: String
}
