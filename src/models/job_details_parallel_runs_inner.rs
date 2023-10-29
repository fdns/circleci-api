/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// JobDetailsParallelRunsInner : Info about a status of the parallel run.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetailsParallelRunsInner {
    /// Index of the parallel run.
    #[serde(rename = "index")]
    pub index: i64,
    /// Status of the parallel run.
    #[serde(rename = "status")]
    pub status: String,
}

impl JobDetailsParallelRunsInner {
    /// Info about a status of the parallel run.
    pub fn new(index: i64, status: String) -> JobDetailsParallelRunsInner {
        JobDetailsParallelRunsInner { index, status }
    }
}
