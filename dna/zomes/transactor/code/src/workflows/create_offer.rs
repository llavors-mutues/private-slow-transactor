use crate::offer::*;
use crate::utils::ParseableEntry;
use crate::{message::*, transaction::Transaction};
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

/**
 * Sends an offer to the receiver address, and when Creates a private offer to the given receiver address, setting up the transaction
 * Also send a direct message to the receiver notifying the offer
 */
pub fn create_offer(
    receiver_address: Address,
    amount: f64,
    timestamp: usize,
) -> ZomeApiResult<Address> {
    let transaction = Transaction {
        sender_address: AGENT_ADDRESS.clone(),
        receiver_address: receiver_address.clone(),
        amount,
        timestamp,
    };

    let offer = Offer {
        transaction,
        state: OfferState::Pending,
    };

    let message_body = MessageBody::SendOffer(Message::Request(offer.clone()));

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
