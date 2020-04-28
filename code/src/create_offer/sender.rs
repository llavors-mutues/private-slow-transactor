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
    creditor_address: Address,
    amount: f64,
    timestamp: usize,
) -> ZomeApiResult<Address> {
    let transaction = Transaction {
        debtor_address: AGENT_ADDRESS.clone(),
        creditor_address: creditor_address.clone(),
        amount,
        timestamp,
    };

    let message_body = MessageBody::SendOffer(Message::Request(transaction.clone()));

    let result = message::send_message(creditor_address, message_body)?;

    match result {
        MessageBody::SendOffer(Message::Response(())) => {
            let offer = Offer {
                transaction: transaction.clone(),
                state: OfferState::Approved {
                    approved_header_address: None,
                },
            };
            hdk::commit_entry(&offer.entry())?;
            Ok(transaction.address()?)
        }
        _ => Err(ZomeApiError::from(format!(
            "Received error when offering credits, {:?}",
            result
        ))),
    }
}
