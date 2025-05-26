/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateEvalJsonlRunDataSource : A JsonlRunDataSource object with that specifies a JSONL file that matches the eval

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvalJsonlRunDataSource {
    #[serde(rename = "source")]
    pub source: Value,
    /// The type of data source. Always `jsonl`.
    #[serde(rename = "type")]
    pub _type: String,
}
