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
pub struct GetFlakyTests200ResponseFlakyTestsInner {
    #[serde(rename = "time-wasted", skip_serializing_if = "Option::is_none")]
    pub time_wasted: Option<Box<i64>>,
    /// The date and time when workflow was created.
    #[serde(
        rename = "workflow-created-at",
        deserialize_with = "Option::deserialize"
    )]
    pub workflow_created_at: Option<serde_json::Value>,
    /// The ID of the workflow associated with the provided test counts
    #[serde(rename = "workflow-id", deserialize_with = "Option::deserialize")]
    pub workflow_id: Option<serde_json::Value>,
    /// The class the test belongs to.
    #[serde(rename = "classname", deserialize_with = "Option::deserialize")]
    pub classname: Option<String>,
    /// The number of the pipeline.
    #[serde(rename = "pipeline-number")]
    pub pipeline_number: Box<i64>,
    /// The name of the workflow.
    #[serde(rename = "workflow-name")]
    pub workflow_name: String,
    /// The name of the test.
    #[serde(rename = "test-name")]
    pub test_name: String,
    /// The name of the job.
    #[serde(rename = "job-name")]
    pub job_name: String,
    /// The number of the job.
    #[serde(rename = "job-number")]
    pub job_number: Box<i64>,
    /// The number of times the test flaked.
    #[serde(rename = "times-flaked")]
    pub times_flaked: i64,
    /// The source of the test.
    #[serde(rename = "source", deserialize_with = "Option::deserialize")]
    pub source: Option<String>,
    /// The file the test belongs to.
    #[serde(rename = "file", deserialize_with = "Option::deserialize")]
    pub file: Option<String>,
}

impl GetFlakyTests200ResponseFlakyTestsInner {
    pub fn new(
        workflow_created_at: Option<serde_json::Value>,
        workflow_id: Option<serde_json::Value>,
        classname: Option<String>,
        pipeline_number: i64,
        workflow_name: String,
        test_name: String,
        job_name: String,
        job_number: i64,
        times_flaked: i64,
        source: Option<String>,
        file: Option<String>,
    ) -> GetFlakyTests200ResponseFlakyTestsInner {
        GetFlakyTests200ResponseFlakyTestsInner {
            time_wasted: None,
            workflow_created_at,
            workflow_id,
            classname,
            pipeline_number: Box::new(pipeline_number),
            workflow_name,
            test_name,
            job_name,
            job_number: Box::new(job_number),
            times_flaked,
            source,
            file,
        }
    }
}