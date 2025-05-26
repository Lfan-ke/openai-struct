/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub BatchRequestOutputError : For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequestOutputError {
  /// A machine-readable error code.
  #[serde(rename = "code")]
  pub code: Option<String>,
  /// A human-readable error message.
  #[serde(rename = "message")]
  pub message: Option<String>
}
