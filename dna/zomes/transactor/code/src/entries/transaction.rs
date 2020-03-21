use crate::{utils, utils::ParseableEntry};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{chain_header::ChainHeader, dna::entry_types::Sharing},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Transaction {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub timestamp: usize,
    pub amount: f64,
}

impl ParseableEntry for Transaction {
    fn entry_type() -> String {
        String::from("transaction")
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
