/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeSessionCreateResponse : A new Realtime session configuration, with an ephermeral key. Default TTL for keys is one minute.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponse {
    #[serde(rename = "client_secret")]
    pub client_secret: crate::models::RealtimeSessionCreateResponseClientSecret,
    /// The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<String>,
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription:
        Option<crate::models::RealtimeSessionCreateResponseInputAudioTranscription>,
    /// The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good  responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion  into your voice\", \"laugh frequently\"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the  desired behavior.  Note that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    /// Maximum number of output tokens for a single assistant response, inclusive of tool calls. Provide an integer between 1 and 4096 to limit output tokens, or `inf` for the maximum available tokens for a given model. Defaults to `inf`.
    #[serde(rename = "max_response_output_tokens")]
    pub max_response_output_tokens: Option<Value>,
    /// The set of modalities the model can respond with. To disable audio, set this to [\"text\"].
    #[serde(rename = "modalities")]
    pub modalities: Option<Value>,
    /// The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(rename = "output_audio_format")]
    pub output_audio_format: Option<String>,
    /// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    /// How the model chooses tools. Options are `auto`, `none`, `required`, or  specify a function.
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    /// Tools (functions) available to the model.
    #[serde(rename = "tools")]
    pub tools: Option<Vec<crate::models::RealtimeResponseCreateParamsTools>>,
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<crate::models::RealtimeSessionCreateResponseTurnDetection>,
    /// The voice the model uses to respond. Voice cannot be changed during the  session once the model has responded with audio at least once. Current  voice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`,  `shimmer` and `verse`.
    #[serde(rename = "voice")]
    pub voice: Option<crate::models::VoiceIdsShared>,
}
