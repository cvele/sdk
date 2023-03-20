/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationConnectionResponseModel {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::OrganizationConnectionType>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

impl OrganizationConnectionResponseModel {
    pub fn new() -> OrganizationConnectionResponseModel {
        OrganizationConnectionResponseModel {
            id: None,
            _type: None,
            organization_id: None,
            enabled: None,
            config: None,
        }
    }
}