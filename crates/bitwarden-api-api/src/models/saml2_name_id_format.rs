/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum Saml2NameIdFormat {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
}

impl ToString for Saml2NameIdFormat {
    fn to_string(&self) -> String {
        match self {
            Self::_0 => String::from("0"),
            Self::_1 => String::from("1"),
            Self::_2 => String::from("2"),
            Self::_3 => String::from("3"),
            Self::_4 => String::from("4"),
            Self::_5 => String::from("5"),
            Self::_6 => String::from("6"),
            Self::_7 => String::from("7"),
            Self::_8 => String::from("8"),
        }
    }
}

impl Default for Saml2NameIdFormat {
    fn default() -> Saml2NameIdFormat {
        Self::_0
    }
}