/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProjectVcsInfo : Information about the VCS that hosts the project source code.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectVcsInfo {
    /// URL to the repository hosting the project's code
    #[serde(rename = "vcs_url")]
    pub vcs_url: String,
    /// The VCS provider
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
}

impl ProjectVcsInfo {
    /// Information about the VCS that hosts the project source code.
    pub fn new(vcs_url: String, provider: Provider, default_branch: String) -> ProjectVcsInfo {
        ProjectVcsInfo {
            vcs_url,
            provider,
            default_branch,
        }
    }
}

/// The VCS provider
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "Bitbucket")]
    Bitbucket,
    #[serde(rename = "CircleCI")]
    CircleCi,
    #[serde(rename = "GitHub")]
    GitHub,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Bitbucket
    }
}
