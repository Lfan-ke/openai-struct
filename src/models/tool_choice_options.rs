/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ToolChoiceOptions : Controls which (if any) tool is called by the model.  `none` means the model will not call any tool and instead generates a message.  `auto` means the model can pick between generating a message or calling one or more tools.  `required` means the model must call one or more tools.

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolChoiceOptions {
}


// TODO enum
// List of ToolChoiceOptions
//const (
//
//
//
//)
