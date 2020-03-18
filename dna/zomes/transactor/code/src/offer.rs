use crate::message::{send_message, Message, GetTransactionsResponse, MessageBody};
use crate::transaction;
use crate::transaction::{Transaction, TransactionsSnapshot};
use hdk::entry_definition::ValidatingEntryType;
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
pub struct OfferBalance {
    sender_balance: f64,
    executable: bool,
    last_header_address: Address,
}

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

    let message_body = MessageBody::SendOffer(Message::Request(offer));

    let result = send_message(receiver_address, message_body)?;

    match result {
        MessageBody::SendOffer(Message::Response(())) => hdk::commit_entry(&offer.entry()),
        _ => Err(ZomeApiError::from(format!(
            "Received error when offering credits, {:?}",
            result
        ))),
    }
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
 * Returns the offer balance, whether it's executable, and the last_header_address of the chain of the agent that made the offer
 */
pub fn get_offer_balance(offer_address: Address) -> ZomeApiResult<OfferBalance> {
    let offer = get_offer(&offer_address)?;

    let transactions_snapshot = match offer.sender_address == AGENT_ADDRESS.clone() {
        true => Err(ZomeApiError::from(format!("wait for it"))),
        false => request_sender_transactions(&offer_address, &offer.sender_address)
    }?;

    let balance = transaction::compute_balance(&offer.sender_address, transactions_snapshot.transactions);

    let next_transaction = Transaction {
        sender_address: offer.sender_address,
        receiver_address: offer.receiver_address,
        amount: offer.amount,
        timestamp: 0
    };

    let executable = transaction::are_transactions_valid(&offer.sender_address, transactions_snapshot.transactions)?;
    
    Ok(OfferBalance {
        sender_balance: balance,
        executable,
        last_header_address: transactions_snapshot.last_header_address
    })
}

/**
 * Requests the transactions for the given offer_address from the sender_address agent, requesting their last header address for later validation
 */
pub fn request_sender_transactions(
    offer_address: &Address,
    sender_address: &Address,
) -> ZomeApiResult<TransactionsSnapshot> {
    let message = MessageBody::GetTransactions(Message::Request(offer_address.clone()));

    let result = send_message(sender_address.clone(), message)?;

    let response = match result {
        MessageBody::GetTransactions(Message::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "Error getting the transaction for agent {}",
            sender_address
        ))),
    }?;

    match response {
        GetTransactionsResponse::Transactions(transactions_snapshot) => Ok(transactions_snapshot),
        GetTransactionsResponse::OfferWasCanceled => {
            cancel_offer(offer_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
    }
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