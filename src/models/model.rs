/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub Model : Describes an OpenAI model offering that can be used with the API.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// The Unix timestamp (in seconds) when the model was created.
    #[serde(rename = "created")]
    pub created: i32,
    /// The model identifier, which can be referenced in the API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always \"model\".
    #[serde(rename = "object")]
    pub object: String,
    /// The organization that owns the model.
    #[serde(rename = "owned_by")]
    pub owned_by: String,
}
