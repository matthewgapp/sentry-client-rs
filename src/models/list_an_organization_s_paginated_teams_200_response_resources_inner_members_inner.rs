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
pub struct ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMembersInner {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "display")]
    pub display: String,
}

impl ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMembersInner {
    pub fn new(value: String, display: String) -> ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMembersInner {
        ListAnOrganizationSPaginatedTeams200ResponseResourcesInnerMembersInner {
            value,
            display,
        }
    }
}

