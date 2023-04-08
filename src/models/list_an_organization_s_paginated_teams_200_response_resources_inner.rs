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
pub struct ListAnOrganizationSPaginatedTeams200ResponseResourcesInner {
    #[serde(rename = "schemas")]
    pub schemas: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "members")]
    pub members: Vec<crate::models::ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMembersInner>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMeta>,
}

impl ListAnOrganizationSPaginatedTeams200ResponseResourcesInner {
    pub fn new(schemas: Vec<String>, id: String, display_name: String, members: Vec<crate::models::ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMembersInner>, meta: crate::models::ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMeta) -> ListAnOrganizationSPaginatedTeams200ResponseResourcesInner {
        ListAnOrganizationSPaginatedTeams200ResponseResourcesInner {
            schemas,
            id,
            display_name,
            members,
            meta: Box::new(meta),
        }
    }
}

