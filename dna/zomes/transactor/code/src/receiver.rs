use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::signature::{Provenance, Signature},
    holochain_persistence_api::cas::content::Address,
};

use crate::message::MessageBody;
use crate::transaction;
use crate::utils;

pub fn validate_and_commit_transaction(
    address: Address,
    message: MessageBody,
) -> ZomeApiResult<String> {
    let entry = transaction::transaction_entry(&message.transaction);

    let entry_address = hdk::entry_address(&entry)?;

    let provenance = Provenance::new(address.clone(), Signature::from(message.signature));

    let valid = hdk::verify_signature(provenance.clone(), entry_address.clone())?;

    if !valid {
        return Err(ZomeApiError::from(String::from("Signature not valid")));
    }

    utils::commit_with_provenance(&entry, provenance)?;

    hdk::sign(entry_address)
}
