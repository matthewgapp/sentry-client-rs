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
pub struct ListAProjectSIssues200ResponseInnerMetadata {
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl ListAProjectSIssues200ResponseInnerMetadata {
    pub fn new(filename: String, r#type: String, value: String, title: String) -> ListAProjectSIssues200ResponseInnerMetadata {
        ListAProjectSIssues200ResponseInnerMetadata {
            filename,
            r#type,
            value,
            title,
        }
    }
}

