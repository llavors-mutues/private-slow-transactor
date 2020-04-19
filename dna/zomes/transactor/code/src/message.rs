use crate::complete_transaction::{
    accept_offer::AcceptOfferRequest,
    complete_transaction::{CompleteTransactionRequest, CompleteTransactionResponse},
    sign_attestation::SignAttestationRequest,
};
use crate::{
    complete_transaction, create_offer, get_chain_snapshot, get_chain_snapshot::ChainSnapshot,
    offer::Offer,
};
use hdk::holochain_core_types::{signature::Signature, time::Timeout};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::*;
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message<Req, Res> {
    Request(Req),
    Response(Res),
}

pub type OfferMessage<Req, Res> = Message<Req, OfferResponse<Res>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OfferResponse<Res> {
    OfferPending(Res),
    OfferCanceled,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub enum MessageBody {
    SendOffer(Message<Offer, ()>),
    GetChainSnapshot(OfferMessage<Address, ChainSnapshot>),
    AcceptOffer(OfferMessage<AcceptOfferRequest, ()>),
    CompleteTransaction(OfferMessage<CompleteTransactionRequest, CompleteTransactionResponse>),
    SignAttestation(OfferMessage<SignAttestationRequest, Signature>),
}

/**
 * Send a direct message to receiver address, serializing and deserializing the message body
 */
pub fn send_message(
    receiver_address: Address,
    message_body: MessageBody,
) -> ZomeApiResult<MessageBody> {
    let result = hdk::send(
        receiver_address,
        JsonString::from(message_body).to_string(),
        Timeout::default(),
    )?;

    let success: Result<ZomeApiResult<MessageBody>, _> = JsonString::from_json(&result).try_into();

    match success {
        Ok(Ok(message_body)) => Ok(message_body),
        Ok(Err(error)) => Err(error),
        _ => Err(ZomeApiError::from(format!(
            "Could not deserialize direct message response"
        ))),
    }
}

/**
 * Receive message, recognizing the type of message and executing the appropriate actions
 */
pub fn receive_message(sender_address: Address, message: String) -> String {
    let success: Result<MessageBody, _> = JsonString::from_json(&message).try_into();
    let response = match success {
        Err(err) => Err(ZomeApiError::from(format!(
            "Error deserializing the message: {:?}",
            err
        ))),
        Ok(message_body) => match message_body {
            MessageBody::SendOffer(Message::Request(offer)) => {
                create_offer::receiver::receive_offer(sender_address, offer)
                    .map(|result| MessageBody::SendOffer(Message::Response(result)))
            }
            MessageBody::GetChainSnapshot(OfferMessage::Request(transaction_address)) => {
                get_chain_snapshot::receiver::get_chain_snapshot(
                    sender_address,
                    transaction_address,
                )
                .map(|result| MessageBody::GetChainSnapshot(OfferMessage::Response(result)))
            }
            MessageBody::AcceptOffer(OfferMessage::Request(accept_offer_request)) => {
                complete_transaction::accept_offer::receive_accept_offer(
                    sender_address,
                    accept_offer_request,
                )
                .map(|result| MessageBody::AcceptOffer(OfferMessage::Response(result)))
            }
            MessageBody::CompleteTransaction(OfferMessage::Request(
                complete_transaction_request,
            )) => complete_transaction::complete_transaction::receive_complete_transaction(
                sender_address,
                complete_transaction_request.chain_header,
            )
            .map(|result| MessageBody::CompleteTransaction(OfferMessage::Response(result))),
            MessageBody::SignAttestation(OfferMessage::Request(sign_attestation_request)) => {
                complete_transaction::sign_attestation::receive_sign_attestation_request(
                    sender_address,
                    sign_attestation_request,
                )
                .map(|result| MessageBody::SignAttestation(OfferMessage::Response(result)))
            }
            _ => Err(ZomeApiError::from(format!("Bad message type"))),
        },
    };

    let json: JsonString = response.into();
    json.to_string()
}
