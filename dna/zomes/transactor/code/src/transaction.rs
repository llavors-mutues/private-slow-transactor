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
        description: "private transactions that ",
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
