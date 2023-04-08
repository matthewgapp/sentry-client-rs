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
pub struct RetrieveAnEventForAProject200Response {
    #[serde(rename = "eventID")]
    pub event_id: String,
    #[serde(rename = "dist", deserialize_with = "Option::deserialize")]
    pub dist: Option<String>,
    #[serde(rename = "userReport", deserialize_with = "Option::deserialize")]
    pub user_report: Option<serde_json::Value>,
    #[serde(rename = "previousEventID", deserialize_with = "Option::deserialize")]
    pub previous_event_id: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::ResolveAnEventId200ResponseEventErrorsInner>,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "nextEventID", deserialize_with = "Option::deserialize")]
    pub next_event_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::RetrieveAnEventForAProject200ResponseMetadata>,
    #[serde(rename = "tags")]
    pub tags: Vec<crate::models::RetrieveAnEventForAProject200ResponseTagsInner>,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateReceived")]
    pub date_received: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<crate::models::ResolveAnEventId200ResponseEventUser>>,
    #[serde(rename = "entries")]
    pub entries: Vec<crate::models::ResolveAnEventId200ResponseEventEntriesInner>,
    #[serde(rename = "packages")]
    pub packages: serde_json::Value,
    #[serde(rename = "sdk")]
    pub sdk: Box<crate::models::RetrieveAnEventForAProject200ResponseSdk>,
    #[serde(rename = "_meta")]
    pub _meta: Box<crate::models::RetrieveAnEventForAProject200ResponseMeta>,
    #[serde(rename = "contexts")]
    pub contexts: serde_json::Value,
    #[serde(rename = "fingerprints")]
    pub fingerprints: Vec<String>,
    #[serde(rename = "context")]
    pub context: serde_json::Value,
    #[serde(rename = "release", deserialize_with = "Option::deserialize")]
    pub release: Option<Box<crate::models::RetrieveAnEventForAProject200ResponseRelease>>,
    #[serde(rename = "groupID")]
    pub group_id: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl RetrieveAnEventForAProject200Response {
    pub fn new(
        event_id: String,
        dist: Option<String>,
        user_report: Option<serde_json::Value>,
        previous_event_id: Option<String>,
        message: String,
        id: String,
        size: i32,
        errors: Vec<crate::models::ResolveAnEventId200ResponseEventErrorsInner>,
        platform: String,
        next_event_id: Option<String>,
        r#type: String,
        metadata: crate::models::RetrieveAnEventForAProject200ResponseMetadata,
        tags: Vec<crate::models::RetrieveAnEventForAProject200ResponseTagsInner>,
        date_created: String,
        date_received: String,
        user: Option<crate::models::ResolveAnEventId200ResponseEventUser>,
        entries: Vec<crate::models::ResolveAnEventId200ResponseEventEntriesInner>,
        packages: serde_json::Value,
        sdk: crate::models::RetrieveAnEventForAProject200ResponseSdk,
        _meta: crate::models::RetrieveAnEventForAProject200ResponseMeta,
        contexts: serde_json::Value,
        fingerprints: Vec<String>,
        context: serde_json::Value,
        release: Option<crate::models::RetrieveAnEventForAProject200ResponseRelease>,
        group_id: String,
        title: String,
    ) -> RetrieveAnEventForAProject200Response {
        RetrieveAnEventForAProject200Response {
            event_id,
            dist,
            user_report,
            previous_event_id,
            message,
            id,
            size,
            errors,
            platform,
            next_event_id,
            r#type,
            metadata: Box::new(metadata),
            tags,
            date_created,
            date_received,
            user: if let Some(x) = user {
                Some(Box::new(x))
            } else {
                None
            },
            entries,
            packages,
            sdk: Box::new(sdk),
            _meta: Box::new(_meta),
            contexts,
            fingerprints,
            context,
            release: if let Some(x) = release {
                Some(Box::new(x))
            } else {
                None
            },
            group_id,
            title,
        }
    }
}
