/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FilePath : A path to a file.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePath {
    /// The ID of the file.
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// The index of the file in the list of files.
    #[serde(rename = "index")]
    pub index: i32,
    /// The type of the file path. Always `file_path`.
    #[serde(rename = "type")]
    pub _type: String,
}
