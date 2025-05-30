/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ModifyThreadRequestToolResources : A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyThreadRequestToolResources {
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<crate::models::CreateAssistantRequestToolResourcesCodeInterpreter>,
    #[serde(rename = "file_search")]
    pub file_search: Option<crate::models::ModifyThreadRequestToolResourcesFileSearch>,
}
