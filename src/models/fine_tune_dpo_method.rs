/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FineTuneDpoMethod : Configuration for the DPO fine-tuning method.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuneDpoMethod {
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: Option<crate::models::FineTuneDpoMethodHyperparameters>,
}
