use crate::{offer::Offer, transaction::TransactionsSnapshot};
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
    OfferWasCanceled,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub enum MessageBody {
    SendOffer(Message<Offer, ()>),
    GetTransactions(OfferMessage<Address, TransactionsSnapshot>),
    AcceptOffer(OfferMessage<Address, (ChainHeader, Signature)>),
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
