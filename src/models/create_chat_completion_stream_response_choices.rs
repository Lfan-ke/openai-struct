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
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatCompletionStreamResponseChoices {
    #[serde(rename = "delta")]
    pub delta: crate::models::ChatCompletionStreamResponseDelta,
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    /// The index of the choice in the list of choices.
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "logprobs")]
    pub logprobs: Option<crate::models::CreateChatCompletionResponseLogprobs>,
}

impl Default for CreateChatCompletionStreamResponseChoices {
    fn default() -> Self {
        Self {
            delta: Default::default(),
            finish_reason: "".to_string(),
            index: 0,
            logprobs: None,
        }
    }
}
