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
pub struct ChatCompletionRequestFunctionMessage {
    /// The contents of the function message.
    #[serde(rename = "content")]
    pub content: String,
    /// The name of the function to call.
    #[serde(rename = "name")]
    pub name: String,
    /// The role of the messages author, in this case `function`.
    #[serde(rename = "role")]
    pub role: String,
}
