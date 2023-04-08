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
pub struct ResolveAShortId200ResponseGroup {
    #[serde(rename = "lastSeen")]
    pub last_seen: String,
    #[serde(rename = "numComments")]
    pub num_comments: i32,
    #[serde(rename = "userCount")]
    pub user_count: i32,
    #[serde(rename = "culprit", deserialize_with = "Option::deserialize")]
    pub culprit: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "assignedTo", deserialize_with = "Option::deserialize")]
    pub assigned_to: Option<Box<crate::models::ResolveAShortId200ResponseGroupAssignedTo>>,
    #[serde(rename = "logger", deserialize_with = "Option::deserialize")]
    pub logger: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "annotations")]
    pub annotations: Vec<String>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::ResolveAShortId200ResponseGroupMetadata>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "subscriptionDetails", deserialize_with = "Option::deserialize")]
    pub subscription_details: Option<Box<crate::models::ResolveAShortId200ResponseGroupSubscriptionDetails>>,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "hasSeen")]
    pub has_seen: bool,
    #[serde(rename = "shortId")]
    pub short_id: String,
    #[serde(rename = "shareId", deserialize_with = "Option::deserialize")]
    pub share_id: Option<String>,
    #[serde(rename = "firstSeen")]
    pub first_seen: String,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "permalink")]
    pub permalink: String,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: bool,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "project")]
    pub project: Box<crate::models::ResolveAShortId200ResponseGroupProject>,
    #[serde(rename = "statusDetails")]
    pub status_details: serde_json::Value,
}

impl ResolveAShortId200ResponseGroup {
    pub fn new(last_seen: String, num_comments: i32, user_count: i32, culprit: Option<String>, title: String, id: String, assigned_to: Option<crate::models::ResolveAShortId200ResponseGroupAssignedTo>, logger: Option<String>, r#type: String, annotations: Vec<String>, metadata: crate::models::ResolveAShortId200ResponseGroupMetadata, status: Status, subscription_details: Option<crate::models::ResolveAShortId200ResponseGroupSubscriptionDetails>, is_public: bool, has_seen: bool, short_id: String, share_id: Option<String>, first_seen: String, count: String, permalink: String, level: String, is_subscribed: bool, is_bookmarked: bool, project: crate::models::ResolveAShortId200ResponseGroupProject, status_details: serde_json::Value) -> ResolveAShortId200ResponseGroup {
        ResolveAShortId200ResponseGroup {
            last_seen,
            num_comments,
            user_count,
            culprit,
            title,
            id,
            assigned_to: if let Some(x) = assigned_to {Some(Box::new(x))} else {None},
            logger,
            r#type,
            annotations,
            metadata: Box::new(metadata),
            status,
            subscription_details: if let Some(x) = subscription_details {Some(Box::new(x))} else {None},
            is_public,
            has_seen,
            short_id,
            share_id,
            first_seen,
            count,
            permalink,
            level,
            is_subscribed,
            is_bookmarked,
            project: Box::new(project),
            status_details,
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

