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
pub struct GetDecisionLogs401Response {
    #[serde(rename = "error")]
    pub error: String,
}

impl GetDecisionLogs401Response {
    pub fn new(error: String) -> GetDecisionLogs401Response {
        GetDecisionLogs401Response { error }
    }
}
