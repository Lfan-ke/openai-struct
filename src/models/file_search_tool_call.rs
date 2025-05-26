/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FileSearchToolCall : The results of a file search tool call. See the  [file search guide](/docs/guides/tools-file-search) for more information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSearchToolCall {
    /// The unique ID of the file search tool call.
    #[serde(rename = "id")]
    pub id: String,
    /// The queries used to search for files.
    #[serde(rename = "queries")]
    pub queries: Vec<String>,
    /// The results of the file search tool call.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::models::FileSearchToolCallResults>>,
    /// The status of the file search tool call. One of `in_progress`,  `searching`, `incomplete` or `failed`,
    #[serde(rename = "status")]
    pub status: String,
    /// The type of the file search tool call. Always `file_search_call`.
    #[serde(rename = "type")]
    pub _type: String,
}
