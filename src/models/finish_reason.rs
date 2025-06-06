#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

/// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
pub enum FinishReason {
    Stop,
    #[serde(rename = "Length")]
    Length,
    ContentFilter,
    ToolCalls,
    FunctionCall,
}

#[test]
fn test_finish_reason() {
    println!("{}", serde_json::to_string(&FinishReason::Stop).unwrap());
    println!("{}", serde_json::to_string(&FinishReason::Length).unwrap());
    println!(
        "{}",
        serde_json::to_string(&FinishReason::ContentFilter).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&FinishReason::ToolCalls).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&FinishReason::FunctionCall).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string(&Option::<FinishReason>::None).unwrap()
    );
}
