/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestMessageContentPartImage : Learn about [image inputs](/docs/guides/vision).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartImage {
    #[serde(rename = "image_url")]
    pub image_url: crate::models::ChatCompletionRequestMessageContentPartImageImageUrl,
    /// The type of the content part.
    #[serde(rename = "type")]
    pub _type: String,
}
