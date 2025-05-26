/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub Embedding : Represents an embedding vector returned by embedding endpoint.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedding {
  /// The embedding vector, which is a list of floats. The length of vector depends on the model as listed in the [embedding guide](/docs/guides/embeddings).
  #[serde(rename = "embedding")]
  pub embedding: Vec<f32>,
  /// The index of the embedding in the list of embeddings.
  #[serde(rename = "index")]
  pub index: i32,
  /// The object type, which is always \"embedding\".
  #[serde(rename = "object")]
  pub object: String
}
