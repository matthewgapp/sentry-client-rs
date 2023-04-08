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
pub struct ResolveAShortId200ResponseGroupMetadata {
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl ResolveAShortId200ResponseGroupMetadata {
    pub fn new() -> ResolveAShortId200ResponseGroupMetadata {
        ResolveAShortId200ResponseGroupMetadata {
            function: None,
            title: None,
            r#type: None,
            value: None,
            filename: None,
        }
    }
}

