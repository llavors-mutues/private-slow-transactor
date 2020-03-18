use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::{chain_header::ChainHeader, dna::entry_types::Sharing};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::AddressableContent;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
};
use holochain_wasm_utils::api_serialization::{QueryArgsNames, QueryArgsOptions, QueryResult};
use std::convert::TryFrom;

use crate::attestation::Attestation;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TransactionsSnapshot {
    pub transactions: Vec<Transaction>,
    pub last_header_address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Transaction {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub timestamp: usize,
    pub amount: f64,
}

impl Transaction {
    pub fn from_entry(entry: &Entry) -> Option<Transaction> {
        match entry {
            Entry::App(entry_type, transaction_entry) => {
                if entry_type.to_string() != "transaction" {
                    return None;
                }

                match Transaction::try_from(transaction_entry) {
                    Ok(t) => Some(t),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub fn transaction_entry(transaction: &Transaction) -> Entry {
    Entry::App("transaction".into(), transaction.clone().into())
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "transaction",
        description: "private transactions that are the base to compute the total balance",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_validation_data: hdk::EntryValidationData<Transaction>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    Ok(())
                },
            _ => Err(String::from("Only create transaction is allowed"))
            }
        }
    )
}

/**
 * Get the list of transactions and the last header from the source chain
 */
pub fn get_my_transactions_snapshot() -> ZomeApiResult<TransactionsSnapshot> {
    let last_header = get_my_last_header()?;
    let transactions = get_all_my_transactions()?;

    Ok(TransactionsSnapshot {
        last_header_address: last_header.address(),
        transactions,
    })
}

/**
 * Gets the last header of my source chain
 */
pub fn get_my_last_header() -> ZomeApiResult<ChainHeader> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        entries: false,
        headers: true,
    };
    let query_result = hdk::query_result(QueryArgsNames::from("*"), options)?;

    match query_result {
        QueryResult::Headers(headers) => headers
            .first()
            .ok_or(ZomeApiError::from(format!("Error getting the last header")))
            .map(|h| h.clone()),
        _ => Err(ZomeApiError::from(format!("Error getting the last header"))),
    }
}

/**
 * Returns all the transactions already completed that are present in the source chain
 */
pub fn get_all_my_transactions() -> ZomeApiResult<Vec<Transaction>> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        entries: true,
        headers: false,
    };
    let query_result = hdk::query_result(QueryArgsNames::from("transaction"), options)?;

    match query_result {
        QueryResult::Entries(entries) => Ok(entries
            .iter()
            .filter_map(|entry| Transaction::from_entry(&entry.1))
            .collect()),
        _ => Err(ZomeApiError::from(format!(
            "Error getting own transactions"
        ))),
    }
}

/**
 * Computes the balance for the given list of transactions and the given agent_address
 */
pub fn compute_balance(agent_address: &Address, transactions: Vec<Transaction>) -> f64 {
    let mut balance: f64 = 0.0;

    for transaction in transactions {
        if transaction.receiver_address == agent_address.clone() {
            balance += transaction.amount as f64;
        } else if transaction.sender_address == agent_address.clone() {
            balance -= transaction.amount as f64;
        }
    }

    balance
}

/**
 * Compute balance for the given transactions and return valid if it's less that the credit limit
 */
pub fn are_transactions_valid(
    agent_address: &Address,
    transactions: Vec<Transaction>,
) -> ZomeApiResult<bool> {
    if let Some(credit_limit) = crate::get_credit_limit(agent_address)? {
        // Get the balance for this agent
        let balance = compute_balance(agent_address, transactions);

        if balance < credit_limit {
            return Ok(false);
        }
    }

    Ok(true)
}
