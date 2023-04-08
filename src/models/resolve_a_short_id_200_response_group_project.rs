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
pub struct ResolveAShortId200ResponseGroupProject {
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ResolveAShortId200ResponseGroupProject {
    pub fn new() -> ResolveAShortId200ResponseGroupProject {
        ResolveAShortId200ResponseGroupProject {
            slug: None,
            id: None,
            name: None,
        }
    }
}

