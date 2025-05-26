/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub InputMessage : A message input to the model with a role indicating instruction following hierarchy. Instructions given with the `developer` or `system` role take precedence over instructions given with the `user` role.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessage {
  #[serde(rename = "content")]
  pub content: crate::models::InputMessageContentList,
  /// The role of the message input. One of `user`, `system`, or `developer`.
  #[serde(rename = "role")]
  pub role: String,
  /// The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
  #[serde(rename = "status")]
  pub status: Option<String>,
  /// The type of the message input. Always set to `message`.
  #[serde(rename = "type")]
  pub _type: Option<String>
}
