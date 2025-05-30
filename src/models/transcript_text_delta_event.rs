/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub TranscriptTextDeltaEvent : Emitted when there is an additional text delta. This is also the first event emitted when the transcription starts. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`.

#[allow(unused_imports)]
use serde_json::Value;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TranscriptTextDeltaEvent {
//   /// The text delta that was additionally transcribed.
//   #[serde(rename = "delta")]
//   pub delta: String,
//   /// The log probabilities of the delta. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.
//   #[serde(rename = "logprobs")]
//   pub logprobs: Option<Vec<crate::models::TranscriptTextDeltaEventLogprobs>>,
//   /// The type of the event. Always `transcript.text.delta`.
//   #[serde(rename = "type")]
//   pub _type: String
// }
