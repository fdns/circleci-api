/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// RerunWorkflow202Response : A response to rerunning a workflow

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RerunWorkflow202Response {
    /// The ID of the newly-created workflow.
    #[serde(rename = "workflow_id")]
    pub workflow_id: uuid::Uuid,
}

impl RerunWorkflow202Response {
    /// A response to rerunning a workflow
    pub fn new(workflow_id: uuid::Uuid) -> RerunWorkflow202Response {
        RerunWorkflow202Response { workflow_id }
    }
}