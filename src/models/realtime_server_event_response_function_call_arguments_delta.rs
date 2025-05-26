/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeServerEventResponseFunctionCallArgumentsDelta : Returned when the model-generated function call arguments are updated.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
  /// The ID of the function call.
  #[serde(rename = "call_id")]
  pub call_id: String,
  /// The arguments delta as a JSON string.
  #[serde(rename = "delta")]
  pub delta: String,
  /// The unique ID of the server event.
  #[serde(rename = "event_id")]
  pub event_id: String,
  /// The ID of the function call item.
  #[serde(rename = "item_id")]
  pub item_id: String,
  /// The index of the output item in the response.
  #[serde(rename = "output_index")]
  pub output_index: i32,
  /// The ID of the response.
  #[serde(rename = "response_id")]
  pub response_id: String,
  /// The event type, must be `response.function_call_arguments.delta`.
  #[serde(rename = "type")]
  pub _type: String
}
