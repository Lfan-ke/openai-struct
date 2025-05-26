/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub WebSearchUserLocation : Approximate location parameters for the search.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearchUserLocation {
    #[serde(rename = "approximate")]
    pub approximate: crate::models::WebSearchLocation,
    /// The type of location approximation. Always `approximate`.
    #[serde(rename = "type")]
    pub _type: String,
}
