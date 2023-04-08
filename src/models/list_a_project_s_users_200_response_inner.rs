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
pub struct ListAProjectSUsers200ResponseInner {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "username", deserialize_with = "Option::deserialize")]
    pub username: Option<String>,
    #[serde(rename = "email", deserialize_with = "Option::deserialize")]
    pub email: Option<String>,
}

impl ListAProjectSUsers200ResponseInner {
    pub fn new(
        date_created: String,
        id: String,
        username: Option<String>,
        email: Option<String>,
    ) -> ListAProjectSUsers200ResponseInner {
        ListAProjectSUsers200ResponseInner {
            date_created,
            id,
            username,
            email,
        }
    }
}
