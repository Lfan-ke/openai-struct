/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub OutputTextContent : A text output from the model.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputTextContent {
  /// The annotations of the text output.
  #[serde(rename = "annotations")]
  pub annotations: Vec<crate::models::Annotation>,
  /// The text output from the model.
  #[serde(rename = "text")]
  pub text: String,
  /// The type of the output text. Always `output_text`.
  #[serde(rename = "type")]
  pub _type: String
}
