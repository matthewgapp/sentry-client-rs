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
pub struct RegisterANewServiceHookRequest {
    /// The URL for the webhook.
    #[serde(rename = "url")]
    pub url: String,
    /// The events to subscribe to.
    #[serde(rename = "events")]
    pub events: Vec<String>,
}

impl RegisterANewServiceHookRequest {
    pub fn new(url: String, events: Vec<String>) -> RegisterANewServiceHookRequest {
        RegisterANewServiceHookRequest { url, events }
    }
}
