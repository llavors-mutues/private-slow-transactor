use crate::utils;
use hdk::{holochain_core_types::chain_header::ChainHeader, prelude::*};
use holochain_wasm_utils::api_serialization::{QueryArgsNames, QueryArgsOptions, QueryResult};
use std::convert::TryFrom;

pub trait ParseableEntry: TryFrom<JsonString> + Into<JsonString> {
    fn from_entry(entry: &Entry) -> Option<Self> {
        if let Entry::App(entry_type, attestation_entry) = entry {
            if entry_type.to_string() == Self::entry_type() {
                if let Ok(t) = Self::try_from(attestation_entry.clone()) {
                    return Some(t);
                }
            }
        }
        None
    }

    fn entry(self) -> Entry {
        Entry::App(Self::entry_type().into(), self.into())
    }

    fn entry_type() -> String;

    fn address(&self) -> ZomeApiResult<Address> {
        hdk::entry_address(&self.entry())
    }
}

/**
 * Retrieve all entries of the given type from the private chain
 */
pub fn query_all(entry_type: String) -> ZomeApiResult<Vec<(ChainHeader, Entry)>> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: true,
        entries: true,
    };
    let query_result = hdk::query_result(QueryArgsNames::from(vec![entry_type]), options)?;

    match query_result {
        QueryResult::HeadersWithEntries(headers_with_entries) => Ok(headers_with_entries),
        _ => Err(ZomeApiError::from(format!("Unable to get entries"))),
    }
}

/**
 * Retrieve all entries of the given type from the private chain and transform them into the given struct
 */
pub fn query_all_into<T>(entry_type: String) -> ZomeApiResult<Vec<(ChainHeader, T)>>
where
    T: ParseableEntry,
{
    let headers_with_entries = query_all(entry_type)?;
    let entry_to_parsed =
        |entry: (ChainHeader, Entry)| T::from_entry(&entry.1).map(|parsed| (entry.0, parsed));

    Ok(headers_with_entries
        .into_iter()
        .filter_map(entry_to_parsed)
        .collect())
}

/**
 * Gets the last header of my source chain
 */
pub fn get_my_last_header() -> ZomeApiResult<ChainHeader> {
    let headers_with_entries = utils::query_all(String::from("*"))?;

    headers_with_entries
        .first()
        .map(|h| h.0.clone())
        .ok_or(ZomeApiError::from(format!("Could not find header")))
}
