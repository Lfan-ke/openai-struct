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
pub struct ChatCompletionMessageToolCallChunk {
    #[serde(rename = "function")]
    pub function: Option<crate::models::ChatCompletionMessageToolCallChunkFunction>,
    /// The ID of the tool call.
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "index")]
    pub index: i32,
    /// The type of the tool. Currently, only `function` is supported.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
