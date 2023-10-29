/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowListResponse : A list of workflows and associated pagination token.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowListResponse {
    /// A list of workflows.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::Workflow>,
    /// A token to pass as a `page-token` query parameter to return the next page of results.
    #[serde(rename = "next_page_token", deserialize_with = "Option::deserialize")]
    pub next_page_token: Option<String>,
}

impl WorkflowListResponse {
    /// A list of workflows and associated pagination token.
    pub fn new(
        items: Vec<crate::models::Workflow>,
        next_page_token: Option<String>,
    ) -> WorkflowListResponse {
        WorkflowListResponse {
            items,
            next_page_token,
        }
    }
}
