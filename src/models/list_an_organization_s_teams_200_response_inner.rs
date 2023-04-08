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
pub struct ListAnOrganizationSTeams200ResponseInner {
    #[serde(rename = "avatar")]
    pub avatar: Box<crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar>,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "projects")]
    pub projects: Vec<crate::models::ListAnOrganizationSTeams200ResponseInnerProjectsInner>,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl ListAnOrganizationSTeams200ResponseInner {
    pub fn new(avatar: crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar, date_created: String, has_access: bool, id: String, is_member: bool, is_pending: bool, member_count: i64, name: String, projects: Vec<crate::models::ListAnOrganizationSTeams200ResponseInnerProjectsInner>, slug: String) -> ListAnOrganizationSTeams200ResponseInner {
        ListAnOrganizationSTeams200ResponseInner {
            avatar: Box::new(avatar),
            date_created,
            has_access,
            id,
            is_member,
            is_pending,
            member_count,
            name,
            projects,
            slug,
        }
    }
}

