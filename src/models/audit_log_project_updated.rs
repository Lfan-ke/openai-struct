/* 
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 * 
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogProjectUpdated : The details for events with this `type`.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectUpdated {
  #[serde(rename = "changes_requested")]
  pub changes_requested: Option<crate::models::AuditLogProjectUpdatedChangesRequested>,
  /// The project ID.
  #[serde(rename = "id")]
  pub id: Option<String>
}
