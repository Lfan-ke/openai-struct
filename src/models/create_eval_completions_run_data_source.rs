/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateEvalCompletionsRunDataSource : A CompletionsRunDataSource object describing a model sampling configuration.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvalCompletionsRunDataSource {
  #[serde(rename = "input_messages")]
  pub input_messages: Option<Value>,
  /// The name of the model to use for generating completions (e.g. \"o3-mini\").
  #[serde(rename = "model")]
  pub model: Option<String>,
  #[serde(rename = "sampling_params")]
  pub sampling_params: Option<crate::models::CreateEvalCompletionsRunDataSourceSamplingParams>,
  #[serde(rename = "source")]
  pub source: Value,
  /// The type of run data source. Always `completions`.
  #[serde(rename = "type")]
  pub _type: String
}
