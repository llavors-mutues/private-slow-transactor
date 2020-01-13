use hdk::holochain_json_api::json::JsonString;
use hdk::prelude::Entry;
use hdk::prelude::{QueryArgsOptions, QueryResult};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        chain_header::ChainHeader,
        signature::{Provenance, Signature},
        time::Timeout,
    },
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
};
use holochain_wasm_utils::api_serialization::query::QueryArgsNames;

use crate::attestation::{attestation_entry, create_initial_attestation, Attestation};
use crate::message::MessageBody;
use crate::transaction::{transaction_entry, Transaction};
use crate::utils;

pub fn send_amout(
    receiver_address: Address,
    amount: usize,
    timestamp: usize,
) -> ZomeApiResult<Address> {
    let transaction = Transaction {
        sender_address: AGENT_ADDRESS.clone(),
        receiver_address: receiver_address.clone(),
        amount,
        timestamp,
    };

    let entry = transaction_entry(&transaction);
    let transaction_address = hdk::entry_address(&entry)?;

    let signature = hdk::sign(transaction_address.clone())?;

    let transactions = get_own_transactions()?;

    let message = MessageBody {
        transaction,
        signature,
        chain_entries: transactions,
    };

    let signature = hdk::send(
        receiver_address.clone(),
        JsonString::from(message).to_string(),
        Timeout::default(),
    )?;

    if signature.contains("Error") {
        return Err(ZomeApiError::from(String::from(
            "Error sending the transaction",
        )));
    }

    let attestation_address = create_initial_attestation()?;
    let new_attestation = Attestation::from(transaction_address.clone());

    hdk::update_entry(attestation_entry(new_attestation), &attestation_address)?;

    let transaction_address = utils::commit_with_provenance(
        &entry,
        Provenance::new(receiver_address, Signature::from(signature)),
    )?;

    Ok(transaction_address)
}

pub fn get_own_transactions() -> ZomeApiResult<Vec<(ChainHeader, Entry)>> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: true,
        entries: true,
    };

    let result = hdk::query_result(QueryArgsNames::from("transaction"), options)?;

    match result {
        QueryResult::HeadersWithEntries(entries) => Ok(entries),
        _ => Err(ZomeApiError::from(String::from(
            "Error when getting own transactions",
        ))),
    }
}
