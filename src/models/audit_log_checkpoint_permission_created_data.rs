/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AuditLogCheckpointPermissionCreatedData : The payload used to create the checkpoint permission.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogCheckpointPermissionCreatedData {
  /// The ID of the fine-tuned model checkpoint.
  #[serde(rename = "fine_tuned_model_checkpoint")]
  pub fine_tuned_model_checkpoint: Option<String>,
  /// The ID of the project that the checkpoint permission was created for.
  #[serde(rename = "project_id")]
  pub project_id: Option<String>
}
