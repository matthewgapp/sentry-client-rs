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
pub struct ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerOrganization {
    #[serde(rename = "slug")]
    pub slug: String,
}

impl ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerOrganization {
    pub fn new(slug: String) -> ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerOrganization {
        ListAnOrganizationSIntegrationPlatformInstallations200ResponseInnerOrganization {
            slug,
        }
    }
}


