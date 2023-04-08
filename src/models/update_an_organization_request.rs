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
pub struct UpdateAnOrganizationRequest {
    /// An optional new name for the organization.
    #[serde(rename = "name")]
    pub name: String,
    /// An optional new slug for the organization. Needs to be available and unique.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl UpdateAnOrganizationRequest {
    pub fn new(name: String) -> UpdateAnOrganizationRequest {
        UpdateAnOrganizationRequest {
            name,
            slug: None,
        }
    }
}


