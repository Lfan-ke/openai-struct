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
pub struct AuditLogCertificatesActivatedCertificates {
    /// The certificate ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the certificate.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
