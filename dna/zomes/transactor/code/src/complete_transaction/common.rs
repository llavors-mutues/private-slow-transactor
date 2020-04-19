use crate::{
    attestation::{validate_transaction_headers, Attestation},
    offer, transaction,
    transaction::Transaction,
    utils,
};
use hdk::holochain_core_types::{chain_header::ChainHeader, signature::Signature};
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

/**
 * Validates that the last header hasn't changed from the given address
 */
pub fn validate_last_header_still_unchanged(last_header_address: Address) -> ZomeApiResult<()> {
    let last_header = utils::get_my_last_header()?;

    match last_header.address() == last_header_address {
        true => Ok(()),
        false => Err(ZomeApiError::from(format!("Last header has changed"))),
    }
}

/**
 * Builds and creates the attestation from the given headers
 */
pub fn create_attestation(
    chain_headers: &Vec<ChainHeader>,
    _counterparty_signature: &Signature,
) -> ZomeApiResult<Address> {
    validate_transaction_headers(&chain_headers)?;

    let attestation = Attestation::from_headers(chain_headers);
    let attestation_address = hdk::commit_entry(&attestation.entry())?;

    for header in chain_headers {
        hdk::link_entries(
            &header.provenances()[0].source(),
            &attestation_address,
            "agent->attestation",
            "",
        )?;
    }

    offer::complete_offer(chain_headers[0].entry_address(), &attestation_address)?;

    Ok(attestation_address)
}

/**
 * Validates the given counterparty header against the actual attestation and the approved header address
 */
pub fn validate_counterparty_header(
    counterparty_header: &ChainHeader,
    transaction: &Transaction,
    approved_header_address: &Option<Address>,
) -> ZomeApiResult<()> {
    if let Some(link) = counterparty_header.link() {
        if let Some(header_address) = approved_header_address {
            if link != header_address.clone() {
                return Err(ZomeApiError::from(String::from("Bad transaction header: the previous header address is not equal to the approved one")));
            }
        }
    } else {
        return Err(ZomeApiError::from(String::from(
            "Bad transaction header: the previous header address is None",
        )));
    }

    if counterparty_header.entry_address().clone() != transaction.address()? {
        return Err(ZomeApiError::from(String::from(
            "Bad transaction header: entry address does not correspond to the transaction offer",
        )));
    }

    let counterparty = transaction::get_counterparty(&transaction);

    if counterparty_header.provenances()[0].source() != counterparty {
        return Err(ZomeApiError::from(String::from(
            "Bad transaction header: author is not the transaction counterparty",
        )));
    }

    Ok(())
}
