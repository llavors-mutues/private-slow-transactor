use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::signature::Signature;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::ValidationData;
use hdk::{
    error::ZomeApiResult, holochain_core_types::dna::entry_types::Sharing,
    holochain_core_types::entry::Entry, AGENT_ADDRESS,
};

use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TransactionProof {
    pub transaction_address: Address,
    pub agent_header: (Address, Signature),
    pub couterparty_header: (Address, Signature),
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Attestation {
    pub agent_address: Address,
    pub transaction_proof: Option<TransactionProof>,
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

    pub fn entry(self) -> Entry {
        Entry::App("attestation".into(), self.into())
    }

    pub fn from_entry(entry: &Entry) -> Option<Attestation> {
        if let Entry::App(entry_type, attestation_entry) = entry {
            if entry_type.to_string() == "attestation" {
                if let Ok(t) = Attestation::try_from(attestation_entry) {
                    return Some(t);
                }
            }
        }
        None
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
 * Gets all the transactions addresses for the given agent from the DHT
 */
pub fn get_agent_transaction_addresses_from_dht(
    agent_address: &Address,
) -> ZomeApiResult<Vec<Address>> {
    let attestations = get_all_attestations(agent_address)?;

    Ok(attestations
        .iter()
        .filter_map(|attestation| attestation.clone().transaction_proof.map(|proof| proof.transaction_address))
        .collect())
}

/**
 * Gets all attestations from the DHT for the given agent
 */
pub fn get_all_attestations(agent_address: &Address) -> ZomeApiResult<Vec<Attestation>> {
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
