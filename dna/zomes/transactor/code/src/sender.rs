use hdk::holochain_json_api::json::JsonString;
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::signature::{Provenance, Signature},
    holochain_core_types::time::Timeout,
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
};

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
    let address = hdk::entry_address(&entry)?;

    let signature = hdk::sign(address)?;

    let message = MessageBody {
        transaction,
        signature,
    };

    let signature = hdk::send(
        receiver_address.clone(),
        JsonString::from(message).to_string(),
        Timeout::default(),
    )?;

    let transaction_address = utils::commit_with_provenance(
        &entry,
        Provenance::new(receiver_address, Signature::from(signature)),
    )?;

    Ok(transaction_address)
}
