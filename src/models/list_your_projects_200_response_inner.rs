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
pub struct ListYourProjects200ResponseInner {
    #[serde(rename = "avatar")]
    pub avatar: Box<crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar>,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "features")]
    pub features: Vec<String>,
    #[serde(rename = "firstEvent", deserialize_with = "Option::deserialize")]
    pub first_event: Option<String>,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organization")]
    pub organization: Box<crate::models::RetrieveATeam200ResponseOrganization>,
    #[serde(rename = "platform", deserialize_with = "Option::deserialize")]
    pub platform: Option<String>,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "status")]
    pub status: Status,
}

impl ListYourProjects200ResponseInner {
    pub fn new(
        avatar: crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar,
        color: String,
        date_created: String,
        features: Vec<String>,
        first_event: Option<String>,
        has_access: bool,
        id: String,
        is_bookmarked: bool,
        is_internal: bool,
        is_member: bool,
        is_public: bool,
        name: String,
        organization: crate::models::RetrieveATeam200ResponseOrganization,
        platform: Option<String>,
        slug: String,
        status: Status,
    ) -> ListYourProjects200ResponseInner {
        ListYourProjects200ResponseInner {
            avatar: Box::new(avatar),
            color,
            date_created,
            features,
            first_event,
            has_access,
            id,
            is_bookmarked,
            is_internal,
            is_member,
            is_public,
            name,
            organization: Box::new(organization),
            platform,
            slug,
            status,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "pending_deletion")]
    PendingDeletion,
    #[serde(rename = "deletion_in_progress")]
    DeletionInProgress,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
