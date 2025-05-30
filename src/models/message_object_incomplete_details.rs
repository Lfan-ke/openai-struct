/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageObjectIncompleteDetails : On an incomplete message, details about why the message is incomplete.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageObjectIncompleteDetails {
    /// The reason the message is incomplete.
    #[serde(rename = "reason")]
    pub reason: String,
}
