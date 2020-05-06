use crate::{
    message::{send_message, Message, MessageBody},
    offer,
    offer::OfferState,
    transaction,
};
use hdk::prelude::*;

/**
 * Cancels the local offer, notifying the counterparty of the transaction
 */
pub fn send_cancel_offer(transaction_address: &Address) -> ZomeApiResult<()> {
    let offer = offer::query_offer(transaction_address)?;

    // If offer had been completed we cannot cancel it
    match offer.state {
        OfferState::Completed { .. } => Err(ZomeApiError::from(String::from(
            "Cannot cancel offer since it has already been completed",
        ))),
        _ => Ok(()),
    }?;

    offer::cancel_offer(transaction_address)?;

    let counterparty = transaction::get_counterparty(&offer.transaction);

    let message = MessageBody::CancelOffer(Message::Request(transaction_address.clone()));

    let response = send_message(counterparty, message)?;

    match response {
        MessageBody::CancelOffer(Message::Response(())) => Ok(()),
        _ => Err(ZomeApiError::from(String::from(
            "There was an error canceling the offer",
        ))),
    }
}

/**
 * Handles an inbound cancel offer request, rejecting if it was already completed
 */
pub fn handle_cancel_offer(transaction_address: &Address) -> ZomeApiResult<()> {
    let offer = offer::query_offer(transaction_address)?;

    // If offer had been completed we cannot cancel it
    match offer.state {
        OfferState::Completed { .. } => Err(ZomeApiError::from(String::from(
            "Cannot cancel offer since it has already been completed",
        ))),
        _ => Ok(()),
    }?;

    offer::cancel_offer(transaction_address)?;

    hdk::emit_signal(
        "offer-canceled",
        JsonString::from_json(&format!("{{\"transaction_address\": \"{}\"}}", transaction_address)),
    )?;

    Ok(())
}
