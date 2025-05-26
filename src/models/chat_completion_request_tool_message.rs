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
pub struct ChatCompletionRequestToolMessage {
  /// The contents of the tool message.
  #[serde(rename = "content")]
  pub content: Value,
  /// The role of the messages author, in this case `tool`.
  #[serde(rename = "role")]
  pub role: String,
  /// Tool call that this message is responding to.
  #[serde(rename = "tool_call_id")]
  pub tool_call_id: String
}
