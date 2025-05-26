/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseCodeInterpreterCallCodeDeltaEvent : Emitted when a partial code snippet is added by the code interpreter.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
  /// The partial code snippet added by the code interpreter.
  #[serde(rename = "delta")]
  pub delta: String,
  /// The index of the output item that the code interpreter call is in progress.
  #[serde(rename = "output_index")]
  pub output_index: i32,
  /// The type of the event. Always `response.code_interpreter_call.code.delta`.
  #[serde(rename = "type")]
  pub _type: String
}
