/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub TextResponseFormatConfiguration : An object specifying the format that the model must output.  Configuring `{ \"type\": \"json_schema\" }` enables Structured Outputs,  which ensures the model will match your supplied JSON schema. Learn more in the  [Structured Outputs guide](/docs/guides/structured-outputs).  The default format is `{ \"type\": \"text\" }` with no additional options.  **Not recommended for gpt-4o and newer pub models:**  Setting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which ensures the message the model generates is valid JSON. Using `json_schema` is preferred for models that support it.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextResponseFormatConfiguration {}
