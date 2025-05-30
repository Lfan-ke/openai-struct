/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateTranscriptionResponseVerboseJson : Represents a verbose json transcription response returned by model, based on the provided input.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseVerboseJson {
    /// The duration of the input audio.
    #[serde(rename = "duration")]
    pub duration: f32,
    /// The language of the input audio.
    #[serde(rename = "language")]
    pub language: String,
    /// Segments of the transcribed text and their corresponding details.
    #[serde(rename = "segments")]
    pub segments: Option<Vec<crate::models::TranscriptionSegment>>,
    /// The transcribed text.
    #[serde(rename = "text")]
    pub text: String,
    /// Extracted words and their corresponding timestamps.
    #[serde(rename = "words")]
    pub words: Option<Vec<crate::models::TranscriptionWord>>,
}
