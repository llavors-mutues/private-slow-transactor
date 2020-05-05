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
extern crate holochain_entry_utils;

use hdk::prelude::*;

use hdk_proc_macros::zome;

pub mod entries;
use entries::attestation;
use entries::offer;
use entries::transaction;

pub mod complete_transaction;
pub mod create_offer;
pub mod get_chain_snapshot;
pub mod message;
pub mod utils;

use get_chain_snapshot::CounterpartySnapshot;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

#[derive(Serialize, Deserialize, Debug, crate::DefaultJson, Clone)]
pub struct MyBalance(f64);

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
    pub fn create_offer(
        creditor_address: Address,
        amount: f64,
        timestamp: usize,
    ) -> ZomeApiResult<Address> {
        create_offer::sender::create_offer(creditor_address, amount, timestamp)
    }

    #[zome_fn("hc_public")]
    pub fn get_counterparty_snapshot(
        transaction_address: Address,
    ) -> ZomeApiResult<CounterpartySnapshot> {
        get_chain_snapshot::sender::get_counterparty_snapshot(transaction_address)
    }

    #[zome_fn("hc_public")]
    pub fn accept_offer(
        transaction_address: Address,
        approved_header_address: Address,
    ) -> ZomeApiResult<()> {
        complete_transaction::accept_offer::accept_offer(
            transaction_address,
            approved_header_address,
        )
    }

    #[zome_fn("hc_public")]
    pub fn query_my_balance() -> ZomeApiResult<MyBalance> {
        let transactions_with_addresses = transaction::get_my_completed_transactions()?;

        let transactions = transactions_with_addresses.into_iter().map(|t| t.1).collect();

        let balance = transaction::compute_balance(&hdk::AGENT_ADDRESS.clone(), &transactions);
        Ok(MyBalance(balance))
    }

    #[zome_fn("hc_public")]
    pub fn query_my_transactions() -> ZomeApiResult<Vec<(Address, transaction::Transaction)>> {
        transaction::get_my_completed_transactions()
    }

    #[zome_fn("hc_public")]
    pub fn query_offer(transaction_address: Address) -> ZomeApiResult<offer::Offer> {
        offer::query_offer(&transaction_address)
    }

    #[zome_fn("hc_public")]
    pub fn query_my_offers() -> ZomeApiResult<Vec<(Address, offer::Offer)>> {
        offer::query_my_offers()
    }

    #[receive]
    pub fn receive(address: Address, message: JsonString) -> String {
        message::receive_message(address, message)
    }
}

pub fn get_credit_limit(_agent_address: &Address) -> ZomeApiResult<Option<f64>> {
    Ok(Some(-100.0))
}
