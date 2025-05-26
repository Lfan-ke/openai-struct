/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CompletionUsagePromptTokensDetails : Breakdown of tokens used in the prompt.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionUsagePromptTokensDetails {
  /// Audio input tokens present in the prompt.
  #[serde(rename = "audio_tokens")]
  pub audio_tokens: Option<i32>,
  /// Cached tokens present in the prompt.
  #[serde(rename = "cached_tokens")]
  pub cached_tokens: Option<i32>
}
