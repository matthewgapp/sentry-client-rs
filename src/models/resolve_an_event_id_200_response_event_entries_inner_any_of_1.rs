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
pub struct ResolveAnEventId200ResponseEventEntriesInnerAnyOf1 {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "data")]
    pub data: Box<crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf1Data>,
}

impl ResolveAnEventId200ResponseEventEntriesInnerAnyOf1 {
    pub fn new(
        r#type: String,
        data: crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf1Data,
    ) -> ResolveAnEventId200ResponseEventEntriesInnerAnyOf1 {
        ResolveAnEventId200ResponseEventEntriesInnerAnyOf1 {
            r#type,
            data: Box::new(data),
        }
    }
}
