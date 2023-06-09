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
pub struct ListAnOrganizationSUsers200ResponseInnerUser {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "lastLogin", deserialize_with = "Option::deserialize")]
    pub last_login: Option<String>,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "lastActive")]
    pub last_active: String,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "has2fa")]
    pub has2fa: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    #[serde(rename = "dateJoined")]
    pub date_joined: String,
    #[serde(rename = "emails")]
    pub emails: Vec<crate::models::ListAnOrganizationSUsers200ResponseInnerUserEmailsInner>,
    #[serde(rename = "avatar")]
    pub avatar: Box<crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar>,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    #[serde(rename = "email")]
    pub email: String,
}

impl ListAnOrganizationSUsers200ResponseInnerUser {
    pub fn new(
        username: String,
        last_login: Option<String>,
        is_superuser: bool,
        is_managed: bool,
        last_active: String,
        is_staff: bool,
        id: String,
        is_active: bool,
        has2fa: bool,
        name: String,
        avatar_url: String,
        date_joined: String,
        emails: Vec<crate::models::ListAnOrganizationSUsers200ResponseInnerUserEmailsInner>,
        avatar: crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar,
        has_password_auth: bool,
        email: String,
    ) -> ListAnOrganizationSUsers200ResponseInnerUser {
        ListAnOrganizationSUsers200ResponseInnerUser {
            username,
            last_login,
            is_superuser,
            is_managed,
            last_active,
            is_staff,
            id,
            is_active,
            has2fa,
            name,
            avatar_url,
            date_joined,
            emails,
            avatar: Box::new(avatar),
            has_password_auth,
            email,
        }
    }
}
