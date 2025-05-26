/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogActorSession : The session in which the audit logged action was performed.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorSession {
  /// The IP address from which the action was performed.
  #[serde(rename = "ip_address")]
  pub ip_address: Option<String>,
  #[serde(rename = "user")]
  pub user: Option<crate::models::AuditLogActorUser>
}
