/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ComputerScreenshotImage : A computer screenshot image used with the computer use tool.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerScreenshotImage {
    /// The identifier of an uploaded file that contains the screenshot.
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    /// The URL of the screenshot image.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Specifies the event type. For a computer screenshot, this property is  always set to `computer_screenshot`.
    #[serde(rename = "type")]
    pub _type: String,
}
