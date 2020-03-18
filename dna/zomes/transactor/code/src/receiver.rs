use hdk::holochain_json_api::json::JsonString;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::signature::{Provenance, Signature},
    holochain_persistence_api::cas::content::Address,
};
use std::convert::TryInto;

use crate::attestation::{attestation_entry, Attestation};
use crate::message::{Message, MessageBody};
use crate::offer;
use crate::transaction::transaction_entry;
use crate::utils;

/**
 * Receive message, recognizing the type of message and executing the appropriate actions
 */
pub fn receive_message(sender_address: Address, message: String) -> String {
    let success: Result<MessageBody, _> = JsonString::from_json(&message).try_into();
    let response = match success {
        Err(err) => Err(ZomeApiError::from(format!(
            "Error deserializing the message: {:?}",
            err
        ))),
        Ok(message_body) => match message_body {
            MessageBody::SendOffer(Message::Request(offer)) => offer::receive_offer(offer)
                .map(|result| MessageBody::SendOffer(Message::Response(result))),
            _ => Err(ZomeApiError::from(format!("Bad message type"))),
        },
    };

    let json: JsonString = response.into();
    json.to_string()
}

// -----------
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
    let dht_transactions_addresses = get_agent_transaction_addresses_from_dht(&address)?;

    let mut old_transactions = message.old_transactions;

    let source_chain_transaction_addresses = old_transactions
        .clone()
        .into_iter()
        .map(|transaction| hdk::entry_address(&transaction_entry(&transaction)).unwrap())
        .collect();

    // Validate that the attestations from the agents are the only transactions present in the given source chain
    validate_transactions_against_attestations(
        &dht_transactions_addresses,
        &source_chain_transaction_addresses,
    )?;

    // Filter the transactions from the source chain entries
    old_transactions.push(message.transaction);

    // Validate that the transactions are valid
    validate_transactions(&address, old_transactions)?;

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
    source_chain_addresses: &Vec<Address>,
) -> ZomeApiResult<()> {
    if attestation_transaction_addresses.len() != source_chain_addresses.len() {
        return Err(ZomeApiError::from(String::from(
            "Chain entries received from the sender do not match the attestation entries",
        )));
    }

    for i in 0..source_chain_addresses.len() {
        if attestation_transaction_addresses.get(i) != source_chain_addresses.get(i) {
            return Err(ZomeApiError::from(String::from(
                "Chain entries received from the sender do not match the attestation entries",
            )));
        }
    }

    Ok(())
}
