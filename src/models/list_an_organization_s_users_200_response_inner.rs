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
pub struct ListAnOrganizationSUsers200ResponseInner {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "user")]
    pub user: Box<crate::models::ListAnOrganizationSUsers200ResponseInnerUser>,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "expired")]
    pub expired: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "projects")]
    pub projects: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "role")]
    pub role: Role,
    #[serde(rename = "flags")]
    pub flags: Box<crate::models::ListAnOrganizationSUsers200ResponseInnerFlags>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "pending")]
    pub pending: bool,
}

impl ListAnOrganizationSUsers200ResponseInner {
    pub fn new(
        date_created: String,
        user: crate::models::ListAnOrganizationSUsers200ResponseInnerUser,
        role_name: String,
        expired: bool,
        id: String,
        projects: Vec<String>,
        name: String,
        role: Role,
        flags: crate::models::ListAnOrganizationSUsers200ResponseInnerFlags,
        email: String,
        pending: bool,
    ) -> ListAnOrganizationSUsers200ResponseInner {
        ListAnOrganizationSUsers200ResponseInner {
            date_created,
            user: Box::new(user),
            role_name,
            expired,
            id,
            projects,
            name,
            role,
            flags: Box::new(flags),
            email,
            pending,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "billing")]
    Billing,
}

impl Default for Role {
    fn default() -> Role {
        Self::Owner
    }
}
