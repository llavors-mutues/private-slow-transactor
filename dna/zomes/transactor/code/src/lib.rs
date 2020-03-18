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

pub mod entries;
use entries::transaction;
use entries::attestation;
use entries::offer;

pub mod workflows;
pub mod message;

use workflows::get_offer_balance::OfferBalance;

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
    pub fn offer_credits(receiver_address: Address, amount: f64) -> ZomeApiResult<Address> {
        workflows::create_offer::send_offer_to(receiver_address, amount)
    }

    #[zome_fn("hc_public")]
    pub fn get_offer_balance(offer_address: Address) -> ZomeApiResult<OfferBalance> {
        workflows::get_offer_balance::get_offer_balance(offer_address)
    }

    #[receive]
    pub fn receive(address: Address, message: JsonString) -> String {
        message::receive_message(address, message)
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

pub fn get_credit_limit(_agent_address: &Address) -> ZomeApiResult<Option<f64>> {
    Ok(Some(-100.0))
}
