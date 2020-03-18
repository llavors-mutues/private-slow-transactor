use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
};
use holochain_wasm_utils::api_serialization::{QueryArgsNames, QueryArgsOptions, QueryResult};
use std::convert::TryFrom;

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

    pub fn entry(&self) -> Entry {
        Entry::App("transaction".into(), self.clone().into())
    }

    pub fn address(&self) -> ZomeApiResult<Address> {
        hdk::entry_address(&self.entry())
    }
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
                hdk::EntryValidationData::Create { .. } => {
                    Ok(())
                },
            _ => Err(String::from("Only create transaction is allowed"))
            }
        }
    )
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
pub fn compute_balance(agent_address: &Address, transactions: &Vec<Transaction>) -> f64 {
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
    transactions: &Vec<Transaction>,
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

/**
 * Returns Ok(()) only if both vector of addresses is identical
 */
pub fn validate_transactions_against_attestations(
    attestation_transaction_addresses: &Vec<Address>,
    source_chain_addresses: &Vec<Address>,
) -> ZomeApiResult<()> {
    if attestation_transaction_addresses.len() != source_chain_addresses.len() {
        return Err(ZomeApiError::from(String::from(
            "Chain entries received from the sender do not match the attestation entries",
        )));
    }

    for i in 0..source_chain_addresses.len() {
        if attestation_transaction_addresses.get(i) != source_chain_addresses.get(i) {
            return Err(ZomeApiError::from(String::from(
                "Chain entries received from the sender do not match the attestation entries",
            )));
        }
    }

    Ok(())
}

