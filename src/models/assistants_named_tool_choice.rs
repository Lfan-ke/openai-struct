/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AssistantsNamedToolChoice : Specifies a tool the model should use. Use to force the model to call a specific tool.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantsNamedToolChoice {
  #[serde(rename = "function")]
  pub function: Option<crate::models::AssistantsNamedToolChoiceFunction>,
  /// The type of the tool. If type is `function`, the function name must be set
  #[serde(rename = "type")]
  pub _type: String
}
