/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CostsResult : The aggregated costs details of the specific time bucket.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CostsResult {
    #[serde(rename = "amount")]
    pub amount: Option<crate::models::CostsResultAmount>,
    /// When `group_by=line_item`, this field provides the line item of the grouped costs result.
    #[serde(rename = "line_item")]
    pub line_item: Option<String>,
    #[serde(rename = "object")]
    pub object: String,
    /// When `group_by=project_id`, this field provides the project ID of the grouped costs result.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
}
