/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseUsageOutputTokensDetails : A detailed breakdown of the output tokens.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUsageOutputTokensDetails {
    /// The number of reasoning tokens.
    #[serde(rename = "reasoning_tokens")]
    pub reasoning_tokens: i32,
}
