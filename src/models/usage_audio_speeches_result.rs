/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub UsageAudioSpeechesResult : The aggregated audio speeches usage details of the specific time bucket.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageAudioSpeechesResult {
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    /// The number of characters processed.
    #[serde(rename = "characters")]
    pub characters: i32,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// The count of requests made to the model.
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: i32,
    #[serde(rename = "object")]
    pub object: String,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
}
