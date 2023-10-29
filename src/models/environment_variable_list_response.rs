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
pub struct EnvironmentVariableListResponse {
    #[serde(rename = "items")]
    pub items: Vec<crate::models::EnvironmentVariable>,
    /// A token to pass as a `page-token` query parameter to return the next page of results.
    #[serde(rename = "next_page_token", deserialize_with = "Option::deserialize")]
    pub next_page_token: Option<String>,
}

impl EnvironmentVariableListResponse {
    pub fn new(
        items: Vec<crate::models::EnvironmentVariable>,
        next_page_token: Option<String>,
    ) -> EnvironmentVariableListResponse {
        EnvironmentVariableListResponse {
            items,
            next_page_token,
        }
    }
}
