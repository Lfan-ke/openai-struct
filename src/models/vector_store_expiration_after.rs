/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub VectorStoreExpirationAfter : The expiration policy for a vector store.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorStoreExpirationAfter {
  /// Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`.
  #[serde(rename = "anchor")]
  pub anchor: String,
  /// The number of days after the anchor time that the vector store will expire.
  #[serde(rename = "days")]
  pub days: i32
}
