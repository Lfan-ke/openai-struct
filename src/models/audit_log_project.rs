/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogProject : The project that the action was scoped to. Absent for actions not scoped to projects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProject {
    /// The project ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The project title.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
