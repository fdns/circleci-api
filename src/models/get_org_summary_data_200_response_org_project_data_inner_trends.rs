/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOrgSummaryData200ResponseOrgProjectDataInnerTrends : Trends for a single project, across all branches.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOrgSummaryData200ResponseOrgProjectDataInnerTrends {
    /// The trend value for total credits consumed.
    #[serde(rename = "total_credits_used")]
    pub total_credits_used: f32,
    /// Trend value for total duration.
    #[serde(rename = "total_duration_secs")]
    pub total_duration_secs: f32,
    /// The trend value for total number of runs.
    #[serde(rename = "total_runs")]
    pub total_runs: f32,
    /// The trend value for the success rate.
    #[serde(rename = "success_rate")]
    pub success_rate: f32,
}

impl GetOrgSummaryData200ResponseOrgProjectDataInnerTrends {
    /// Trends for a single project, across all branches.
    pub fn new(
        total_credits_used: f32,
        total_duration_secs: f32,
        total_runs: f32,
        success_rate: f32,
    ) -> GetOrgSummaryData200ResponseOrgProjectDataInnerTrends {
        GetOrgSummaryData200ResponseOrgProjectDataInnerTrends {
            total_credits_used,
            total_duration_secs,
            total_runs,
            success_rate,
        }
    }
}
