/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogUserAddedData : The payload used to add the user to the project.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogUserAddedData {
    /// The role of the user. Is either `owner` or `member`.
    #[serde(rename = "role")]
    pub role: Option<String>,
}
