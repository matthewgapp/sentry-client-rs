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
pub struct RetrieveAnOrganization200ResponseProjectsInnerTeam {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl RetrieveAnOrganization200ResponseProjectsInnerTeam {
    pub fn new(
        id: String,
        name: String,
        slug: String,
    ) -> RetrieveAnOrganization200ResponseProjectsInnerTeam {
        RetrieveAnOrganization200ResponseProjectsInnerTeam { id, name, slug }
    }
}
