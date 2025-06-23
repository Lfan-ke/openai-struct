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
    ChatCompletionRequestAssistantMessage, 
    ChatCompletionRequestDeveloperMessage, 
    ChatCompletionRequestFunctionMessage, 
    ChatCompletionRequestSystemMessage, 
    ChatCompletionRequestToolMessage, 
    ChatCompletionRequestUserMessage,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum ChatCompletionRequestMessage {
    #[serde(rename = "developer")] 
    Developer(ChatCompletionRequestDeveloperMessage),
    #[serde(rename = "system")]
    System(ChatCompletionRequestSystemMessage),
    #[serde(rename = "user")]
    User(ChatCompletionRequestUserMessage),
    #[serde(rename = "assistant")]
    Assistant(ChatCompletionRequestAssistantMessage),
    #[serde(rename = "tool")]
    Tool(ChatCompletionRequestToolMessage),
    #[serde(rename = "function")]
    Function(ChatCompletionRequestFunctionMessage),
}

#[test]
fn test_serde_chat_completion_request_message() {
    use serde_json::Value;
    
    let tmp = ChatCompletionRequestMessage::User(
        ChatCompletionRequestUserMessage {
            content: Value::String("once upon a time".to_string()),
            name: None,
        }
    );
    
    let serialized = serde_json::to_string(&tmp).unwrap();
    assert_eq!(
        serialized, 
        r#"{"role":"user","content":"once upon a time","name":null}"#
    );
    
    let deserialized: ChatCompletionRequestMessage = serde_json::from_str(&serialized).unwrap();
    assert_eq!(tmp, deserialized);
    
    let emm: ChatCompletionRequestMessage = serde_json::from_str(
        r#"{"content":"once upon a time","name":null,"role":"user"}"#
    ).unwrap();
    assert_eq!(tmp, emm);
}
