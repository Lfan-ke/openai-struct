/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub StaticChunkingStrategyRequestParam : Customize your own chunking strategy by setting chunk size and chunk overlap.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticChunkingStrategyRequestParam {
  #[serde(rename = "static")]
  pub _static: crate::models::StaticChunkingStrategy,
  /// Always `static`.
  #[serde(rename = "type")]
  pub _type: String
}
