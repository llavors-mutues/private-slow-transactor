use crate::{
    message,
    message::{Message, MessageBody},
    offer::{Offer, OfferState},
    transaction::Transaction,
};
use hdk::{prelude::*, AGENT_ADDRESS};
use holochain_entry_utils::HolochainEntry;

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
        debtor_address: AGENT_ADDRESS.clone(),
        creditor_address: receiver_address.clone(),
        amount,
        timestamp,
    };

    let offer = Offer {
        transaction: transaction.clone(),
        state: OfferState::Approved {
            approved_header_address: None,
        },
    };

    let message_body = MessageBody::SendOffer(Message::Request(offer.clone()));

    let result = message::send_message(receiver_address, message_body)?;

    match result {
        MessageBody::SendOffer(Message::Response(())) => {
            hdk::commit_entry(&offer.entry())?;
            Ok(transaction.address()?)
        }
        _ => Err(ZomeApiError::from(format!(
            "Received error when offering credits, {:?}",
            result
        ))),
    }
}
