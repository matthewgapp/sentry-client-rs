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
pub struct RetrieveATeam200ResponseOrganizationStatus {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl RetrieveATeam200ResponseOrganizationStatus {
    pub fn new(id: String, name: String) -> RetrieveATeam200ResponseOrganizationStatus {
        RetrieveATeam200ResponseOrganizationStatus {
            id,
            name,
        }
    }
}


