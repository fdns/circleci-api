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
pub struct GetProjectWorkflowsPageData200ResponseProjectWorkflowBranchDataInner {
    /// The name of the workflow.
    #[serde(rename = "workflow_name")]
    pub workflow_name: String,
    /// The VCS branch of a workflow's trigger.
    #[serde(rename = "branch")]
    pub branch: String,
    #[serde(rename = "metrics")]
    pub metrics:
        Box<crate::models::GetProjectWorkflowsPageData200ResponseProjectWorkflowDataInnerMetrics>,
    #[serde(rename = "trends")]
    pub trends:
        Box<crate::models::GetProjectWorkflowsPageData200ResponseProjectWorkflowDataInnerTrends>,
}

impl GetProjectWorkflowsPageData200ResponseProjectWorkflowBranchDataInner {
    pub fn new(
        workflow_name: String,
        branch: String,
        metrics: crate::models::GetProjectWorkflowsPageData200ResponseProjectWorkflowDataInnerMetrics,
        trends: crate::models::GetProjectWorkflowsPageData200ResponseProjectWorkflowDataInnerTrends,
    ) -> GetProjectWorkflowsPageData200ResponseProjectWorkflowBranchDataInner {
        GetProjectWorkflowsPageData200ResponseProjectWorkflowBranchDataInner {
            workflow_name,
            branch,
            metrics: Box::new(metrics),
            trends: Box::new(trends),
        }
    }
}