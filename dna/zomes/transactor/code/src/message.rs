use crate::{offer::Offer, transaction::Transaction};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub enum MessageBody {
    SendOffer(Offer),
    GetTransactions { offer_address: Address },
    AcceptOffer { last_header_address: Address },
}
