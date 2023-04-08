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
pub struct RetrieveAProject200ResponseLatestRelease {
    #[serde(rename = "authors")]
    pub authors: Vec<crate::models::RetrieveAProject200ResponseLatestReleaseAuthorsInner>,
    #[serde(rename = "commitCount")]
    pub commit_count: i32,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateReleased", deserialize_with = "Option::deserialize")]
    pub date_released: Option<String>,
    #[serde(rename = "deployCount")]
    pub deploy_count: i32,
    #[serde(rename = "firstEvent", deserialize_with = "Option::deserialize")]
    pub first_event: Option<String>,
    #[serde(rename = "lastCommit", deserialize_with = "Option::deserialize")]
    pub last_commit: Option<serde_json::Value>,
    #[serde(rename = "lastDeploy", deserialize_with = "Option::deserialize")]
    pub last_deploy: Option<serde_json::Value>,
    #[serde(rename = "lastEvent", deserialize_with = "Option::deserialize")]
    pub last_event: Option<String>,
    #[serde(rename = "newGroups")]
    pub new_groups: i32,
    #[serde(rename = "owner", deserialize_with = "Option::deserialize")]
    pub owner: Option<String>,
    #[serde(rename = "projects")]
    pub projects: Vec<crate::models::RetrieveAProject200ResponseLatestReleaseProjectsInner>,
    #[serde(rename = "ref", deserialize_with = "Option::deserialize")]
    pub r#ref: Option<String>,
    #[serde(rename = "shortVersion")]
    pub short_version: String,
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    #[serde(rename = "version")]
    pub version: String,
}

impl RetrieveAProject200ResponseLatestRelease {
    pub fn new(
        authors: Vec<crate::models::RetrieveAProject200ResponseLatestReleaseAuthorsInner>,
        commit_count: i32,
        data: serde_json::Value,
        date_created: String,
        date_released: Option<String>,
        deploy_count: i32,
        first_event: Option<String>,
        last_commit: Option<serde_json::Value>,
        last_deploy: Option<serde_json::Value>,
        last_event: Option<String>,
        new_groups: i32,
        owner: Option<String>,
        projects: Vec<crate::models::RetrieveAProject200ResponseLatestReleaseProjectsInner>,
        r#ref: Option<String>,
        short_version: String,
        url: Option<String>,
        version: String,
    ) -> RetrieveAProject200ResponseLatestRelease {
        RetrieveAProject200ResponseLatestRelease {
            authors,
            commit_count,
            data,
            date_created,
            date_released,
            deploy_count,
            first_event,
            last_commit,
            last_deploy,
            last_event,
            new_groups,
            owner,
            projects,
            r#ref,
            short_version,
            url,
            version,
        }
    }
}
