/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionList : An object representing a list of Chat Completions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionList {
    /// An array of chat completion objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::CreateChatCompletionResponse>,
    /// The identifier of the first chat completion in the data array.
    #[serde(rename = "first_id")]
    pub first_id: String,
    /// Indicates whether there are more Chat Completions available.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// The identifier of the last chat completion in the data array.
    #[serde(rename = "last_id")]
    pub last_id: String,
    /// The type of this object. It is always set to \"list\".
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
}

fn default_object() -> String {
    "list".into()
}
