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

use crate::{
    ChatCompletionRequestMessageContentPartAudio, ChatCompletionRequestMessageContentPartFile,
    ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
};

/// # on openapi.yaml
///
/// ```yaml
/// ChatCompletionRequestUserMessageContentPart:
///   oneOf:
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartText"
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartImage"
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartAudio"
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartFile"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum ChatCompletionRequestUserMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Image(ChatCompletionRequestMessageContentPartImage),
    Audio(ChatCompletionRequestMessageContentPartAudio),
    File(ChatCompletionRequestMessageContentPartFile),
}
