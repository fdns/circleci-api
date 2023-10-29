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
pub struct DecisionLogMetadata {
    #[serde(rename = "build_number", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<uuid::Uuid>,
    #[serde(rename = "ssh_rerun", skip_serializing_if = "Option::is_none")]
    pub ssh_rerun: Option<bool>,
    #[serde(rename = "vcs", skip_serializing_if = "Option::is_none")]
    pub vcs: Option<Box<crate::models::DecisionLogMetadataVcs>>,
}

impl DecisionLogMetadata {
    pub fn new() -> DecisionLogMetadata {
        DecisionLogMetadata {
            build_number: None,
            project_id: None,
            ssh_rerun: None,
            vcs: None,
        }
    }
}
