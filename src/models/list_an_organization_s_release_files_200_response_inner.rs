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
pub struct ListAnOrganizationSReleaseFiles200ResponseInner {
    #[serde(rename = "sha1")]
    pub sha1: String,
    #[serde(rename = "dist", deserialize_with = "Option::deserialize")]
    pub dist: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "headers")]
    pub headers: serde_json::Value,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "size")]
    pub size: i32,
}

impl ListAnOrganizationSReleaseFiles200ResponseInner {
    pub fn new(
        sha1: String,
        dist: Option<String>,
        name: String,
        date_created: String,
        headers: serde_json::Value,
        id: String,
        size: i32,
    ) -> ListAnOrganizationSReleaseFiles200ResponseInner {
        ListAnOrganizationSReleaseFiles200ResponseInner {
            sha1,
            dist,
            name,
            date_created,
            headers,
            id,
            size,
        }
    }
}
