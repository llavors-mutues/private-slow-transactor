#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType, error::ZomeApiResult, holochain_core_types::entry::Entry,
};

use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

use std::convert::TryInto;

pub mod attestation;
pub mod message;
pub mod receiver;
pub mod sender;
pub mod transaction;
pub mod utils;

use crate::message::MessageBody;

#[zome]
mod transaction {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn transaction_entry_def() -> ValidatingEntryType {
        transaction::entry_definition()
    }

    #[entry_def]
    fn attestation_entry_def() -> ValidatingEntryType {
        attestation::entry_definition()
    }

    #[zome_fn("hc_public")]
    fn send_amount(
        receiver_address: Address,
        amount: usize,
        timestamp: usize,
    ) -> ZomeApiResult<Address> {
        sender::send_amout(receiver_address, amount, timestamp)
    }

    #[receive]
    pub fn receive(address: Address, message: JsonString) -> String {
        let success: Result<MessageBody, _> = JsonString::from_json(&message).try_into();
        match success {
            Err(err) => format!("Error: {}", err),
            Ok(message) => {
                /* let r = hdk::emit_signal(
                    message.signal.as_str(),
                    JsonString::from_json(&format!("{{message: {:?}}}", message)),
                );
                json!(r).to_string() */
                match receiver::validate_and_commit_transaction(address, message) {
                    Ok(signature) => signature,
                    Err(err) => format!("Error: there was an error validating the transaction: {}", err),
                }
            }
        }
    }

    #[zome_fn("hc_public")]
    pub fn get_my_transactions() -> ZomeApiResult<Vec<Address>> {
        hdk::query("transaction".into(), 0, 0)
    }

    #[zome_fn("hc_public")]
    pub fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }
}

pub fn get_credit_limit(_agent_address: &Address) -> ZomeApiResult<Option<isize>> {
    Ok(Some(-100))
}
