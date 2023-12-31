/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetWorkflowSummary200ResponseTrends : Trends for aggregated metrics across a workflow for a given time window.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWorkflowSummary200ResponseTrends {
    /// The trend value for total number of runs.
    #[serde(rename = "total_runs")]
    pub total_runs: f32,
    /// The trend value for number of failed runs.
    #[serde(rename = "failed_runs")]
    pub failed_runs: f32,
    /// The trend value for the success rate.
    #[serde(rename = "success_rate")]
    pub success_rate: f32,
    /// Trend value for the 95th percentile duration for a workflow for a given time window.
    #[serde(rename = "p95_duration_secs")]
    pub p95_duration_secs: f32,
    /// Trend value for the 50th percentile duration for a workflow for a given time window.
    #[serde(rename = "median_duration_secs")]
    pub median_duration_secs: f32,
    /// The trend value for total credits consumed.
    #[serde(rename = "total_credits_used")]
    pub total_credits_used: f32,
    /// trend for mean time to recovery (mean time between failures and their next success).
    #[serde(rename = "mttr")]
    pub mttr: f32,
    /// Trend value for the average number of runs per day.
    #[serde(rename = "throughput")]
    pub throughput: f32,
}

impl GetWorkflowSummary200ResponseTrends {
    /// Trends for aggregated metrics across a workflow for a given time window.
    pub fn new(
        total_runs: f32,
        failed_runs: f32,
        success_rate: f32,
        p95_duration_secs: f32,
        median_duration_secs: f32,
        total_credits_used: f32,
        mttr: f32,
        throughput: f32,
    ) -> GetWorkflowSummary200ResponseTrends {
        GetWorkflowSummary200ResponseTrends {
            total_runs,
            failed_runs,
            success_rate,
            p95_duration_secs,
            median_duration_secs,
            total_credits_used,
            mttr,
            throughput,
        }
    }
}
