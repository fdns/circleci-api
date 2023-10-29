/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// JobDetailsExecutor : Information about executor used for a job.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetailsExecutor {
    /// Resource class name.
    #[serde(rename = "resource_class")]
    pub resource_class: String,
    /// Executor type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl JobDetailsExecutor {
    /// Information about executor used for a job.
    pub fn new(resource_class: String) -> JobDetailsExecutor {
        JobDetailsExecutor {
            resource_class,
            r#type: None,
        }
    }
}