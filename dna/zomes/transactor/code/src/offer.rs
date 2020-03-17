use crate::message::MessageBody;
use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::time::Timeout;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::dna::entry_types::Sharing,
    holochain_core_types::entry::Entry,
    AGENT_ADDRESS,
};
use holochain_wasm_utils::api_serialization::{QueryArgsNames, QueryArgsOptions, QueryResult};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OfferExecutable {
    executable: bool,
    last_header_address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum OfferState {
    Pending,
    Declined,
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

    pub fn from_entry(entry: &Entry) -> ZomeApiResult<Offer> {
        match entry {
            Entry::App(entry_type, offer_entry) => {
                if entry_type.to_string() != "offer" {
                    return Err(ZomeApiError::from(format!("Given entry is not an offer")));
                }

                match Offer::try_from(offer_entry) {
                    Ok(t) => Ok(t),
                    _ => Err(ZomeApiError::from(format!("Given entry is not an offer"))),
                }
            }
            _ => Err(ZomeApiError::from(format!("Given entry is not an offer"))),
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
 * Sends an offer to the receiver address, and when Creates a private offer to the given receiver address, setting up the transaction
 * Also send a direct message to the receiver notifying the offer
 */
pub fn send_offer_to(receiver_address: Address, amount: f64) -> ZomeApiResult<Address> {
    let offer = Offer {
        sender_address: AGENT_ADDRESS.clone(),
        receiver_address: receiver_address.clone(),
        amount,
        state: OfferState::Pending,
    };

    let message = MessageBody::SendOffer(offer);

    let result = hdk::send(
        receiver_address,
        JsonString::from(message).to_string(),
        Timeout::default(),
    )?;

    if !result.contains("Ok") {
        return Err(ZomeApiError::from(format!(
            "Received error when offering credits, {:?}",
            result
        )));
    }

    // Create our private entry
    let offer_entry = offer.entry();

    hdk::commit_entry(&offer_entry)
}

/**
 * Receive and offer, check that it's valid, and store it privately
 */
pub fn receive_offer(offer: Offer) -> ZomeApiResult<()> {
    if offer.receiver_address != AGENT_ADDRESS.clone() {
        return Err(ZomeApiError::from(format!("This offer is not for me")));
    }
    match offer.state {
        OfferState::Pending => Ok(()),
        _ => Err(ZomeApiError::from(format!("The offer must be pending"))),
    }?;

    hdk::commit_entry(&offer.entry())?;
    Ok(())
}

/**
 * Gets the offer from the private chain
 */
pub fn get_offer(offer_address: &Address) -> ZomeApiResult<Offer> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: true,
        entries: true,
    };
    let query_result = hdk::query_result(QueryArgsNames::from(vec!["offer"]), options)?;

    match query_result {
        QueryResult::HeadersWithEntries(entries_with_headers) => {
            let entry_with_header = entries_with_headers
                .iter()
                .find(|entry_and_header| entry_and_header.0.entry_address() == offer_address);

            match entry_with_header {
                Some(offer_entry_with_header) => Offer::from_entry(&offer_entry_with_header.1),
                None => Err(ZomeApiError::from(format!("Given offer was not found"))),
            }
        }
        _ => Err(ZomeApiError::from(format!("Unable to get offers"))),
    }
}

/**
 * Returns whether the offer is executable, and the last_header_address of the chain of the agent that made the offer
 */
pub fn is_offer_executable(offer_address: Address) -> ZomeApiResult<OfferExecutable> {
    let offer = get_offer(&offer_address)?;

    // For now we assume that the offer is to another agent

    let message = MessageBody::GetTransactions {
        offer_address: offer_address.clone(),
    };

    let result = hdk::send(
        offer.sender_address,
        JsonString::from(message).to_string(),
        Timeout::default(),
    )?;

    if result.contains("Err") {
        return Err(ZomeApiError::from(format!(
            "Error getting the transactions from agent {}: {:?}",
            offer.sender_address, result
        )));
    }

    

}
