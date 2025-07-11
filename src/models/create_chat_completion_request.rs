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
pub struct CreateChatCompletionRequest {
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    #[serde(rename = "service_tier")]
    pub service_tier: Option<crate::models::ServiceTier>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. We generally recommend altering this or `top_p` but not both.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or `temperature` but not both.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).
    #[serde(rename = "user")]
    pub user: Option<String>,
    #[serde(rename = "audio")]
    pub audio: Option<crate::models::CreateChatCompletionRequestAudio>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f32>,
    /// Deprecated in favor of `tool_choice`.  Controls which (if any) function is called by the model.  `none` means the model will not call a function and instead generates a message.  `auto` means the model can pick between generating a message or calling a function.  Specifying a particular function via `{\"name\": \"my_function\"}` forces the model to call that function.  `none` is the default when no functions are present. `auto` is the default if functions are present.
    #[serde(rename = "function_call")]
    pub function_call: Option<CreateChatCompletionRequestFunctionCall>,
    /// Deprecated in favor of `tools`.  A list of functions the model may generate JSON inputs for.
    #[serde(rename = "functions")]
    pub functions: Option<Vec<crate::models::ChatCompletionFunctions>>,
    /// Modify the likelihood of specified tokens appearing in the completion.  Accepts a JSON object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(rename = "logit_bias")]
    pub logit_bias: Option<::std::collections::HashMap<String, i32>>,
    /// Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the `content` of `message`.
    #[serde(rename = "logprobs")]
    pub logprobs: Option<bool>,
    /// An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<i32>,
    /// The maximum number of [tokens](/tokenizer) that can be generated in the chat completion. This value can be used to control [costs](https://openai.com/api/pricing/) for text generated via API.  This value is now deprecated in favor of `max_completion_tokens`, and is not compatible with [o-series models](/docs/guides/reasoning).
    #[serde(rename = "max_tokens")]
    pub max_tokens: Option<i32>,
    /// A list of messages comprising the conversation so far. Depending on the [model](/docs/models) you use, different message types (modalities) are supported, like [text](/docs/guides/text-generation), [images](/docs/guides/vision), and [audio](/docs/guides/audio).
    #[serde(rename = "messages")]
    pub messages: Vec<crate::models::ChatCompletionRequestMessage>,
    #[serde(rename = "modalities")]
    pub modalities: Option<crate::models::ResponseModalities>,
    /// Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI offers a wide range of models with different capabilities, performance characteristics, and price points. Refer to the [model guide](/docs/models) to browse and compare available models.
    ///
    /// 可参考：
    ///
    /// ``` pub use crate::models::ModelIdsShared; ```
    #[serde(rename = "model")]
    pub model: String,
    /// How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs.
    #[serde(rename = "n")]
    pub n: Option<i32>,
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<crate::models::ParallelToolCalls>,
    /// Configuration for a [Predicted Output](/docs/guides/predicted-outputs), which can greatly improve response times when large parts of the model response are known ahead of time. This is most common when you are regenerating a file with only minor changes to most of the content.
    #[serde(rename = "prediction")]
    pub prediction: Option<crate::PredictionContent>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: Option<f32>,
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<crate::models::ReasoningEffort>,
    /// An object specifying the format that the model must output.  Setting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](/docs/guides/structured-outputs).  Setting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which ensures the message the model generates is valid JSON. Using `json_schema` is preferred for models that support it.
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateChatCompletionRequestResponseFormat>,
    /// This feature is in Beta. If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result. Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
    #[serde(rename = "seed")]
    pub seed: Option<i32>,
    #[serde(rename = "stop")]
    pub stop: Option<crate::models::StopConfiguration>,
    /// Whether or not to store the output of this chat completion request for  use in our [model distillation](/docs/guides/distillation) or [evals](/docs/guides/evals) products.
    #[serde(rename = "store")]
    pub store: Option<bool>,
    /// If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). See the [Streaming section below](/docs/api-reference/chat/streaming) for more information, along with the [streaming responses](/docs/guides/streaming-responses) guide for more information on how to handle the streaming events.
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    #[serde(rename = "stream_options")]
    pub stream_options: Option<crate::models::ChatCompletionStreamOptions>,
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<crate::models::ChatCompletionToolChoiceOption>,
    /// A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
    #[serde(rename = "tools")]
    pub tools: Option<Vec<crate::models::ChatCompletionTool>>,
    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability. `logprobs` must be set to `true` if this parameter is used.
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Option<i32>,
    #[serde(rename = "web_search_options")]
    pub web_search_options: Option<crate::models::WebSearch>,
}

impl Default for CreateChatCompletionRequest {
    fn default() -> CreateChatCompletionRequest {
        Self {
            metadata: None,
            service_tier: None,
            temperature: None,
            top_p: None,
            user: None,
            audio: None,
            frequency_penalty: None,
            function_call: None,
            functions: None,
            logit_bias: None,
            logprobs: None,
            max_completion_tokens: None,
            max_tokens: None,
            messages: vec![],
            modalities: None,
            model: serde_json::to_string(&crate::ModelIdsShared::Gpt4_1).unwrap(),
            n: None,
            parallel_tool_calls: None,
            prediction: None,
            presence_penalty: None,
            reasoning_effort: None,
            response_format: None,
            seed: None,
            stop: None,
            store: None,
            stream: None,
            stream_options: None,
            tool_choice: None,
            tools: None,
            top_logprobs: None,
            web_search_options: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestFunctionCall {
    Text(String),
    FunctionCallOption(crate::ChatCompletionFunctionCallOption),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestResponseFormat {
    Text(crate::ResponseFormatText),
    JsonSchema(crate::ResponseFormatJsonSchema),
    JsonObject(crate::ResponseFormatJsonObject),
}
