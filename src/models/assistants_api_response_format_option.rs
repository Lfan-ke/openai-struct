/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub AssistantsApiResponseFormatOption : Specifies the format that the model must output. Compatible with [GPT-4o](/docs/models#gpt-4o), [GPT-4 Turbo](/docs/models#gpt-4-turbo-and-gpt-4), and all GPT-3.5 Turbo models since `gpt-3.5-turbo-1106`.  Setting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](/docs/guides/structured-outputs).  Setting to `{ \"type\": \"json_object\" }` enables JSON mode, which ensures the message the model generates is valid JSON.  **pub Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly \"stuck\" request. Also note that the message content may be partially cut off if `finish_reason=\"length\"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.

#[allow(unused_imports)]
use serde_json::Value;
use crate::{ResponseFormatJsonObject, ResponseFormatJsonSchema, ResponseFormatText};

/// # on openapi.yaml
/// 
/// ```yaml
/// AssistantsApiResponseFormatOption:
///   description: >
///     Specifies the format that the model must output. Compatible with
///     [GPT-4o](/docs/models#gpt-4o), [GPT-4
///     Turbo](/docs/models#gpt-4-turbo-and-gpt-4), and all GPT-3.5 Turbo models
///     since `gpt-3.5-turbo-1106`.
/// 
/// 
///     Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
///     Structured Outputs which ensures the model will match your supplied JSON
///     schema. Learn more in the [Structured Outputs
///     guide](/docs/guides/structured-outputs).
/// 
/// 
///     Setting to `{ "type": "json_object" }` enables JSON mode, which ensures
///     the message the model generates is valid JSON.
/// 
/// 
///     **Important:** when using JSON mode, you **must** also instruct the
///     model to produce JSON yourself via a system or user message. Without
///     this, the model may generate an unending stream of whitespace until the
///     generation reaches the token limit, resulting in a long-running and
///     seemingly "stuck" request. Also note that the message content may be
///     partially cut off if `finish_reason="length"`, which indicates the
///     generation exceeded `max_tokens` or the conversation exceeded the max
///     context length.
///   oneOf:
///     - type: string
///       description: |
///         `auto` is the default value
///       enum:
///         - auto
///       x-stainless-const: true
///     - $ref: "#/components/schemas/ResponseFormatText"
///     - $ref: "#/components/schemas/ResponseFormatJsonObject"
///     - $ref: "#/components/schemas/ResponseFormatJsonSchema"
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssistantsApiResponseFormatOption {
    #[serde(rename = "text")]
    Text(ResponseFormatText),
    #[serde(rename = "json_object")]
    Object(ResponseFormatJsonObject),
    #[serde(rename = "json_schema")]
    Schema(ResponseFormatJsonSchema),
}
