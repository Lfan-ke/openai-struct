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
pub struct ApproximateLocation {
    #[serde(rename = "city")]
    pub city: Option<Value>,
    #[serde(rename = "country")]
    pub country: Option<Value>,
    #[serde(rename = "region")]
    pub region: Option<Value>,
    #[serde(rename = "timezone")]
    pub timezone: Option<Value>,
    /// The type of location approximation. Always `approximate`.
    #[serde(rename = "type")]
    pub _type: String,
}
