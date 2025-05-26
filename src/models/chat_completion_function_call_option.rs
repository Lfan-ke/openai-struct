/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionFunctionCallOption : Specifying a particular function via `{\"name\": \"my_function\"}` forces the model to call that function.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionFunctionCallOption {
    /// The name of the function to call.
    #[serde(rename = "name")]
    pub name: String,
}
