use crate::{
    accept_offer, create_offer, get_chain_snapshot, get_chain_snapshot::ChainSnapshot, offer::Offer,
};
use hdk::holochain_core_types::time::Timeout;
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
    OfferCompleted(Address), // Will contain the attestation address from which to be able to complete the transaction
    OfferCanceled,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub enum MessageBody {
    SendOffer(Message<Offer, ()>),
    GetChainSnapshot(OfferMessage<Address, ChainSnapshot>),
    AcceptOffer(OfferMessage<accept_offer::AcceptOfferRequest, ()>),
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
            MessageBody::AcceptOffer(OfferMessage::Request(request)) => {
                accept_offer::receiver::receive_accept_offer(sender_address, request)
                    .map(|result| MessageBody::AcceptOffer(OfferMessage::Response(result)))
            }
            _ => Err(ZomeApiError::from(format!("Bad message type"))),
        },
    };

    let json: JsonString = response.into();
    json.to_string()
}
