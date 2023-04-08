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
pub struct ResolveAnEventId200ResponseEventEntriesInnerAnyOf3Data {
    #[serde(rename = "excOmitted", deserialize_with = "Option::deserialize")]
    pub exc_omitted: Option<Vec<i32>>,
    #[serde(rename = "hasSystemFrames")]
    pub has_system_frames: bool,
    #[serde(rename = "values")]
    pub values: Vec<crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf3DataValuesInner>,
}

impl ResolveAnEventId200ResponseEventEntriesInnerAnyOf3Data {
    pub fn new(exc_omitted: Option<Vec<i32>>, has_system_frames: bool, values: Vec<crate::models::ResolveAnEventId200ResponseEventEntriesInnerAnyOf3DataValuesInner>) -> ResolveAnEventId200ResponseEventEntriesInnerAnyOf3Data {
        ResolveAnEventId200ResponseEventEntriesInnerAnyOf3Data {
            exc_omitted,
            has_system_frames,
            values,
        }
    }
}

