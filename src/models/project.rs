/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// Project : NOTE: The definition of Project is subject to change.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped.
    #[serde(rename = "slug")]
    pub slug: String,
    /// The name of the project
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The name of the organization the project belongs to
    #[serde(rename = "organization_name")]
    pub organization_name: String,
    /// The slug of the organization the project belongs to
    #[serde(rename = "organization_slug")]
    pub organization_slug: String,
    /// The id of the organization the project belongs to
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    #[serde(rename = "vcs_info")]
    pub vcs_info: Box<crate::models::ProjectVcsInfo>,
}

impl Project {
    /// NOTE: The definition of Project is subject to change.
    pub fn new(
        slug: String,
        name: String,
        id: uuid::Uuid,
        organization_name: String,
        organization_slug: String,
        organization_id: uuid::Uuid,
        vcs_info: crate::models::ProjectVcsInfo,
    ) -> Project {
        Project {
            slug,
            name,
            id,
            organization_name,
            organization_slug,
            organization_id,
            vcs_info: Box::new(vcs_info),
        }
    }
}
