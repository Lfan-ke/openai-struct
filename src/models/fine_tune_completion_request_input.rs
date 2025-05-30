/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FineTuneCompletionRequestInput : The per-line training example of a fine-tuning input file for completions models

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuneCompletionRequestInput {
    /// The desired completion for this training example.
    #[serde(rename = "completion")]
    pub completion: Option<String>,
    /// The input prompt for this training example.
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
}
