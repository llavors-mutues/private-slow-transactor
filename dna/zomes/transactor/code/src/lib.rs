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

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

pub mod entries;
use entries::attestation;
use entries::offer;
use entries::transaction;

pub mod utils;
pub mod message;
pub mod workflows;

use workflows::get_sender_balance::BalanceSnapshot;

#[zome]
mod transactor {

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

    #[entry_def]
    fn offer_entry_def() -> ValidatingEntryType {
        offer::entry_definition()
    }

    #[zome_fn("hc_public")]
    pub fn offer_credits(
        receiver_address: Address,
        amount: f64,
        timestamp: usize,
    ) -> ZomeApiResult<Address> {
        workflows::create_offer::create_offer(receiver_address, amount, timestamp)
    }

    #[zome_fn("hc_public")]
    pub fn get_sender_balance(transaction_address: Address) -> ZomeApiResult<BalanceSnapshot> {
        workflows::get_sender_balance::get_sender_balance(transaction_address)
    }

    #[zome_fn("hc_public")]
    pub fn accept_offer(
        offer_address: Address,
        last_header_address: Address,
    ) -> ZomeApiResult<Address> {
        workflows::accept_offer::accept_offer(offer_address, last_header_address)
    }

    #[zome_fn("hc_public")]
    pub fn get_completed_transactions() -> ZomeApiResult<Vec<transaction::Transaction>> {
        transaction::get_my_completed_transactions()
    }

    #[receive]
    pub fn receive(address: Address, message: JsonString) -> String {
        message::receive_message(address, message)
    }
}

pub fn get_credit_limit(_agent_address: &Address) -> ZomeApiResult<Option<f64>> {
    Ok(Some(-100.0))
}
