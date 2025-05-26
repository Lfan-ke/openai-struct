/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFormatJsonSchema : JSON Schema response format. Used to generate structured JSON responses. Learn more about [Structured Outputs](/docs/guides/structured-outputs).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFormatJsonSchema {
    #[serde(rename = "json_schema")]
    pub json_schema: crate::models::JsonSchema,
    /// The type of response format being defined. Always `json_schema`.
    #[serde(rename = "type")]
    pub _type: String,
}
