/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ComputerUsePreviewTool : A tool that controls a virtual computer. Learn more about the [computer tool](https://platform.openai.com/docs/guides/tools-computer-use).

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerUsePreviewTool {
  /// The height of the computer display.
  #[serde(rename = "display_height")]
  pub display_height: i32,
  /// The width of the computer display.
  #[serde(rename = "display_width")]
  pub display_width: i32,
  /// The type of computer environment to control.
  #[serde(rename = "environment")]
  pub environment: String,
  /// The type of the computer use tool. Always `computer_use_preview`.
  #[serde(rename = "type")]
  pub _type: String
}
