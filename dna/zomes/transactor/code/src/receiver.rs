use hdk::prelude::Entry;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        chain_header::ChainHeader,
        signature::{Provenance, Signature},
    },
    holochain_persistence_api::cas::content::Address,
};

use crate::attestation::{attestation_entry, Attestation};
use crate::message::MessageBody;
use crate::transaction::{get_transactions, transaction_entry, validate_transactions};
use crate::utils;

pub fn validate_and_commit_transaction(
    address: Address,
    message: MessageBody,
) -> ZomeApiResult<String> {
    // Verify signature for this transaction
    let entry = transaction_entry(&message.transaction);
    let entry_address = hdk::entry_address(&entry)?;

    let provenance = Provenance::new(address.clone(), Signature::from(message.signature));
    let valid = hdk::verify_signature(provenance.clone(), entry_address.clone())?;

    if !valid {
        return Err(ZomeApiError::from(String::from("Signature not valid")));
    }

    // Get the attestations addresses from the DHT
    let transactions_addresses = get_agent_transaction_addresses_from_dht(&address)?;

    // Validate that the attestations from the agents are the only transactions present in the given source chain
    validate_transactions_against_attestations(&transactions_addresses, &message.chain_entries)?;

    let entries: Vec<Entry> = message
        .chain_entries
        .into_iter()
        .map(|chain_entry| chain_entry.1)
        .collect();

    // Filter the transactions from the source chain entries
    let mut transactions = get_transactions(&entries);

    transactions.push(message.transaction);

    // Validate that the transactions are valid
    validate_transactions(&address, transactions)?;

    // Commit the transaction and return our own signature
    let signature = hdk::sign(entry_address)?;

    utils::commit_with_provenance(&entry, provenance)?;

    Ok(signature)
}

pub fn get_agent_transaction_addresses_from_dht(
    agent_address: &Address,
) -> ZomeApiResult<Vec<Address>> {
    let attestation = Attestation::initial(agent_address);

    let entry = attestation_entry(attestation);

    let initial_address = hdk::entry_address(&entry)?;

    let history = hdk::get_entry_history(&initial_address)?;
    if let None = history {
        let vector: Vec<Address> = vec![];
        return Ok(vector);
    }
    let transaction_addresses: Vec<Address> = history
        .unwrap()
        .items
        .into_iter()
        .filter_map(|item| {
            let attestation = Attestation::from_entry(&item.entry.unwrap()).unwrap();

            attestation.last_transaction_address
        })
        .collect();

    Ok(transaction_addresses)
}

pub fn validate_transactions_against_attestations(
    attestation_transaction_addresses: &Vec<Address>,
    chain_entries: &Vec<(ChainHeader, Entry)>,
) -> ZomeApiResult<()> {
    if attestation_transaction_addresses.len() != chain_entries.len() {
        return Err(ZomeApiError::from(String::from(
            "Chain entries received from the sender do not match the attestation entries",
        )));
    }

    for i in 0..chain_entries.len() {
        if attestation_transaction_addresses.get(i).unwrap()
            != chain_entries.get(i).unwrap().0.entry_address()
        {
            return Err(ZomeApiError::from(String::from(
                "Chain entries received from the sender do not match the attestation entries",
            )));
        }
    }

    Ok(())
}
