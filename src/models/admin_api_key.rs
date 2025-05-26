/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AdminApiKey : Represents an individual Admin API key in an org.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminApiKey {
  /// The Unix timestamp (in seconds) of when the API key was created
  #[serde(rename = "created_at")]
  pub created_at: i64,
  /// The identifier, which can be referenced in API endpoints
  #[serde(rename = "id")]
  pub id: String,
  /// The Unix timestamp (in seconds) of when the API key was last used
  #[serde(rename = "last_used_at")]
  pub last_used_at: i64,
  /// The name of the API key
  #[serde(rename = "name")]
  pub name: String,
  /// The object type, which is always `organization.admin_api_key`
  #[serde(rename = "object")]
  pub object: String,
  #[serde(rename = "owner")]
  pub owner: crate::models::AdminApiKeyOwner,
  /// The redacted value of the API key
  #[serde(rename = "redacted_value")]
  pub redacted_value: String,
  /// The value of the API key. Only shown on create.
  #[serde(rename = "value")]
  pub value: Option<String>
}
