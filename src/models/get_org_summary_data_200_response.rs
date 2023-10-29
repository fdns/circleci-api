/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOrgSummaryData200Response : Summary metrics with trends for the entire org, and for each project.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOrgSummaryData200Response {
    #[serde(rename = "org_data")]
    pub org_data: Box<crate::models::GetOrgSummaryData200ResponseOrgData>,
    /// Metrics for a single project, across all branches
    #[serde(rename = "org_project_data")]
    pub org_project_data: Vec<crate::models::GetOrgSummaryData200ResponseOrgProjectDataInner>,
    /// A list of all the project names in the organization.
    #[serde(rename = "all_projects", deserialize_with = "Option::deserialize")]
    pub all_projects: Option<Vec<String>>,
}

impl GetOrgSummaryData200Response {
    /// Summary metrics with trends for the entire org, and for each project.
    pub fn new(
        org_data: crate::models::GetOrgSummaryData200ResponseOrgData,
        org_project_data: Vec<crate::models::GetOrgSummaryData200ResponseOrgProjectDataInner>,
        all_projects: Option<Vec<String>>,
    ) -> GetOrgSummaryData200Response {
        GetOrgSummaryData200Response {
            org_data: Box::new(org_data),
            org_project_data,
            all_projects,
        }
    }
}