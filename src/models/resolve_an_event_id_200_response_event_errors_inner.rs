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
pub struct ResolveAnEventId200ResponseEventErrorsInner {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ResolveAnEventId200ResponseEventErrorsInner {
    pub fn new() -> ResolveAnEventId200ResponseEventErrorsInner {
        ResolveAnEventId200ResponseEventErrorsInner {
            message: None,
            r#type: None,
            data: None,
        }
    }
}
