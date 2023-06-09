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
pub struct RetrieveAnIssue200ResponseFirstRelease {
    #[serde(rename = "authors", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(rename = "commitCount", skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<i32>,
    #[serde(
        rename = "data",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<Option<serde_json::Value>>,
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(
        rename = "dateReleased",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_released: Option<Option<String>>,
    #[serde(rename = "deployCount", skip_serializing_if = "Option::is_none")]
    pub deploy_count: Option<i32>,
    #[serde(rename = "firstEvent", skip_serializing_if = "Option::is_none")]
    pub first_event: Option<String>,
    #[serde(
        rename = "lastCommit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_commit: Option<Option<String>>,
    #[serde(
        rename = "lastDeploy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_deploy: Option<Option<String>>,
    #[serde(rename = "lastEvent", skip_serializing_if = "Option::is_none")]
    pub last_event: Option<String>,
    #[serde(rename = "newGroups", skip_serializing_if = "Option::is_none")]
    pub new_groups: Option<i32>,
    #[serde(
        rename = "owner",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner: Option<Option<String>>,
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<crate::models::RetrieveAProject200ResponseLatestReleaseProjectsInner>>,
    #[serde(
        rename = "ref",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#ref: Option<Option<String>>,
    #[serde(rename = "shortVersion", skip_serializing_if = "Option::is_none")]
    pub short_version: Option<String>,
    #[serde(
        rename = "url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<Option<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl RetrieveAnIssue200ResponseFirstRelease {
    pub fn new() -> RetrieveAnIssue200ResponseFirstRelease {
        RetrieveAnIssue200ResponseFirstRelease {
            authors: None,
            commit_count: None,
            data: None,
            date_created: None,
            date_released: None,
            deploy_count: None,
            first_event: None,
            last_commit: None,
            last_deploy: None,
            last_event: None,
            new_groups: None,
            owner: None,
            projects: None,
            r#ref: None,
            short_version: None,
            url: None,
            version: None,
        }
    }
}
