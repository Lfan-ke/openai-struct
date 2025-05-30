/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub PredictionContent : Static predicted output content, such as the content of a text file that is being regenerated.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionContent {
    /// The content that should be matched when generating a model response. If generated tokens would match this content, the entire model response can be returned much more quickly.
    #[serde(rename = "content")]
    pub content: Value,
    /// The type of the predicted content you want to provide. This type is currently always `content`.
    #[serde(rename = "type")]
    pub _type: String,
}
