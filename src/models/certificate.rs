/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub Certificate : Represents an individual `certificate` uploaded to the organization.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    /// Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate.
    #[serde(rename = "active")]
    pub active: Option<bool>,
    #[serde(rename = "certificate_details")]
    pub certificate_details: crate::models::CertificateCertificateDetails,
    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the certificate.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type.  - If creating, updating, or getting a specific certificate, the object type is `certificate`. - If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`. - If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.
    #[serde(rename = "object")]
    pub object: String,
}
