/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFormatJsonObject : JSON object response format. An older method of generating JSON responses. Using `json_schema` is recommended for models that support it. Note that the model will not generate JSON without a system or user message instructing it to do so.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFormatJsonObject {
    /// The type of response format being defined. Always `json_object`.
    #[serde(rename = "type")]
    pub _type: String,
}
