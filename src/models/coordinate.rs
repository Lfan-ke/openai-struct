/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub Coordinate : An x/y coordinate pair, e.g. `{ pub x: 100, pub y: 200 }`.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    /// The x-coordinate.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y-coordinate.
    #[serde(rename = "y")]
    pub y: i32,
}
