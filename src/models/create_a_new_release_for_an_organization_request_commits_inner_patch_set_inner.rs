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
pub struct CreateANewReleaseForAnOrganizationRequestCommitsInnerPatchSetInner {
    /// The path to the file. Both forward and backward slashes are supported.
    #[serde(rename = "path")]
    pub path: String,
    /// The type of change that happened in the commit.
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl CreateANewReleaseForAnOrganizationRequestCommitsInnerPatchSetInner {
    pub fn new(
        path: String,
        r#type: RHashType,
    ) -> CreateANewReleaseForAnOrganizationRequestCommitsInnerPatchSetInner {
        CreateANewReleaseForAnOrganizationRequestCommitsInnerPatchSetInner { path, r#type }
    }
}

/// The type of change that happened in the commit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "D")]
    D,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::A
    }
}
