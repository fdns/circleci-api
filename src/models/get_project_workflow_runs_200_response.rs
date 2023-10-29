/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetProjectWorkflowRuns200Response : Paginated recent workflow runs.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProjectWorkflowRuns200Response {
    /// Recent workflow runs.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::GetProjectWorkflowRuns200ResponseItemsInner>,
    /// A token to pass as a `page-token` query parameter to return the next page of results.
    #[serde(rename = "next_page_token", deserialize_with = "Option::deserialize")]
    pub next_page_token: Option<String>,
}

impl GetProjectWorkflowRuns200Response {
    /// Paginated recent workflow runs.
    pub fn new(
        items: Vec<crate::models::GetProjectWorkflowRuns200ResponseItemsInner>,
        next_page_token: Option<String>,
    ) -> GetProjectWorkflowRuns200Response {
        GetProjectWorkflowRuns200Response {
            items,
            next_page_token,
        }
    }
}