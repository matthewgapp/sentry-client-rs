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
pub struct ResolveAnEventId200ResponseEventEntriesInnerAnyOf2 {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "data")]
    pub data: Box<crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf2Data>,
}

impl ResolveAnEventId200ResponseEventEntriesInnerAnyOf2 {
    pub fn new(
        r#type: String,
        data: crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf2Data,
    ) -> ResolveAnEventId200ResponseEventEntriesInnerAnyOf2 {
        ResolveAnEventId200ResponseEventEntriesInnerAnyOf2 {
            r#type,
            data: Box::new(data),
        }
    }
}
