/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub EvalStoredCompletionsDataSourceConfig : A StoredCompletionsDataSourceConfig which specifies the metadata property of your stored completions query. This is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc. The schema returned by this data source config is used to defined what variables are available in your evals. `item` and `sample` are both defined when using this data source config.

#[allow(unused_imports)]
use  serde_json::Value;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct EvalStoredCompletionsDataSourceConfig {
//   #[serde(rename = "metadata")]
//   pub metadata: Option<crate::models::Metadata>,
//   /// The json schema for the run data source items. Learn how to build JSON schemas [here](pub https://json-schema.org/).
//   #[serde(rename = "schema")]
//   pub schema: ::std::collections::HashMap<String, pub crate::models::Object>,
//   /// The type of data source. Always `stored_completions`.
//   #[serde(rename = "type")]
//   pub _type: String
// }
