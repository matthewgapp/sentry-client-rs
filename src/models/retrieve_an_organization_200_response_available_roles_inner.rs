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
pub struct RetrieveAnOrganization200ResponseAvailableRolesInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RetrieveAnOrganization200ResponseAvailableRolesInner {
    pub fn new() -> RetrieveAnOrganization200ResponseAvailableRolesInner {
        RetrieveAnOrganization200ResponseAvailableRolesInner {
            id: None,
            name: None,
        }
    }
}
