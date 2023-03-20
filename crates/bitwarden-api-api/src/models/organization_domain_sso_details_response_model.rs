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
pub struct OrganizationDomainSsoDetailsResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "ssoAvailable", skip_serializing_if = "Option::is_none")]
    pub sso_available: Option<bool>,
    #[serde(rename = "domainName", skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(
        rename = "organizationIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_identifier: Option<String>,
    #[serde(rename = "ssoRequired", skip_serializing_if = "Option::is_none")]
    pub sso_required: Option<bool>,
    #[serde(rename = "verifiedDate", skip_serializing_if = "Option::is_none")]
    pub verified_date: Option<String>,
}

impl OrganizationDomainSsoDetailsResponseModel {
    pub fn new() -> OrganizationDomainSsoDetailsResponseModel {
        OrganizationDomainSsoDetailsResponseModel {
            object: None,
            sso_available: None,
            domain_name: None,
            organization_identifier: None,
            sso_required: None,
            verified_date: None,
        }
    }
}