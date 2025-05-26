/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub WebSearch : This tool searches the web for relevant results to use in a response. Learn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat).

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearch {
  #[serde(rename = "search_context_size")]
  pub search_context_size: Option<crate::models::WebSearchContextSize>,
  #[serde(rename = "user_location")]
  pub user_location: Option<crate::models::WebSearchUserLocation>
}
