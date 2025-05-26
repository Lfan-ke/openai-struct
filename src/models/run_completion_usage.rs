/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunCompletionUsage : Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.).

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunCompletionUsage {
  /// Number of completion tokens used over the course of the run.
  #[serde(rename = "completion_tokens")]
  pub completion_tokens: i32,
  /// Number of prompt tokens used over the course of the run.
  #[serde(rename = "prompt_tokens")]
  pub prompt_tokens: i32,
  /// Total number of tokens used (prompt + completion).
  #[serde(rename = "total_tokens")]
  pub total_tokens: i32
}
