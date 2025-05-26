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
pub struct ProjectServiceAccountApiKey {
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.project.service_account.api_key`
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "value")]
    pub value: String,
}
