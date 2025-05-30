/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogProjectCreatedData : The payload used to create the project.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectCreatedData {
    /// The project name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The title of the project as seen on the dashboard.
    #[serde(rename = "title")]
    pub title: Option<String>,
}
