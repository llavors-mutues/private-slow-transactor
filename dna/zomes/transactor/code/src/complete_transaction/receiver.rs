use super::{
    common::{
        create_attestation, validate_counterparty_header, validate_last_header_still_unchanged,
        validate_transaction_headers,
    },
    AcceptOfferRequest, CompleteTransactionRequest, CompleteTransactionResponse,
    SignAttestationRequest,
};
use crate::{
    attestation::Attestation,
    message,
    message::{Message, MessageBody, OfferMessage, OfferResponse},
    offer,
    offer::{Offer, OfferState},
    transaction, utils,
};
use hdk::{
    holochain_core_types::{chain_header::ChainHeader, signature::Signature},
    prelude::*,
    AGENT_ADDRESS,
};
use holochain_entry_utils::HolochainEntry;

/**
 * Verifies that offer is approved, and then begins the complete transaction workflow
 */
pub fn receive_accept_offer(
    sender_address: Address,
    accept_offer_request: AcceptOfferRequest,
) -> ZomeApiResult<OfferResponse<()>> {
    let offer = offer::query_offer(&accept_offer_request.transaction_address)?;

    let transaction = offer.transaction;

    let counterparty = transaction::get_counterparty(&transaction);

    if sender_address != counterparty {
        return Err(ZomeApiError::from(String::from(
            "Agent sending the message is not the counterparty for this transaction",
        )));
    }

    match offer.state {
        OfferState::Approved {
            approved_header_address,
        } => {
            handle_accept_offer(accept_offer_request, approved_header_address)?;

            Ok(OfferResponse::OfferPending(()))
        }
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
        _ => Err(ZomeApiError::from(format!(
            "Offer for transaction {} has not been approved",
            accept_offer_request.transaction_address
        ))),
    }
}

/**
 * Handles an incoming AcceptOfferRequest, assuming that the offer was approved
 *
 * 1. Check that the approved_header_address is still the same
 * 2. Create the transaction
 * 3. Get the transaction header
 * 4. Send a CompleteTransactionRequest
 * 5. Create Attestation
 */
pub fn handle_accept_offer(
    accept_offer_request: AcceptOfferRequest,
    _approved_header_address: Option<Address>, // TODO: in the future, verify that creditor hasn't also committed anything new
) -> ZomeApiResult<()> {
    validate_last_header_still_unchanged(accept_offer_request.approved_header_address)?;

    let offer = offer::query_offer(&accept_offer_request.transaction_address)?;
    hdk::commit_entry(&offer.transaction.clone().entry())?;

    let transaction_header = utils::get_my_last_header()?;

    let complete_transaction_request = CompleteTransactionRequest {
        chain_header: transaction_header,
    };

    let message =
        MessageBody::CompleteTransaction(OfferMessage::Request(complete_transaction_request));

    let counterparty_address = transaction::get_counterparty(&offer.transaction);

    let result = message::send_message(counterparty_address, message)?;

    match result {
        MessageBody::CompleteTransaction(OfferMessage::Response(OfferResponse::OfferPending(
            complete_transaction_response,
        ))) => {
            create_attestation(
                &complete_transaction_response.chain_headers,
                &complete_transaction_response.signature,
            )?;
            Ok(())
        }
        MessageBody::CompleteTransaction(OfferMessage::Response(OfferResponse::OfferCanceled)) => {
            offer::cancel_offer(&accept_offer_request.transaction_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
        _ => Err(ZomeApiError::from(format!(
            "Received error when attempting to complete the transaction {:?}",
            result
        ))),
    }
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

/**
 * Verifies that offer is approved, and then begins the complete transaction workflow
 */
pub fn receive_sign_attestation_request(
    sender_address: Address,
    sign_attestation_request: SignAttestationRequest,
) -> ZomeApiResult<OfferResponse<Signature>> {
    let transaction_address = sign_attestation_request.chain_headers[0].entry_address();
    let offer = offer::query_offer(&transaction_address)?;

    let transaction = offer.transaction;

    let counterparty = transaction::get_counterparty(&transaction);

    if sender_address != counterparty {
        return Err(ZomeApiError::from(String::from(
            "Agent sending the message is not the counterparty for this transaction",
        )));
    }
    match offer.state {
        OfferState::Approved {
            approved_header_address,
        } => {
            let signature =
                handle_sign_attestation(sign_attestation_request, approved_header_address)?;

            Ok(OfferResponse::OfferPending(signature))
        }
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
        _ => Err(ZomeApiError::from(format!(
            "Offer for transaction {} has not been approved",
            transaction_address
        ))),
    }
}

/**
 * Assumes that the offer is still pending
 *
 * 1. Check that my header has not moved
 * 2. Check that the transaction headers are valid
 * 3. Check that the counterparty's header is valid
 * 4. Build and sign the attestation entry
 */
pub fn handle_sign_attestation(
    sign_attestation_request: SignAttestationRequest,
    approved_header_address: Option<Address>,
) -> ZomeApiResult<Signature> {
    let my_header = sign_attestation_request
        .chain_headers
        .iter()
        .find(|h| h.provenances()[0].source() != AGENT_ADDRESS.clone())
        .ok_or(ZomeApiError::from(String::from(
            "Could not find my transaction header",
        )))?;

    let previous_header_address = my_header
        .link()
        .ok_or(ZomeApiError::from(String::from("Bad header")))?;

    validate_last_header_still_unchanged(previous_header_address)?;

    validate_transaction_headers(&sign_attestation_request.chain_headers)?;

    let counterparty_header = sign_attestation_request
        .chain_headers
        .iter()
        .find(|h| h.provenances()[0].source() != AGENT_ADDRESS.clone())
        .ok_or(ZomeApiError::from(String::from(
            "Could not find the transaction header for my counterparty",
        )))?;

    let transaction_address = sign_attestation_request.chain_headers[0].entry_address();
    let offer = offer::query_offer(&transaction_address)?;

    validate_counterparty_header(
        &counterparty_header,
        &offer.transaction,
        &approved_header_address,
    )?;

    let attestation = Attestation::from_headers(&sign_attestation_request.chain_headers);

    let signature = hdk::sign(attestation.address()?)?;

    Ok(Signature::from(signature))
}
