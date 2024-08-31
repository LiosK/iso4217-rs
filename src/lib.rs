//! Deserializes the ISO 4217 currency code XML list into Rust types.

use std::borrow::Cow;

/// The URL for the List One (current currencies & funds) XML document.
pub const LIST_ONE_URL: &str ="https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml";

/// Builds a [`ListOne`] struct from a string slice containing the List One XML document.
pub fn deserialize_list_one(xml: &str) -> Result<ListOne<'_>, impl serde::de::Error> {
    quick_xml::de::from_str(xml)
}

/// The List One (current currencies & funds) XML document.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct ListOne<'d> {
    #[serde(borrow)]
    #[serde(rename = "CcyTbl")]
    ccy_tbl: CcyTbl<'d>,

    #[serde(borrow)]
    #[serde(rename = "@Pblshd")]
    pblshd: Cow<'d, str>,
}

impl ListOne<'_> {
    /// Returns the list of CcyNtry elements.
    pub fn entries(&self) -> &[CcyNtry<'_>] {
        &self.ccy_tbl.ccy_ntry
    }

    /// Returns the published date of the document.
    pub fn pblshd(&self) -> &str {
        &self.pblshd
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CcyTbl<'d> {
    #[serde(borrow)]
    ccy_ntry: Vec<CcyNtry<'d>>,
}

/// A CcyNtry element in the List One document.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CcyNtry<'d> {
    #[serde(borrow)]
    ctry_nm: Cow<'d, str>,

    #[serde(borrow)]
    ccy_nm: CcyNm<'d>,

    #[serde(borrow)]
    ccy: Option<Cow<'d, str>>,

    #[serde(borrow)]
    ccy_nbr: Option<Cow<'d, str>>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_ccy_mnr_unts")]
    ccy_mnr_unts: Option<i32>,
}

impl CcyNtry<'_> {
    /// Returns the country name.
    pub fn ctry_nm(&self) -> &str {
        &self.ctry_nm
    }

    /// Returns the currency name.
    pub fn ccy_nm(&self) -> &str {
        &self.ccy_nm.content
    }

    /// Returns true if the currency is a fund.
    pub fn is_fund(&self) -> bool {
        self.ccy_nm.is_fund
    }

    /// Returns the alphabetic code.
    pub fn ccy(&self) -> Option<&str> {
        self.ccy.as_deref()
    }

    /// Returns the numeric code.
    pub fn ccy_nbr(&self) -> Option<&str> {
        self.ccy_nbr.as_deref()
    }

    /// Returns the minor unit.
    pub fn ccy_mnr_unts(&self) -> Option<i32> {
        self.ccy_mnr_unts
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
struct CcyNm<'d> {
    #[serde(borrow)]
    #[serde(rename = "$value")]
    content: Cow<'d, str>,

    #[serde(default)]
    #[serde(rename = "@IsFund")]
    #[serde(deserialize_with = "parse_ccy_nm_is_fund")]
    is_fund: bool,
}

fn parse_ccy_mnr_unts<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;
    match Cow::<'de, str>::deserialize(deserializer)?.as_ref() {
        "N.A." => Ok(None),
        n => Ok(Some(n.parse().map_err(|_| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(n), &r#"a number or "N.A.""#)
        })?)),
    }
}

fn parse_ccy_nm_is_fund<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;
    match bool::deserialize(deserializer)? {
        true => Ok(true),
        false => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Bool(false),
            &"`true`",
        )),
    }
}
