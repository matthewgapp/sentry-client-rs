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
pub struct ResolveAnEventId200ResponseEventEntriesInner {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "data")]
    pub data: Box<crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf3Data>,
}

impl ResolveAnEventId200ResponseEventEntriesInner {
    pub fn new(r#type: String, data: crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf3Data) -> ResolveAnEventId200ResponseEventEntriesInner {
        ResolveAnEventId200ResponseEventEntriesInner {
            r#type,
            data: Box::new(data),
        }
    }
}


