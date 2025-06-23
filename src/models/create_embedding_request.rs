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

/// todo：长改看
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingRequest {
    /// The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.
    #[serde(rename = "dimensions")]
    pub dimensions: Option<i32>,
    /// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/).
    #[serde(rename = "encoding_format")]
    pub encoding_format: Option<String>,
    /// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.
    #[serde(rename = "input")]
    pub input: Value,
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.
    #[serde(rename = "model")]
    pub model: Value,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).
    #[serde(rename = "user")]
    pub user: Option<String>,
}
