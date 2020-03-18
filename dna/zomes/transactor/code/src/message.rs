use crate::{
    offer::Offer,
    workflows::{
        accept_offer, create_offer, get_offer_balance, get_offer_balance::TransactionsSnapshot,
    },
};
use hdk::holochain_core_types::{chain_header::ChainHeader, signature::Signature, time::Timeout};
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
    OfferNotPending,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub enum MessageBody {
    SendOffer(Message<Offer, ()>),
    GetTransactions(OfferMessage<Address, TransactionsSnapshot>),
    AcceptOffer(OfferMessage<accept_offer::AcceptOfferRequest, (ChainHeader, Signature)>),
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
                create_offer::receive_offer(sender_address, offer)
                    .map(|result| MessageBody::SendOffer(Message::Response(result)))
            }
            MessageBody::GetTransactions(OfferMessage::Request(offer_address)) => {
                get_offer_balance::get_my_transactions_snapshot_for_offer(offer_address)
                    .map(|result| MessageBody::GetTransactions(OfferMessage::Response(result)))
            }
            _ => Err(ZomeApiError::from(format!("Bad message type"))),
        },
    };

    let json: JsonString = response.into();
    json.to_string()
}
