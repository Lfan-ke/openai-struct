/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub BatchRequestInput : The per-line object of the batch input file

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequestInput {
  /// A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch.
  #[serde(rename = "custom_id")]
  pub custom_id: Option<String>,
  /// The HTTP method to be used for the request. Currently only `POST` is supported.
  #[serde(rename = "method")]
  pub method: Option<String>,
  /// The OpenAI API relative URL to be used for the request. Currently `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported.
  #[serde(rename = "url")]
  pub url: Option<String>
}
