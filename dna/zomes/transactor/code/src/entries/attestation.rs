use crate::utils;
use crate::utils::ParseableEntry;
use super::transaction::Transaction;
use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::ValidationData;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{dna::entry_types::Sharing, signature::Signature},
    AGENT_ADDRESS,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum TransactionRole {
    Sender {
        receiver_transaction_snapshot_proof: Signature, // Signature of 'transaction_address,last_header_address' by the receiver
    },
    Receiver {
        sender_attestation_address: Address, // Address of the sender's attestation
    },
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AttestationProof {
    pub transaction_address: Address,
    pub transaction_header: (Address, Signature),
    pub transaction_role: TransactionRole,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Attestation {
    pub agent_address: Address,
    pub transaction_proof: Option<AttestationProof>,
}

impl Attestation {
    pub fn my_initial() -> Attestation {
        Attestation {
            transaction_proof: None,
            agent_address: AGENT_ADDRESS.clone(),
        }
    }

    pub fn initial(agent_address: &Address) -> Attestation {
        Attestation {
            transaction_proof: None,
            agent_address: agent_address.clone(),
        }
    }
}

impl utils::ParseableEntry for Attestation {
    fn entry_type() -> String {
        String::from("attestation")
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "attestation",
        description: "attestation entry to vouch for the last transaction of an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Attestation>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => validate_attestation(entry, validation_data),
                hdk::EntryValidationData::Modify {
                    new_entry,
                    old_entry,
                    ..
                 } => {
                    if new_entry.agent_address != old_entry.agent_address {
                        return Err(String::from("Cannot modify agent address of an attestation"));
                    }
                    if let None = new_entry.transaction_proof {
                        return Err(String::from("Cannot update an attestation without a transaction proof"));
                    }

                    Ok(())
                },

            _ => Err(String::from("Delete attestation is not allowed"))
            }
        }
    )
}

/**
 * Creates initial public attestation entry for myself
 */
pub fn create_initial_attestation() -> ZomeApiResult<Address> {
    let attestation = Attestation::my_initial();

    let attestations = hdk::query("attestation".into(), 0, 0)?;

    let entry = attestation.entry();

    if attestations.len() == 0 {
        hdk::commit_entry(&entry)
    } else {
        hdk::entry_address(&entry)
    }
}

pub fn validate_attestation(
    _attestation: Attestation,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}

/**
 * Gets all attestations from the DHT for the given agent
 */
pub fn get_attestations_for_agent(agent_address: &Address) -> ZomeApiResult<Vec<Attestation>> {
    let attestation = Attestation::initial(agent_address);

    let initial_address = hdk::entry_address(&attestation.entry())?;

    let maybe_history = hdk::get_entry_history(&initial_address)?;

    match maybe_history {
        None => Ok(vec![]),
        Some(history) => {
            let attestations = history
                .items
                .iter()
                .filter_map(|item| {
                    if let Some(entry) = item.entry.clone() {
                        return Attestation::from_entry(&entry);
                    }
                    None
                })
                .collect();
            Ok(attestations)
        }
    }
}

/**
 * Gets the attestation identified with the given address from the private chain
 */
pub fn query_attestation(attestation_address: &Address) -> ZomeApiResult<Attestation> {
    let attestations = utils::query_all_into(String::from("attestation"))?;

    attestations
        .iter()
        .find(|attestation| attestation.0.entry_address() == attestation_address)
        .map(|a| a.1)
        .ok_or(ZomeApiError::from(format!(
            "Could not find attestation {}",
            attestation_address
        )))
}

/**
 * Validates the given list of transactions for one agent with the given list of attestations for the same agent
 */
pub fn validate_transactions_against_attestations(
    attestations: &Vec<Attestation>,
    transactions: &Vec<Transaction>,
) -> ZomeApiResult<()> {
    if attestations.len() != transactions.len() {
        return Err(ZomeApiError::from(String::from(
            "Chain entries received from the sender do not match the attestation entries",
        )));
    }

    for (attestation, transaction) in attestations.iter().zip(transactions.iter()) {
        validate_transaction_against_attestation(attestation, transaction)?;
    }

    Ok(())
}

/**
 * Validates a single transaction for one agent with the attestation from the same agent
 */
fn validate_transaction_against_attestation(
    attestation: &Attestation,
    transaction: &Transaction,
) -> ZomeApiResult<()> {
    let transaction_proof = attestation
        .transaction_proof
        .ok_or(ZomeApiError::from(format!(
            "Attestation does not contain transaction proof"
        )))?;
    let transaction_address = transaction.address()?;

    if transaction_proof.transaction_address != transaction_address {
        return Err(ZomeApiError::from(format!(
            "Transaction addresses do not match"
        )));
    }

    let role_valid = match transaction_proof.transaction_role {
        TransactionRole::Sender {
            receiver_transaction_snapshot_proof,
        } => transaction.sender_address == attestation.agent_address,
        TransactionRole::Receiver { .. } => {
            transaction.receiver_address == attestation.agent_address
        }
    };

    if !role_valid {
        return Err(ZomeApiError::from(format!("Role proof not valid")));
    }

    Ok(())
}
