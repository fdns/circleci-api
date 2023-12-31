/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// PipelineVcs : VCS information for the pipeline.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineVcs {
    /// Name of the VCS provider (e.g. GitHub, Bitbucket).
    #[serde(rename = "provider_name")]
    pub provider_name: String,
    /// URL for the repository the trigger targets (i.e. the repository where the PR will be merged). For fork-PR pipelines, this is the URL to the parent repo. For other pipelines, the `origin_` and `target_repository_url`s will be the same.
    #[serde(rename = "target_repository_url")]
    pub target_repository_url: String,
    /// The branch where the pipeline ran. The HEAD commit on this branch was used for the pipeline. Note that `branch` and `tag` are mutually exclusive. To trigger a pipeline for a PR by number use `pull/<number>/head` for the PR ref or `pull/<number>/merge` for the merge ref (GitHub only).
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The code review id.
    #[serde(rename = "review_id", skip_serializing_if = "Option::is_none")]
    pub review_id: Option<String>,
    /// The code review URL.
    #[serde(rename = "review_url", skip_serializing_if = "Option::is_none")]
    pub review_url: Option<String>,
    /// The code revision the pipeline ran.
    #[serde(rename = "revision")]
    pub revision: String,
    /// The tag used by the pipeline. The commit that this tag points to was used for the pipeline. Note that `branch` and `tag` are mutually exclusive.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<Box<crate::models::PipelineVcsCommit>>,
    /// URL for the repository where the trigger originated. For fork-PR pipelines, this is the URL to the fork. For other pipelines the `origin_` and `target_repository_url`s will be the same.
    #[serde(rename = "origin_repository_url")]
    pub origin_repository_url: String,
}

impl PipelineVcs {
    /// VCS information for the pipeline.
    pub fn new(
        provider_name: String,
        target_repository_url: String,
        revision: String,
        origin_repository_url: String,
    ) -> PipelineVcs {
        PipelineVcs {
            provider_name,
            target_repository_url,
            branch: None,
            review_id: None,
            review_url: None,
            revision,
            tag: None,
            commit: None,
            origin_repository_url,
        }
    }
}
