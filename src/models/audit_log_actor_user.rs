/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogActorUser : The user who performed the audit logged action.

#[allow(unused_imports)]
use serde_json::Value;

/// # on openapi.yaml
///
/// ```yaml
/// AuditLogActorUser:
///   type: object
///   description: The user who performed the audit logged action.
///   properties:
///     id:
///       type: string
///       description: The user id.
///     email:
///       type: string
///       description: The user email.
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorUser {
    /// The user email.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The user id.
    #[serde(rename = "id")]
    pub id: Option<String>,
}
