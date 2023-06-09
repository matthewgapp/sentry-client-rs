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
pub struct RetrieveAnEventForAProject200ResponseMeta {
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<String>>,
    #[serde(
        rename = "context",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<Option<String>>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<serde_json::Value>,
    #[serde(
        rename = "contexts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub contexts: Option<Option<String>>,
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Option<String>>,
    #[serde(
        rename = "packages",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub packages: Option<Option<String>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(
        rename = "sdk",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sdk: Option<Option<String>>,
}

impl RetrieveAnEventForAProject200ResponseMeta {
    pub fn new() -> RetrieveAnEventForAProject200ResponseMeta {
        RetrieveAnEventForAProject200ResponseMeta {
            user: None,
            context: None,
            entries: None,
            contexts: None,
            message: None,
            packages: None,
            tags: None,
            sdk: None,
        }
    }
}
