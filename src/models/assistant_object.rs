/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AssistantObject : Represents an `assistant` that can call the model and use tools.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantObject {
    /// The Unix timestamp (in seconds) for when the assistant was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The description of the assistant. The maximum length is 512 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    #[serde(rename = "instructions")]
    pub instructions: String,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.
    #[serde(rename = "model")]
    pub model: String,
    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `assistant`.
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::models::AssistantsApiResponseFormatOption>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<crate::models::AssistantObjectToolResources>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.
    #[serde(rename = "tools")]
    pub tools: Vec<Value>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or temperature but not both.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
}
