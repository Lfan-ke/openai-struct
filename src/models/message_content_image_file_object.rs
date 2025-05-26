/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageContentImageFileObject : References an image [File](/docs/api-reference/files) in the content of a message.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageContentImageFileObject {
  #[serde(rename = "image_file")]
  pub image_file: crate::models::MessageContentImageFileObjectImageFile,
  /// Always `image_file`.
  #[serde(rename = "type")]
  pub _type: String
}
