/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FileSearchRankingOptions : The ranking options for the file search. If not specified, the file search tool will use the `auto` ranker and a score_threshold of 0.  See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSearchRankingOptions {
    #[serde(rename = "ranker")]
    pub ranker: Option<crate::models::FileSearchRanker>,
    /// The score threshold for the file search. All values must be a floating point number between 0 and 1.
    #[serde(rename = "score_threshold")]
    pub score_threshold: f32,
}
