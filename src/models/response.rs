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
pub struct Response {
    /// Inserts a system (or developer) message as the first item in the model's context.  When using along with `previous_response_id`, the instructions from a previous response will not be carried over to the next response. This makes it simple to swap out system (or developer) messages in new responses.
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    /// An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).
    #[serde(rename = "max_output_tokens")]
    pub max_output_tokens: Option<i32>,
    /// Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI offers a wide range of models with different capabilities, performance characteristics, and price points. Refer to the [model guide](/docs/models) to browse and compare available models.
    ///
    /// 可参考：
    ///
    /// ``` use crate::models::ModelIdsResponses; ```
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// The unique ID of the previous response to the model. Use this to create multi-turn conversations. Learn more about  [conversation state](/docs/guides/conversation-state).
    #[serde(rename = "previous_response_id")]
    pub previous_response_id: Option<String>,
    #[serde(rename = "reasoning")]
    pub reasoning: Option<crate::models::Reasoning>,
    #[serde(rename = "text")]
    pub text: Option<crate::models::ResponsePropertiesText>,
    /// How the model should select which tool (or tools) to use when generating a response. See the `tools` parameter to see how to specify which tools the model can call.
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<Value>,
    /// An array of tools the model may call while generating a response. You  can specify which tool to use by setting the `tool_choice` parameter.  The two categories of tools you can provide the model are:  - **Built-in tools**: Tools that are provided by OpenAI that extend the   model's capabilities, like [web search](/docs/guides/tools-web-search)   or [file search](/docs/guides/tools-file-search). Learn more about   [built-in tools](/docs/guides/tools). - **Function calls (custom tools)**: Functions that are defined by you,   enabling the model to call your own code. Learn more about   [function calling](/docs/guides/function-calling).
    #[serde(rename = "tools")]
    pub tools: Option<Vec<crate::models::Tool>>,
    /// The truncation strategy to use for the model response. - `auto`: If the context of this response and previous ones exceeds   the model's context window size, the model will truncate the    response to fit the context window by dropping input items in the   middle of the conversation.  - `disabled` (default): If a model response will exceed the context window    size for a model, the request will fail with a 400 error.
    #[serde(rename = "truncation")]
    pub truncation: Option<String>,
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
    /// Unix timestamp (in seconds) of when this Response was created.
    #[serde(rename = "created_at")]
    pub created_at: f32,
    #[serde(rename = "error")]
    pub error: crate::models::ResponseError,
    /// Unique identifier for this Response.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "incomplete_details")]
    pub incomplete_details: crate::models::ResponseIncompleteDetails,
    /// The object type of this resource - always set to `response`.
    #[serde(rename = "object")]
    pub object: String,
    /// An array of content items generated by the model.  - The length and order of items in the `output` array is dependent   on the model's response. - Rather than accessing the first item in the `output` array and    assuming it's an `assistant` message with the content generated by   the model, you might consider using the `output_text` property where   supported in SDKs.
    #[serde(rename = "output")]
    pub output: Vec<crate::models::OutputItem>,
    /// SDK-only convenience property that contains the aggregated text output  from all `output_text` items in the `output` array, if any are present.  Supported in the Python and JavaScript SDKs.
    #[serde(rename = "output_text")]
    pub output_text: Option<String>,
    /// Whether to allow the model to run tool calls in parallel.
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: bool,
    /// The status of the response generation. One of `completed`, `failed`,  `in_progress`, or `incomplete`.
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "usage")]
    pub usage: Option<crate::models::ResponseUsage>,
}
