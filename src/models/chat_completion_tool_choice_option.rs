/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionToolChoiceOption : Controls which (if any) tool is called by the model. `none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools. Specifying a particular tool via `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.  `none` is the default when no tools are present. `auto` is the default if tools are present.

#[allow(unused_imports)]
use serde_json::Value;

/// # on openapi.yaml
/// 
/// ```yaml
/// ChatCompletionToolChoiceOption:
///   description: >
///     Controls which (if any) tool is called by the model.
/// 
///     `none` means the model will not call any tool and instead generates a
///     message.
/// 
///     `auto` means the model can pick between generating a message or calling
///     one or more tools.
/// 
///     `required` means the model must call one or more tools.
/// 
///     Specifying a particular tool via `{"type": "function", "function":
///     {"name": "my_function"}}` forces the model to call that tool.
/// 
/// 
///     `none` is the default when no tools are present. `auto` is the default
///     if tools are present.
///   oneOf:
///     - type: string
///       description: >
///         `none` means the model will not call any tool and instead generates
///         a message. `auto` means the model can pick between generating a
///         message or calling one or more tools. `required` means the model
///         must call one or more tools.
///       enum:
///         - none
///         - auto
///         - required
///     - $ref: "#/components/schemas/ChatCompletionNamedToolChoice"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionToolChoiceOption {
    // todo: oneOf : string、ChatCompletionNamedToolChoice
}
