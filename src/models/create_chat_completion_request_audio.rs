/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateChatCompletionRequestAudio : Parameters for audio output. Required when audio output is requested with `modalities: [\"audio\"]`. [Learn more](/docs/guides/audio).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestAudio {
    /// Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`, `opus`, or `pcm16`.
    #[serde(rename = "format")]
    pub format: String,
    /// The voice the model uses to respond. Supported voices are  `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`, `sage`, and `shimmer`.
    #[serde(rename = "voice")]
    pub voice: crate::models::VoiceIdsShared,
}
