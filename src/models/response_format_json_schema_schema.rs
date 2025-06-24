/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ResponseFormatJsonSchemaSchema : The schema for the response format, described as a JSON Schema object. Learn how to build JSON schemas [here](pub https://json-schema.org/).

#[allow(unused_imports)]
use serde_json::Value;

/// # on openapi.yaml
///
/// ```yaml
/// ResponseFormatJsonSchemaSchema:
///   type: object
///   title: JSON schema
///   description: |
///     The schema for the response format, described as a JSON Schema object.
///     Learn how to build JSON schemas [here](https://json-schema.org/).
///   additionalProperties: true
/// ```
pub type ResponseFormatJsonSchemaSchema = serde_json::Value;
