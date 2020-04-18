use crate::utils;
use holochain_entry_utils::HolochainEntry;
use hdk::{
    prelude::Entry,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{chain_header::ChainHeader, dna::entry_types::Sharing},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Transaction {
    pub debtor_address: Address,
    pub creditor_address: Address,
    pub timestamp: usize,
    pub amount: f64,
}

impl HolochainEntry for Transaction {
    fn entry_type() -> String {
        String::from("transaction")
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: Transaction::entry_type(),
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
pub fn get_my_completed_transactions() -> ZomeApiResult<Vec<Transaction>> {
    let transactions_entries: Vec<(ChainHeader, Transaction)> = utils::query_all_into()?;

    Ok(transactions_entries.iter().map(|t| t.1.clone()).collect())
}

/**
 * Computes the balance for the given list of transactions and the given agent_address
 */
pub fn compute_balance(agent_address: &Address, transactions: &Vec<Transaction>) -> f64 {
    let mut balance: f64 = 0.0;

    for transaction in transactions {
        if transaction.creditor_address == agent_address.clone() {
            balance += transaction.amount as f64;
        } else if transaction.debtor_address == agent_address.clone() {
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
 * Filters the entries of the given source chain and returns only the transactions
 */
pub fn get_transactions_from_chain_snapshot(chain_snapshot: Vec<(ChainHeader, Entry)>) -> Vec<Transaction> {
    chain_snapshot.iter().filter_map(|(_, entry)| Transaction::from_entry(entry)).collect()
}