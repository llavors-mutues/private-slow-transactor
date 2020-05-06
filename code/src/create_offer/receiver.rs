use crate::{
    offer::{Offer, OfferState},
    transaction::Transaction,
};
use hdk::{prelude::*, AGENT_ADDRESS};
use holochain_entry_utils::HolochainEntry;

/**
 * Receive and offer, check that it's valid, and store it privately
 */
pub fn receive_offer(sender_address: Address, transaction: Transaction) -> ZomeApiResult<()> {
    if sender_address != transaction.debtor_address {
        return Err(ZomeApiError::from(format!(
            "This offer is not from the agent that sent the message"
        )));
    }

    if transaction.creditor_address != AGENT_ADDRESS.clone() {
        return Err(ZomeApiError::from(format!("This offer is not for me")));
    }

    let offer = Offer {
        state: OfferState::Pending,
        transaction: transaction.clone(),
    };

    hdk::commit_entry(&offer.entry())?;

    let transaction_address = transaction.address()?;
    hdk::emit_signal(
        "offer-received",
        JsonString::from_json(&format!("{{\"transaction_address\": \"{}\"}}", transaction_address)),
    )?;

    Ok(())
}
