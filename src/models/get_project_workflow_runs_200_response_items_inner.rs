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
pub struct GetProjectWorkflowRuns200ResponseItemsInner {
    /// The unique ID of the workflow.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The VCS branch of a Workflow's trigger.
    #[serde(rename = "branch")]
    pub branch: String,
    /// The duration in seconds of a run.
    #[serde(rename = "duration", deserialize_with = "Option::deserialize")]
    pub duration: Option<i64>,
    /// The date and time the workflow was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the workflow stopped.
    #[serde(rename = "stopped_at", deserialize_with = "Option::deserialize")]
    pub stopped_at: Option<String>,
    /// The number of credits used during execution. Note that Insights is not a real time financial reporting tool and should not be used for credit reporting.
    #[serde(rename = "credits_used")]
    pub credits_used: i64,
    /// Workflow status.
    #[serde(rename = "status", deserialize_with = "Option::deserialize")]
    pub status: Option<Status>,
    /// Describes if the job is an approval job or not. Approval jobs are intermediary jobs that are created to pause the workflow until approved.
    #[serde(rename = "is_approval")]
    pub is_approval: bool,
}

impl GetProjectWorkflowRuns200ResponseItemsInner {
    pub fn new(
        id: uuid::Uuid,
        branch: String,
        duration: Option<i64>,
        created_at: String,
        stopped_at: Option<String>,
        credits_used: i64,
        status: Option<Status>,
        is_approval: bool,
    ) -> GetProjectWorkflowRuns200ResponseItemsInner {
        GetProjectWorkflowRuns200ResponseItemsInner {
            id,
            branch,
            duration,
            created_at,
            stopped_at,
            credits_used,
            status,
            is_approval,
        }
    }
}

/// Workflow status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "unauthorized")]
    Unauthorized,
}

impl Default for Status {
    fn default() -> Status {
        Self::Success
    }
}
