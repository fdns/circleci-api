/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// JobDetailsOrganization : Information about an organization.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetailsOrganization {
    /// The name of the organization.
    #[serde(rename = "name")]
    pub name: String,
}

impl JobDetailsOrganization {
    /// Information about an organization.
    pub fn new(name: String) -> JobDetailsOrganization {
        JobDetailsOrganization { name }
    }
}
