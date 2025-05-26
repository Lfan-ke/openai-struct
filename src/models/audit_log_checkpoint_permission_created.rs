/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogCheckpointPermissionCreated : The project and fine-tuned model checkpoint that the checkpoint permission was created for.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogCheckpointPermissionCreated {
    #[serde(rename = "data")]
    pub data: Option<crate::models::AuditLogCheckpointPermissionCreatedData>,
    /// The ID of the checkpoint permission.
    #[serde(rename = "id")]
    pub id: Option<String>,
}
