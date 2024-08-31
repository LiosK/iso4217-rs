//! Prints the List One in JSON.

use std::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let xml_text = ureq::get(iso4217::LIST_ONE_URL).call()?.into_string()?;
    let list_one = iso4217::deserialize_list_one(&xml_text)?;

    let entries: Vec<_> = list_one.entries().iter().map(to_serializable).collect();

    let mut writer = io::BufWriter::new(io::stdout());
    serde_json::to_writer(&mut writer, &entries)?;

    Ok(())
}

fn to_serializable<'a>(entry: &'a iso4217::CcyNtry) -> impl serde::Serialize + 'a {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct SerializableEntry<'a> {
        ctry_nm: &'a str,
        ccy_nm: &'a str,
        is_fund: bool,
        ccy: Option<&'a str>,
        ccy_nbr: Option<&'a str>,
        ccy_mnr_unts: Option<i32>,
    }

    SerializableEntry {
        ctry_nm: entry.ctry_nm(),
        ccy_nm: entry.ccy_nm(),
        is_fund: entry.is_fund(),
        ccy: entry.ccy(),
        ccy_nbr: entry.ccy_nbr(),
        ccy_mnr_unts: entry.ccy_mnr_unts(),
    }
}
