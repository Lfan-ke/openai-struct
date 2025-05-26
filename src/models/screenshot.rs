/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub Screenshot : A screenshot action.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Screenshot {
    /// Specifies the event type. For a screenshot action, this property is  always set to `screenshot`.
    #[serde(rename = "type")]
    pub _type: String,
}
