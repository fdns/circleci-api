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
pub struct CreateContextRequestOwnerOneOf1 {
    /// A string that represents an organization. Specify either this or id. Cannot be used for accounts.
    #[serde(rename = "slug")]
    pub slug: String,
    /// The type of owner. Defaults to \"organization\". Accounts are only used as context owners in server and must be specified by an id instead of a slug.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl CreateContextRequestOwnerOneOf1 {
    pub fn new(slug: String) -> CreateContextRequestOwnerOneOf1 {
        CreateContextRequestOwnerOneOf1 { slug, r#type: None }
    }
}

/// The type of owner. Defaults to \"organization\". Accounts are only used as context owners in server and must be specified by an id instead of a slug.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "organization")]
    Organization,
}

impl Default for Type {
    fn default() -> Type {
        Self::Organization
    }
}