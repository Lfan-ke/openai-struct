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

use crate::{ComparisonFilter, CompoundFilter};

/// # on openapi.yaml
///
/// ```yaml
/// Filters:
///   anyOf:
///     - $ref: "#/components/schemas/ComparisonFilter"
///     - $ref: "#/components/schemas/CompoundFilter"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum Filters {
    Comparison(ComparisonFilter),
    Compound(CompoundFilter),
}
