/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub EvalCustomDataSourceConfig : A CustomDataSourceConfig which specifies the schema of your `item` and optionally `sample` namespaces. The response schema defines the shape of the data that will be: - Used to define your testing criteria and - What data is required when creating a run

#[allow(unused_imports)]
use  serde_json::Value;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct EvalCustomDataSourceConfig {
//   /// The json schema for the run data source items. Learn how to build JSON schemas [here](pub https://json-schema.org/).
//   #[serde(rename = "schema")]
//   pub schema: ::std::collections::HashMap<String, pub crate::models::Object>,
//   /// The type of data source. Always `custom`.
//   #[serde(rename = "type")]
//   pub _type: String
// }
