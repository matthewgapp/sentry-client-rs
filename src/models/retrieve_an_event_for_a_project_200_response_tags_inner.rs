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
pub struct RetrieveAnEventForAProject200ResponseTagsInner {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(
        rename = "_meta",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub _meta: Option<Option<String>>,
}

impl RetrieveAnEventForAProject200ResponseTagsInner {
    pub fn new() -> RetrieveAnEventForAProject200ResponseTagsInner {
        RetrieveAnEventForAProject200ResponseTagsInner {
            value: None,
            key: None,
            _meta: None,
        }
    }
}
