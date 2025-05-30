/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteProjects {
    /// Project's public ID
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Project membership role
    #[serde(rename = "role")]
    pub role: Option<String>,
}
