/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestAssistantMessageAudio : Data about a previous audio response from the model.  [Learn more](/docs/guides/audio).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageAudio {
    /// Unique identifier for a previous audio response from the model.
    #[serde(rename = "id")]
    pub id: String,
}
