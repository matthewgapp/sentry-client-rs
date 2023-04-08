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
pub struct ListAProjectSServiceHooks200ResponseInner {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "events")]
    pub events: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl ListAProjectSServiceHooks200ResponseInner {
    pub fn new(date_created: String, events: Vec<String>, id: String, secret: String, status: String, url: String) -> ListAProjectSServiceHooks200ResponseInner {
        ListAProjectSServiceHooks200ResponseInner {
            date_created,
            events,
            id,
            secret,
            status,
            url,
        }
    }
}


