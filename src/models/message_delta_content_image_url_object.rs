/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageDeltaContentImageUrlObject : References an image URL in the content of a message.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaContentImageUrlObject {
    #[serde(rename = "image_url")]
    pub image_url: Option<crate::models::MessageDeltaContentImageUrlObjectImageUrl>,
    /// The index of the content part in the message.
    #[serde(rename = "index")]
    pub index: i32,
    /// Always `image_url`.
    #[serde(rename = "type")]
    pub _type: String,
}
