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
pub struct ResolveAnEventId200ResponseEventMetadata {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ResolveAnEventId200ResponseEventMetadata {
    pub fn new() -> ResolveAnEventId200ResponseEventMetadata {
        ResolveAnEventId200ResponseEventMetadata { title: None }
    }
}
