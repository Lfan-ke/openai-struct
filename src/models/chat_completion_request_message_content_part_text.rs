/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestMessageContentPartText : Learn about [text inputs](/docs/guides/text-generation).

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartText {
  /// The text content.
  #[serde(rename = "text")]
  pub text: String,
  /// The type of the content part.
  #[serde(rename = "type")]
  pub _type: String
}
