use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
};
use std::convert::TryFrom;

use crate::attestation::Attestation;
use crate::utils::get_chain_agent_id;

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
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_validation_data: hdk::EntryValidationData<Transaction>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    // 1. Check if receiver and sender are not the same
                    if entry.receiver_address == entry.sender_address {
                        return Err(String::from("Receiver and sender cannot be the same"));
                    }

                    /*
                    // 2. Check that the receiver and the sender have both signed the transaction
                    let sources = validation_data.sources();
                    if !sources.contains(&entry.receiver_address) || !sources.contains(&entry.sender_address) {
                        return Err(String::from("Transaction must be signed by sender and receiver"));
                    } */
                    let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();
                    let agent_address = get_chain_agent_id(&chain_entries)?;

                    let mut transactions = get_transactions(&chain_entries);
                    transactions.push(entry.clone());

                    // 3. Check that the balance is not less than the credit limit
                    validate_transactions(&agent_address, transactions)?;

                    // 4. If this is the sender's chain, check that there is an attestation for the transaction
                    if agent_address == entry.receiver_address {
                        return Ok(());
                    }

                    let last_attestation = get_last_attestation(&chain_entries)?;
                    let transaction_address = hdk::entry_address(&transaction_entry(&entry))?;

                    if let Some(last_transaction_address) = last_attestation.last_transaction_address {
                        if last_transaction_address == transaction_address {
                            return Ok(());
                        }
                    }
                    return Err(String::from("Transaction entry must be preceded by its attestation"));
                },
            _ => Err(String::from("Only create transaction is allowed"))
            }
        }
    )
}

pub fn get_transactions(entries: &Vec<Entry>) -> Vec<Transaction> {
    entries
        .into_iter()
        .filter_map(|entry| Transaction::from_entry(entry))
        .collect()
}

pub fn get_last_attestation(entries: &Vec<Entry>) -> ZomeApiResult<Attestation> {
    let maybe_attestation = entries.first();
    if let Some(attestation_entry) = maybe_attestation {
        if let Entry::App(entry_type, attestation_entry) = attestation_entry {
            if entry_type.to_string() == "attestation" {
                if let Ok(a) = Attestation::try_from(attestation_entry) {
                    return Ok(a);
                }
            }
        }
    }

    Err(ZomeApiError::from(String::from(
        "Last entry is not an attestation",
    )))
}

pub fn compute_balance(agent_address: &Address, transactions: Vec<Transaction>) -> isize {
    let mut balance: isize = 0;

    for transaction in transactions {
        if transaction.receiver_address == agent_address.clone() {
            balance += transaction.amount as isize;
        } else if transaction.sender_address == agent_address.clone() {
            balance -= transaction.amount as isize;
        }
    }

    balance
}

pub fn validate_transactions(
    agent_address: &Address,
    transactions: Vec<Transaction>,
) -> ZomeApiResult<()> {
    if let Some(credit_limit) = crate::get_credit_limit(agent_address)? {
        // Get the balance for this agent
        let balance = compute_balance(agent_address, transactions);

        if balance < credit_limit {
            return Err(ZomeApiError::from(String::from(
                "Agent does not have enough credit",
            )));
        }
    }

    Ok(())
}
