/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunObjectIncompleteDetails : Details on why the run is incomplete. Will be `null` if the run is not incomplete.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunObjectIncompleteDetails {
  /// The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run.
  #[serde(rename = "reason")]
  pub reason: Option<String>
}
