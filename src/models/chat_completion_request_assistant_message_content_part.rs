/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

use crate::{
    ChatCompletionRequestMessageContentPartText,
    ChatCompletionRequestMessageContentPartRefusal,
};

/// # on openapi.yaml
/// 
/// ```yaml
/// ChatCompletionRequestAssistantMessageContentPart:
///   oneOf:
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartText"
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartRefusal"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Refusal(ChatCompletionRequestMessageContentPartRefusal),
}
