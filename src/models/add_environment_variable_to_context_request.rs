/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddEnvironmentVariableToContextRequest {
    /// The value of the environment variable
    #[serde(rename = "value")]
    pub value: String,
}

impl AddEnvironmentVariableToContextRequest {
    pub fn new(value: String) -> AddEnvironmentVariableToContextRequest {
        AddEnvironmentVariableToContextRequest { value }
    }
}
