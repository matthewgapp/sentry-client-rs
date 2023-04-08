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
pub struct CreateANewReleaseForAnOrganizationRequestCommitsInner {
    /// A list of the files that have been changed in the commit. Specifying the patch_set is necessary to power suspect commits and suggested assignees.
    #[serde(rename = "patch_set", skip_serializing_if = "Option::is_none")]
    pub patch_set: Option<Vec<crate::models::CreateANewReleaseForAnOrganizationRequestCommitsInnerPatchSetInner>>,
    /// The full name of the repository the commit belongs to. If this field is not given Sentry will generate a name in the form: u'organization-<organization_id>' (i.e. if the organization id is 123, then the generated repository name will be u'organization-123).
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// The name of the commit author.
    #[serde(rename = "author_name", skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// The email of the commit author. The commit author's email is required to enable the suggested assignee feature.
    #[serde(rename = "author_email", skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    /// The commit timestamp is used to sort the commits given. If a timestamp is not included, the commits will remain sorted in the order given.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The commit message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The commit ID (the commit SHA).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateANewReleaseForAnOrganizationRequestCommitsInner {
    pub fn new() -> CreateANewReleaseForAnOrganizationRequestCommitsInner {
        CreateANewReleaseForAnOrganizationRequestCommitsInner {
            patch_set: None,
            repository: None,
            author_name: None,
            author_email: None,
            timestamp: None,
            message: None,
            id: None,
        }
    }
}

