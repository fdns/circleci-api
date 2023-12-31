/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// JobDetailsProject : Information about a project.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetailsProject {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped.
    #[serde(rename = "slug")]
    pub slug: String,
    /// The name of the project
    #[serde(rename = "name")]
    pub name: String,
    /// URL to the repository hosting the project's code
    #[serde(rename = "external_url")]
    pub external_url: String,
}

impl JobDetailsProject {
    /// Information about a project.
    pub fn new(
        id: uuid::Uuid,
        slug: String,
        name: String,
        external_url: String,
    ) -> JobDetailsProject {
        JobDetailsProject {
            id,
            slug,
            name,
            external_url,
        }
    }
}
