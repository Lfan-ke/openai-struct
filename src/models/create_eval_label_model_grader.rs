/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateEvalLabelModelGrader : A LabelModelGrader object which uses a model to assign labels to each item in the evaluation.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvalLabelModelGrader {
  /// A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}.
  #[serde(rename = "input")]
  pub input: Vec<crate::models::CreateEvalItem>,
  /// The labels to classify to each item in the evaluation.
  #[serde(rename = "labels")]
  pub labels: Vec<String>,
  /// The model to use for the evaluation. Must support structured outputs.
  #[serde(rename = "model")]
  pub model: String,
  /// The name of the grader.
  #[serde(rename = "name")]
  pub name: String,
  /// The labels that indicate a passing result. Must be a subset of labels.
  #[serde(rename = "passing_labels")]
  pub passing_labels: Vec<String>,
  /// The object type, which is always `label_model`.
  #[serde(rename = "type")]
  pub _type: String
}
