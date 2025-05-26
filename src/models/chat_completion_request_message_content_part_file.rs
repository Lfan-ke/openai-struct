/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestMessageContentPartFile : Learn about [file inputs](/docs/guides/text) for text generation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartFile {
    #[serde(rename = "file")]
    pub file: crate::models::ChatCompletionRequestMessageContentPartFileFile,
    /// The type of the content part. Always `file`.
    #[serde(rename = "type")]
    pub _type: String,
}
