/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// Average logprob of the segment. If the value is lower than -1, consider the logprobs failed.
    #[serde(rename = "avg_logprob")]
    pub avg_logprob: f32,
    /// Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed.
    #[serde(rename = "compression_ratio")]
    pub compression_ratio: f32,
    /// End time of the segment in seconds.
    #[serde(rename = "end")]
    pub end: f32,
    /// Unique identifier of the segment.
    #[serde(rename = "id")]
    pub id: i32,
    /// Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent.
    #[serde(rename = "no_speech_prob")]
    pub no_speech_prob: f32,
    /// Seek offset of the segment.
    #[serde(rename = "seek")]
    pub seek: i32,
    /// Start time of the segment in seconds.
    #[serde(rename = "start")]
    pub start: f32,
    /// Temperature parameter used for generating the segment.
    #[serde(rename = "temperature")]
    pub temperature: f32,
    /// Text content of the segment.
    #[serde(rename = "text")]
    pub text: String,
    /// Array of token IDs for the text content.
    #[serde(rename = "tokens")]
    pub tokens: Vec<i32>,
}
