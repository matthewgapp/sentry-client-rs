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
pub struct ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerApp {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerApp {
    pub fn new(
        uuid: String,
        slug: String,
    ) -> ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerApp {
        ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerApp { uuid, slug }
    }
}
