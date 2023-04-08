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
pub struct RetrieveAProject200ResponsePluginsInner {
    #[serde(rename = "assets")]
    pub assets: Vec<String>,
    #[serde(rename = "author", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub author: Option<Option<Box<crate::models::RetrieveAProject200ResponsePluginsInnerAuthor>>>,
    #[serde(rename = "canDisable")]
    pub can_disable: bool,
    #[serde(rename = "contexts")]
    pub contexts: Vec<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "doc")]
    pub doc: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "hasConfiguration")]
    pub has_configuration: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isTestable")]
    pub is_testable: bool,
    #[serde(rename = "metadata")]
    pub metadata: serde_json::Value,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "resourceLinks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resource_links: Option<Option<Vec<crate::models::RetrieveAProject200ResponsePluginsInnerResourceLinksInner>>>,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub version: Option<Option<String>>,
}

impl RetrieveAProject200ResponsePluginsInner {
    pub fn new(assets: Vec<String>, can_disable: bool, contexts: Vec<String>, doc: String, enabled: bool, has_configuration: bool, id: String, is_testable: bool, metadata: serde_json::Value, name: String, short_name: String, slug: String, status: String, r#type: String) -> RetrieveAProject200ResponsePluginsInner {
        RetrieveAProject200ResponsePluginsInner {
            assets,
            author: None,
            can_disable,
            contexts,
            description: None,
            doc,
            enabled,
            has_configuration,
            id,
            is_testable,
            metadata,
            name,
            resource_links: None,
            short_name,
            slug,
            status,
            r#type,
            version: None,
        }
    }
}


