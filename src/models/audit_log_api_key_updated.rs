/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogApiKeyUpdated : The details for events with this `type`.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyUpdated {
  #[serde(rename = "changes_requested")]
  pub changes_requested: Option<crate::models::AuditLogApiKeyUpdatedChangesRequested>,
  /// The tracking ID of the API key.
  #[serde(rename = "id")]
  pub id: Option<String>
}
