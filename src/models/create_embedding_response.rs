/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingResponse {
    /// The list of embeddings generated by the model.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Embedding>,
    /// The name of the model used to generate the embedding.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always \"list\".
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "usage")]
    pub usage: crate::models::CreateEmbeddingResponseUsage,
}
