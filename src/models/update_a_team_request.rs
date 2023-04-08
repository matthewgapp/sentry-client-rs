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
pub struct UpdateATeamRequest {
    /// The new name for the team.
    #[serde(rename = "name")]
    pub name: String,
    /// A new slug for the team. It has to be unique and available.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl UpdateATeamRequest {
    pub fn new(name: String) -> UpdateATeamRequest {
        UpdateATeamRequest {
            name,
            slug: None,
        }
    }
}


