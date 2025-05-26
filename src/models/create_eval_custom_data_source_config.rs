/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateEvalCustomDataSourceConfig : A CustomDataSourceConfig object that defines the schema for the data source used for the evaluation runs. This schema is used to define the shape of the data that will be: - Used to define your testing criteria and - What data is required when creating a run

#[allow(unused_imports)]
use  serde_json::Value;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CreateEvalCustomDataSourceConfig {
//   /// Whether the eval should expect you to populate the sample namespace (ie, by generating responses off of your data source)
//   #[serde(rename = "include_sample_schema")]
//   pub include_sample_schema: Option<bool>,
//   /// The json schema for each row in the data source.
//   #[serde(rename = "item_schema")]
//   pub item_schema: ::std::collections::HashMap<String, pub crate::models::Object>,
//   /// The type of data source. Always `custom`.
//   #[serde(rename = "type")]
//   pub _type: String
// }
