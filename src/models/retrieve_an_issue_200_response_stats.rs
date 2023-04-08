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
pub struct RetrieveAnIssue200ResponseStats {
    #[serde(rename = "24h", skip_serializing_if = "Option::is_none")]
    pub param_24h: Option<Vec<Vec<f32>>>,
    #[serde(rename = "30d", skip_serializing_if = "Option::is_none")]
    pub param_30d: Option<Vec<Vec<f32>>>,
}

impl RetrieveAnIssue200ResponseStats {
    pub fn new() -> RetrieveAnIssue200ResponseStats {
        RetrieveAnIssue200ResponseStats {
            param_24h: None,
            param_30d: None,
        }
    }
}


