/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CodeInterpreterToolCall : A tool call to run code.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeInterpreterToolCall {
  /// The code to run.
  #[serde(rename = "code")]
  pub code: String,
  /// The unique ID of the code interpreter tool call.
  #[serde(rename = "id")]
  pub id: String,
  /// The results of the code interpreter tool call.
  #[serde(rename = "results")]
  pub results: Vec<crate::models::CodeInterpreterToolOutput>,
  /// The status of the code interpreter tool call.
  #[serde(rename = "status")]
  pub status: String,
  /// The type of the code interpreter tool call. Always `code_interpreter_call`.
  #[serde(rename = "type")]
  pub _type: String
}
