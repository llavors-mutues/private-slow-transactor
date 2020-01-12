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

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

use std::convert::TryInto;

pub mod message;
pub mod receiver;
pub mod sender;
pub mod transaction;
pub mod utils;

use crate::message::MessageBody;
use crate::transaction::Transaction;

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
        entry!(
            name: "transaction",
            description: "this is a same entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::ChainFull
            },
            validation: |_validation_data: hdk::EntryValidationData<Transaction>| {
                match _validation_data {
                    hdk::EntryValidationData::Create { entry, validation_data } => {
                        let sources = validation_data.sources();

                        if !sources.contains(&entry.receiver_address) || !sources.contains(&entry.sender_address) {
                            return Err(String::from("Transaction must be signed by sender and receiver"));
                        }

                        Ok(())
                    },
                _ => Err(String::from("Only create transaction is allowed"))
                }
            }
        )
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
            Err(err) => format!("error: {}", err),
            Ok(message) => {
                /* let r = hdk::emit_signal(
                    message.signal.as_str(),
                    JsonString::from_json(&format!("{{message: {:?}}}", message)),
                );
                json!(r).to_string() */
                match receiver::validate_and_commit_transaction(address, message) {
                    Ok(signature) => signature,
                    Err(err) => format!("There was an error validating the transaction: {}", err),
                }
            }
        }
    }
}
