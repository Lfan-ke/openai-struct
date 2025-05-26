/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub BatchRequestOutput : The per-line object of the batch output and error files

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequestOutput {
  /// A developer-provided per-request id that will be used to match outputs to inputs.
  #[serde(rename = "custom_id")]
  pub custom_id: Option<String>,
  #[serde(rename = "error")]
  pub error: Option<crate::models::BatchRequestOutputError>,
  #[serde(rename = "id")]
  pub id: Option<String>,
  #[serde(rename = "response")]
  pub response: Option<crate::models::BatchRequestOutputResponse>
}
