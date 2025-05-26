/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogActor : The actor who performed the audit logged action.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActor {
  #[serde(rename = "api_key")]
  pub api_key: Option<crate::models::AuditLogActorApiKey>,
  #[serde(rename = "session")]
  pub session: Option<crate::models::AuditLogActorSession>,
  /// The type of actor. Is either `session` or `api_key`.
  #[serde(rename = "type")]
  pub _type: Option<String>
}
