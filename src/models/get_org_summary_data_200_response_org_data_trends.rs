/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOrgSummaryData200ResponseOrgDataTrends : Trends for a single org.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOrgSummaryData200ResponseOrgDataTrends {
    /// The trend value for total number of runs.
    #[serde(rename = "total_runs")]
    pub total_runs: f32,
    /// Trend value for total duration.
    #[serde(rename = "total_duration_secs")]
    pub total_duration_secs: f32,
    /// The trend value for total credits consumed.
    #[serde(rename = "total_credits_used")]
    pub total_credits_used: f32,
    /// The trend value for the success rate.
    #[serde(rename = "success_rate")]
    pub success_rate: f32,
    /// Trend value for the average number of runs per day.
    #[serde(rename = "throughput")]
    pub throughput: f32,
}

impl GetOrgSummaryData200ResponseOrgDataTrends {
    /// Trends for a single org.
    pub fn new(
        total_runs: f32,
        total_duration_secs: f32,
        total_credits_used: f32,
        success_rate: f32,
        throughput: f32,
    ) -> GetOrgSummaryData200ResponseOrgDataTrends {
        GetOrgSummaryData200ResponseOrgDataTrends {
            total_runs,
            total_duration_secs,
            total_credits_used,
            success_rate,
            throughput,
        }
    }
}