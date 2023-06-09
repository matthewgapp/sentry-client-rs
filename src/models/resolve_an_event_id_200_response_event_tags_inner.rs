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
pub struct ResolveAnEventId200ResponseEventTagsInner {
    #[serde(
        rename = "_meta",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub _meta: Option<Option<String>>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ResolveAnEventId200ResponseEventTagsInner {
    pub fn new() -> ResolveAnEventId200ResponseEventTagsInner {
        ResolveAnEventId200ResponseEventTagsInner {
            _meta: None,
            key: None,
            value: None,
        }
    }
}
