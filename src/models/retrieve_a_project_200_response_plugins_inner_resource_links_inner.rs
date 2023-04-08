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
pub struct RetrieveAProject200ResponsePluginsInnerResourceLinksInner {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RetrieveAProject200ResponsePluginsInnerResourceLinksInner {
    pub fn new() -> RetrieveAProject200ResponsePluginsInnerResourceLinksInner {
        RetrieveAProject200ResponsePluginsInnerResourceLinksInner {
            title: None,
            url: None,
        }
    }
}

