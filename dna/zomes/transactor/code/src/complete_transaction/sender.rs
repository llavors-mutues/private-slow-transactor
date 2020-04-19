use super::AcceptOfferRequest;
use crate::{
    message::{send_message, MessageBody, OfferMessage, OfferResponse},
    offer,
};
use hdk::{holochain_persistence_api::cas::content::Address, prelude::*};

/**
 * Accepts the offer, verifying that the source chain of the sender agent has not changed,
 * and creating the transaction privately
 */
pub fn accept_offer(
    transaction_address: Address,
    approved_header_address: Address,
) -> ZomeApiResult<()> {
    let offer = offer::query_offer(&transaction_address)?;

    let transaction = offer.transaction;

    offer::approve_offer(&transaction_address, &Some(approved_header_address.clone()))?;

    let accept_offer_request = AcceptOfferRequest {
        transaction_address: transaction_address.clone(),
        approved_header_address: approved_header_address.clone(),
    };

    let message = MessageBody::AcceptOffer(OfferMessage::Request(accept_offer_request));

    // TODO: generalize transaction.debtor_address to be the counterparty
    let result = send_message(transaction.debtor_address.clone(), message)?;

    let response = match result {
        MessageBody::AcceptOffer(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "CompleteOffer response is not valid"
        ))),
    }?;

    match response {
        OfferResponse::OfferPending(()) => Ok(()),
        OfferResponse::OfferCanceled => {
            offer::cancel_offer(&transaction_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
    }
}
