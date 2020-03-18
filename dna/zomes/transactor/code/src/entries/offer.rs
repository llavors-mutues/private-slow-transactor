use crate::transaction::Transaction;
use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::dna::entry_types::Sharing,
    holochain_core_types::entry::Entry,
};
use holochain_wasm_utils::api_serialization::{QueryArgsNames, QueryArgsOptions, QueryResult};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum OfferState {
    Pending,
    Canceled,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Offer {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub amount: f64,
    pub state: OfferState,
}

impl Offer {
    pub fn entry(self) -> Entry {
        Entry::App("offer".into(), self.into())
    }

    pub fn from_entry(entry: &Entry) -> Option<Offer> {
        if let Entry::App(entry_type, attestation_entry) = entry {
            if entry_type.to_string() == "offer" {
                if let Ok(t) = Offer::try_from(attestation_entry) {
                    return Some(t);
                }
            }
        }
        None
    }

    pub fn to_hypotetical_transaction(&self) -> Transaction {
        Transaction {
            sender_address: self.sender_address.clone(),
            receiver_address: self.receiver_address.clone(),
            amount: self.amount.clone(),
            timestamp: 0,
        }
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "offer",
        description: "offer private entry to temporarily store the data of a transaction before accepting it",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Offer>| {
            Ok(())
        }
    )
}

/**
 * Retrieve all offers from the private chain
 */
pub fn get_all_offers() -> ZomeApiResult<Vec<(Address, Offer)>> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: false,
        entries: true,
    };
    let query_result = hdk::query_result(QueryArgsNames::from(vec!["offer"]), options)?;

    match query_result {
        QueryResult::Entries(entries) => {
            let entry_to_offer =
                |entry: (Address, Entry)| Offer::from_entry(&entry.1).map(|offer| (entry.0, offer));
            let offers = entries.into_iter().filter_map(entry_to_offer).collect();
            Ok(offers)
        }
        _ => Err(ZomeApiError::from(format!("Unable to get offers"))),
    }
}

/**
 * Gets the offer identified with the given address from the private chain
 */
pub fn get_offer(offer_address: &Address) -> ZomeApiResult<Offer> {
    let offers = get_all_offers()?;

    let maybe_offer = offers.iter().find(|offer| offer.0 == offer_address.clone());

    maybe_offer
        .map(|offer| offer.1.clone())
        .ok_or(ZomeApiError::from(format!("Given offer was not found")))
}

/**
 * Updates the private offer to a canceled state
 */
pub fn cancel_offer(offer_address: &Address) -> ZomeApiResult<()> {
    let mut offer = get_offer(offer_address)?;

    offer.state = OfferState::Canceled;

    hdk::update_entry(offer.entry(), offer_address)?;

    Ok(())
}
