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
pub struct RetrieveReleaseHealthSessionStatistics200Response {
    /// The start time of the data being returned.
    #[serde(rename = "start")]
    pub start: String,
    /// The exclusive end time of the data being returned.
    #[serde(rename = "end")]
    pub end: String,
    /// The time slices of the timeseries data given in the `groups[].series` field.
    #[serde(rename = "intervals")]
    pub intervals: Vec<String>,
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::RetrieveReleaseHealthSessionStatistics200ResponseGroupsInner>,
}

impl RetrieveReleaseHealthSessionStatistics200Response {
    pub fn new(start: String, end: String, intervals: Vec<String>, groups: Vec<crate::models::RetrieveReleaseHealthSessionStatistics200ResponseGroupsInner>) -> RetrieveReleaseHealthSessionStatistics200Response {
        RetrieveReleaseHealthSessionStatistics200Response {
            start,
            end,
            intervals,
            groups,
        }
    }
}


