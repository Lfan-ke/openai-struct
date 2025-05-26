/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ProjectUser : Represents an individual user in a project.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectUser {
    /// The Unix timestamp (in seconds) of when the project was added.
    #[serde(rename = "added_at")]
    pub added_at: i32,
    /// The email address of the user
    #[serde(rename = "email")]
    pub email: String,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the user
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `organization.project.user`
    #[serde(rename = "object")]
    pub object: String,
    /// `owner` or `member`
    #[serde(rename = "role")]
    pub role: String,
}
