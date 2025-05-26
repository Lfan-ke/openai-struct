/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeResponseUsage : Usage statistics for the Response, this will correspond to billing. A  Realtime API session will maintain a conversation context and append new  Items to the Conversation, thus output from previous turns (text and  audio tokens) will become the input for later turns.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeResponseUsage {
  #[serde(rename = "input_token_details")]
  pub input_token_details: Option<crate::models::RealtimeResponseUsageInputTokenDetails>,
  /// The number of input tokens used in the Response, including text and  audio tokens.
  #[serde(rename = "input_tokens")]
  pub input_tokens: Option<i32>,
  #[serde(rename = "output_token_details")]
  pub output_token_details: Option<crate::models::RealtimeResponseUsageOutputTokenDetails>,
  /// The number of output tokens sent in the Response, including text and  audio tokens.
  #[serde(rename = "output_tokens")]
  pub output_tokens: Option<i32>,
  /// The total number of tokens in the Response including input and output  text and audio tokens.
  #[serde(rename = "total_tokens")]
  pub total_tokens: Option<i32>
}
