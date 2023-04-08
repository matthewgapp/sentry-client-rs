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
pub struct UpdateAnIssue200Response {
    #[serde(rename = "annotations")]
    pub annotations: Vec<String>,
    #[serde(rename = "assignedTo", deserialize_with = "Option::deserialize")]
    pub assigned_to: Option<serde_json::Value>,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "culprit")]
    pub culprit: String,
    #[serde(rename = "firstSeen")]
    pub first_seen: String,
    #[serde(rename = "hasSeen")]
    pub has_seen: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: bool,
    #[serde(rename = "lastSeen")]
    pub last_seen: String,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "logger", deserialize_with = "Option::deserialize")]
    pub logger: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::ListAProjectSIssues200ResponseInnerMetadata>,
    #[serde(rename = "numComments")]
    pub num_comments: i32,
    #[serde(rename = "permalink")]
    pub permalink: String,
    #[serde(rename = "project")]
    pub project: Box<crate::models::ListAProjectSIssues200ResponseInnerProject>,
    #[serde(rename = "shareId", deserialize_with = "Option::deserialize")]
    pub share_id: Option<String>,
    #[serde(rename = "shortId")]
    pub short_id: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "statusDetails")]
    pub status_details: serde_json::Value,
    #[serde(rename = "subscriptionDetails", deserialize_with = "Option::deserialize")]
    pub subscription_details: Option<serde_json::Value>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "userCount")]
    pub user_count: i32,
}

impl UpdateAnIssue200Response {
    pub fn new(annotations: Vec<String>, assigned_to: Option<serde_json::Value>, count: String, culprit: String, first_seen: String, has_seen: bool, id: String, is_bookmarked: bool, is_public: bool, is_subscribed: bool, last_seen: String, level: String, logger: Option<String>, metadata: crate::models::ListAProjectSIssues200ResponseInnerMetadata, num_comments: i32, permalink: String, project: crate::models::ListAProjectSIssues200ResponseInnerProject, share_id: Option<String>, short_id: String, status: Status, status_details: serde_json::Value, subscription_details: Option<serde_json::Value>, title: String, r#type: String, user_count: i32) -> UpdateAnIssue200Response {
        UpdateAnIssue200Response {
            annotations,
            assigned_to,
            count,
            culprit,
            first_seen,
            has_seen,
            id,
            is_bookmarked,
            is_public,
            is_subscribed,
            last_seen,
            level,
            logger,
            metadata: Box::new(metadata),
            num_comments,
            permalink,
            project: Box::new(project),
            share_id,
            short_id,
            status,
            status_details,
            subscription_details,
            title,
            r#type,
            user_count,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "unresolved")]
    Unresolved,
    #[serde(rename = "ignored")]
    Ignored,
}

impl Default for Status {
    fn default() -> Status {
        Self::Resolved
    }
}

