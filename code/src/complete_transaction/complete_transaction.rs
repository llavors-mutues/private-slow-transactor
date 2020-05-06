use super::{
    common::{create_attestation, validate_counterparty_header},
    sign_attestation::SignAttestationRequest,
};
use crate::{
    message,
    message::{Message, MessageBody, OfferMessage, OfferResponse},
    offer,
    offer::{Offer, OfferState},
    transaction, utils,
};
use hdk::{
    holochain_core_types::{chain_header::ChainHeader, signature::Signature},
    prelude::*,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct CompleteTransactionRequest {
    pub chain_header: ChainHeader,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct CompleteTransactionResponse {
    pub chain_headers: Vec<ChainHeader>,
    pub signature: Signature,
}

/**
 * Validate the received proof from the sender of the transaction and execute it
 */
pub fn receive_complete_transaction(
    sender_address: Address,
    chain_header: ChainHeader,
) -> ZomeApiResult<OfferResponse<CompleteTransactionResponse>> {
    let offer = offer::query_offer(chain_header.entry_address())?;

    let transaction = offer.clone().transaction;

    let counterparty = transaction::get_counterparty(&transaction);

    if sender_address != counterparty {
        return Err(ZomeApiError::from(String::from(
            "Agent sending the message is not the counterparty for this transaction",
        )));
    }

    match offer.clone().state {
        OfferState::Approved {
            approved_header_address,
        } => handle_complete_transaction(offer, chain_header, approved_header_address)
            .map(|result| OfferResponse::OfferPending(result)),
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
        _ => Err(ZomeApiError::from(format!(
            "Offer for transaction {:?} has not been approved",
            transaction.address()
        ))),
    }
}

/**
 * Completes the transaction
 *
 * 1. Checks that the counterparty's header is valid
 * 2. Creates the transaction
 * 3. Builds and signs the attestation
 * 4. Sends a SignAttestationRequest
 * 5. Commits the attestation
 */
pub fn handle_complete_transaction(
    offer: Offer,
    counterparty_header: ChainHeader,
    approved_header_address: Option<Address>,
) -> ZomeApiResult<CompleteTransactionResponse> {
    validate_counterparty_header(
        &counterparty_header,
        &offer.transaction,
        &approved_header_address,
    )?;

    hdk::commit_entry(&offer.transaction.clone().entry())?;

    let transaction_header = utils::get_my_last_header()?;

    let headers = vec![transaction_header, counterparty_header];

    let request = SignAttestationRequest {
        chain_headers: headers.clone(),
    };

    let message = MessageBody::SignAttestation(Message::Request(request));

    let counterparty = transaction::get_counterparty(&offer.transaction);

    let result = message::send_message(counterparty, message)?;

    match result {
        MessageBody::SignAttestation(OfferMessage::Response(OfferResponse::OfferPending(
            counterpary_signature,
        ))) => {
            // Create the attestation from the headers and the received counterparty_signature
            let attestation_address = create_attestation(&headers, &counterpary_signature)?;

            let my_signature = hdk::sign(attestation_address)?;

            let response = CompleteTransactionResponse {
                chain_headers: headers,
                signature: Signature::from(my_signature),
            };

            hdk::emit_signal(
                "offer-completed",
                JsonString::from_json(&format!(
                    "{{\"transaction_address\": \"{}\"}}",
                    offer.transaction.address()?
                )),
            )?;

            Ok(response)
        }
        MessageBody::SignAttestation(OfferMessage::Response(OfferResponse::OfferCanceled)) => {
            offer::cancel_offer(&offer.transaction.address()?)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
        _ => Err(ZomeApiError::from(format!(
            "Received error when attempting to get the attestation signature: {:?}",
            result
        ))),
    }
}
