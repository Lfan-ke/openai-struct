/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub StopConfiguration : Not supported with latest reasoning models `o3` and `o4-mini`.  Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopConfiguration {}
