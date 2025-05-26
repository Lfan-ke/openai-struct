/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ChatCompletionRequestMessageContentPartAudio : Learn about [audio inputs](/docs/guides/audio).

#[allow(unused_imports)]
use  serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartAudio {
  #[serde(rename = "input_audio")]
  pub input_audio: crate::models::ChatCompletionRequestMessageContentPartAudioInputAudio,
  /// The type of the content part. Always `input_audio`.
  #[serde(rename = "type")]
  pub _type: String
}
