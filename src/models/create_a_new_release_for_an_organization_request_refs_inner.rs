/*
 * API Reference
 *
 * Sentry Public API
 *
 * The version of the OpenAPI document: v0
 * Contact: partners@sentry.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateANewReleaseForAnOrganizationRequestRefsInner {
    /// The full name of the repository the commit belongs to.
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// The current release's commit.
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// The previous release's commit.
    #[serde(rename = "previousCommit", skip_serializing_if = "Option::is_none")]
    pub previous_commit: Option<String>,
}

impl CreateANewReleaseForAnOrganizationRequestRefsInner {
    pub fn new() -> CreateANewReleaseForAnOrganizationRequestRefsInner {
        CreateANewReleaseForAnOrganizationRequestRefsInner {
            repository: None,
            commit: None,
            previous_commit: None,
        }
    }
}
