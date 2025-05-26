/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CompoundFilter : Combine multiple filters using `and` or `or`.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompoundFilter {
  /// Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`.
  #[serde(rename = "filters")]
  pub filters: Vec<Value>,
  /// Type of operation: `and` or `or`.
  #[serde(rename = "type")]
  pub _type: String
}
