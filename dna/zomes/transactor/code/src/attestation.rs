use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    error::ZomeApiResult, holochain_core_types::dna::entry_types::Sharing,
    holochain_core_types::entry::Entry, AGENT_ADDRESS,
};

use std::convert::TryFrom;

use crate::utils::get_chain_agent_id;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Attestation {
    pub sender_address: Address,
    pub last_transaction_address: Option<Address>,
}

impl Attestation {
    pub fn my_initial() -> Attestation {
        Attestation {
            last_transaction_address: None,
            sender_address: AGENT_ADDRESS.clone(),
        }
    }

    pub fn initial(agent_address: &Address) -> Attestation {
        Attestation {
            last_transaction_address: None,
            sender_address: agent_address.clone(),
        }
    }

    pub fn from(transaction_address: Address) -> Attestation {
        Attestation {
            last_transaction_address: Some(transaction_address),
            sender_address: AGENT_ADDRESS.clone(),
        }
    }

    pub fn from_entry(entry: &Entry) -> Option<Attestation> {
        match entry {
            Entry::App(entry_type, attestation_entry) => {
                if entry_type.to_string() != "attestation" {
                    return None;
                }

                match Attestation::try_from(attestation_entry) {
                    Ok(t) => Some(t),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "attestation",
        description: "attestation entry to vouch for the last transaction of an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_validation_data: hdk::EntryValidationData<Attestation>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    if let Some(_) = entry.last_transaction_address {
                        return Err(String::from("Last transaction must be empty on attestation creation"));
                    }

                    let chain_entries = validation_data.package.source_chain_entries.unwrap().clone();

                    let agent_address = get_chain_agent_id(&chain_entries)?;

                    if entry.sender_address != agent_address {
                        return Err(String::from("Only sender agent can create its own attestation"));
                    }

                    Ok(())
                },
                hdk::EntryValidationData::Modify {
                    new_entry,
                    old_entry,
                    ..
                 } => {
                    if new_entry.sender_address != old_entry.sender_address {
                        return Err(String::from("Cannot modify sender address of an attestation"));
                    }

                    Ok(())
                },

            _ => Err(String::from("Delete attestation is not allowed"))
            }
        }
    )
}

pub fn attestation_entry(attestation: Attestation) -> Entry {
    Entry::App("attestation".into(), attestation.into())
}

pub fn create_initial_attestation() -> ZomeApiResult<Address> {
    let attestation = Attestation::my_initial();

    let attestations = hdk::query("attestation".into(), 0, 0)?;

    let entry = attestation_entry(attestation);

    if attestations.len() == 0 {
        hdk::commit_entry(&entry)
    } else {
        hdk::entry_address(&entry)
    }
}
