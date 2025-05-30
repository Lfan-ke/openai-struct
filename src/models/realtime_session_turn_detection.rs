/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RealtimeSessionTurnDetection : Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response. Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech. Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeSessionTurnDetection {
    /// Whether or not to automatically generate a response when a VAD stop event occurs.
    #[serde(rename = "create_response")]
    pub create_response: Option<bool>,
    /// Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.
    #[serde(rename = "eagerness")]
    pub eagerness: Option<String>,
    /// Whether or not to automatically interrupt any ongoing response with output to the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
    #[serde(rename = "interrupt_response")]
    pub interrupt_response: Option<bool>,
    /// Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in  milliseconds). Defaults to 300ms.
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<i32>,
    /// Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults  to 500ms. With shorter values the model will respond more quickly,  but may jump in on short pauses from the user.
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<i32>,
    /// Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A  higher threshold will require louder audio to activate the model, and  thus might perform better in noisy environments.
    #[serde(rename = "threshold")]
    pub threshold: Option<f32>,
    /// Type of turn detection.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
