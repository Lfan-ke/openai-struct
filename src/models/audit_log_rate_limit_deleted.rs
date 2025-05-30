/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogRateLimitDeleted : The details for events with this `type`.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogRateLimitDeleted {
    /// The rate limit ID
    #[serde(rename = "id")]
    pub id: Option<String>,
}
