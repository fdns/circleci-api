/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// PipelineTriggerActor : The user who triggered the Pipeline.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTriggerActor {
    /// The login information for the user on the VCS.
    #[serde(rename = "login")]
    pub login: String,
    /// URL to the user's avatar on the VCS
    #[serde(rename = "avatar_url", deserialize_with = "Option::deserialize")]
    pub avatar_url: Option<String>,
}

impl PipelineTriggerActor {
    /// The user who triggered the Pipeline.
    pub fn new(login: String, avatar_url: Option<String>) -> PipelineTriggerActor {
        PipelineTriggerActor { login, avatar_url }
    }
}
