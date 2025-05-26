/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunStepCompletionUsage : Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepCompletionUsage {
  /// Number of completion tokens used over the course of the run step.
  #[serde(rename = "completion_tokens")]
  pub completion_tokens: i32,
  /// Number of prompt tokens used over the course of the run step.
  #[serde(rename = "prompt_tokens")]
  pub prompt_tokens: i32,
  /// Total number of tokens used (prompt + completion).
  #[serde(rename = "total_tokens")]
  pub total_tokens: i32
}
