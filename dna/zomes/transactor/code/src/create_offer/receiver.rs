use crate::{
    offer::{Offer, OfferState},
    utils::ParseableEntry,
};
use hdk::{prelude::*, AGENT_ADDRESS};

/**
 * Receive and offer, check that it's valid, and store it privately
 */
pub fn receive_offer(sender_address: Address, offer: Offer) -> ZomeApiResult<()> {
    if sender_address != offer.transaction.sender_address {
        return Err(ZomeApiError::from(format!(
            "This offer is not from the agent that sent the message"
        )));
    }

    if offer.transaction.receiver_address != AGENT_ADDRESS.clone() {
        return Err(ZomeApiError::from(format!("This offer is not for me")));
    }
    match offer.state {
        OfferState::Pending => Ok(()),
        _ => Err(ZomeApiError::from(format!("The offer must be pending"))),
    }?;

    hdk::commit_entry(&offer.entry())?;
    Ok(())
}
