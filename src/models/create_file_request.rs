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
pub struct CreateFileRequest {
  /// The File object (not file name) to be uploaded. 
  #[serde(rename = "file")]
  pub file: Vec<u8>,
  /// The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets 
  #[serde(rename = "purpose")]
  pub purpose: String
}




