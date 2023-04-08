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
pub struct ListAProjectSUserFeedback200ResponseInner {
    #[serde(rename = "comments")]
    pub comments: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "event")]
    pub event: Box<crate::models::ListAProjectSUserFeedback200ResponseInnerEvent>,
    #[serde(rename = "eventID")]
    pub event_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "issue", deserialize_with = "Option::deserialize")]
    pub issue: Option<serde_json::Value>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<serde_json::Value>,
}

impl ListAProjectSUserFeedback200ResponseInner {
    pub fn new(comments: String, date_created: String, email: String, event: crate::models::ListAProjectSUserFeedback200ResponseInnerEvent, event_id: String, id: String, issue: Option<serde_json::Value>, name: String, user: Option<serde_json::Value>) -> ListAProjectSUserFeedback200ResponseInner {
        ListAProjectSUserFeedback200ResponseInner {
            comments,
            date_created,
            email,
            event: Box::new(event),
            event_id,
            id,
            issue,
            name,
            user,
        }
    }
}

