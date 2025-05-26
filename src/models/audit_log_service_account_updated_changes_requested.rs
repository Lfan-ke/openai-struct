/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogServiceAccountUpdatedChangesRequested : The payload used to updated the service account.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogServiceAccountUpdatedChangesRequested {
  /// The role of the service account. Is either `owner` or `member`.
  #[serde(rename = "role")]
  pub role: Option<String>
}
