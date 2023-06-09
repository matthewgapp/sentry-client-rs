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
pub struct ListAnIssueSHashes200ResponseInner {
    #[serde(rename = "latestEvent", skip_serializing_if = "Option::is_none")]
    pub latest_event: Option<Box<crate::models::ListAnIssueSHashes200ResponseInnerLatestEvent>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ListAnIssueSHashes200ResponseInner {
    pub fn new() -> ListAnIssueSHashes200ResponseInner {
        ListAnIssueSHashes200ResponseInner {
            latest_event: None,
            id: None,
        }
    }
}
