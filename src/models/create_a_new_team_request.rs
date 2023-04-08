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
pub struct CreateANewTeamRequest {
    /// The name of the team.
    #[serde(rename = "name")]
    pub name: String,
    /// The optional slug for this team. If not provided it will be auto generated from the name.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl CreateANewTeamRequest {
    pub fn new(name: String) -> CreateANewTeamRequest {
        CreateANewTeamRequest {
            name,
            slug: None,
        }
    }
}

