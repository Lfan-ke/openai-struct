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
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteUploadRequest {
  /// The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.
  #[serde(rename = "md5")]
  pub md5: Option<String>,
  /// The ordered list of Part IDs.
  #[serde(rename = "part_ids")]
  pub part_ids: Vec<String>
}
