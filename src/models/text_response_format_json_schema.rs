/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub TextResponseFormatJsonSchema : JSON Schema response format. Used to generate structured JSON responses. Learn more about [Structured Outputs](/docs/guides/structured-outputs).

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextResponseFormatJsonSchema {
  /// A description of what the response format is for, used by the model to determine how to respond in the format.
  #[serde(rename = "description")]
  pub description: Option<String>,
  /// The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "schema")]
  pub schema: crate::models::ResponseFormatJsonSchemaSchema,
  /// Whether to enable strict schema adherence when generating the output. If set to true, the model will always follow the exact schema defined in the `schema` field. Only a subset of JSON Schema is supported when `strict` is `true`. To learn more, read the [Structured Outputs guide](/docs/guides/structured-outputs).
  #[serde(rename = "strict")]
  pub strict: Option<bool>,
  /// The type of response format being defined. Always `json_schema`.
  #[serde(rename = "type")]
  pub _type: String
}
