use crate::utils;
use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        chain_header::ChainHeader, dna::entry_types::Sharing, link::LinkMatch, signature::Signature,
    },
    ValidationData, AGENT_ADDRESS,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct HeaderProof {
    pub header_address: Address,
    pub agent_signature: Signature,
    pub counterparty_signature: Signature,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Attestation {
    pub agent_address: Address,
    pub agent_header: HeaderProof,
    pub courterparty_header: HeaderProof,
}

impl HolochainEntry for Attestation {
    fn entry_type() -> String {
        String::from("attestation")
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: Attestation::entry_type(),
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

                    Ok(())
                },

            _ => Err(String::from("Delete attestation is not allowed"))
            }
        },
        links:[
            from!(
                "%agent_id",
                link_type: "agent->attestation",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}

pub fn validate_attestation(
    _attestation: Attestation,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}

/**
 * Gets the last attestation from the DHT for the given agent and the number of attestations present in the DHT
 */
pub fn get_last_attestation_for(agent_address: &Address) -> ZomeApiResult<(Option<Attestation>, usize)> {
    let links_result = hdk::get_links(
        &agent_address,
        LinkMatch::Exactly("agent->attestation"),
        LinkMatch::Any,
    )?;

    let mut links = links_result.links();

    if links.len() == 0 {
        return Ok((None, 0));
    }

    let tag_to_version = |tag: String| tag.parse::<u32>().or::<u32>(Ok(0)).unwrap();

    links.sort_by(|link_a, link_b| {
        let version_a = tag_to_version(link_a.tag);
        let version_b = tag_to_version(link_b.tag);
        version_a.cmp(&version_b)
    });

    let mut tags: Vec<String> = links.iter().map(|l| l.tag).collect();
    tags.dedup();
    
    match hdk::get_entry(&links[0].address)? {
        Some(entry) => match Attestation::from_entry(&entry) {
            Some(attestation) => Ok((Some(attestation), tags.len())),
            None => Err(ZomeApiError::from(String::from(
                "Entry retrieved was not an attestation",
            ))),
        },
        None => Err(ZomeApiError::from(String::from(
            "Could not get attestation when it should be ",
        ))),
    }
}

/**
 * Gets the last attestation that this agent has committed from the private chain
 */
pub fn query_my_last_attestation() -> ZomeApiResult<Attestation> {
    let attestations: Vec<(ChainHeader, Attestation)> = utils::query_all_into()?;

    attestations
        .iter()
        .find(|attestation| attestation.1.agent_address == AGENT_ADDRESS.clone())
        .map(|a| a.1.clone())
        .ok_or(ZomeApiError::from(format!(
            "Could not find last attestation"
        )))
}

/**
 * Gets the attestation identified with the given address from the private chain
 */
pub fn query_attestation(attestation_address: &Address) -> ZomeApiResult<Attestation> {
    let attestations: Vec<(ChainHeader, Attestation)> = utils::query_all_into()?;

    attestations
        .iter()
        .find(|attestation| attestation.0.entry_address() == attestation_address)
        .map(|a| a.1.clone())
        .ok_or(ZomeApiError::from(format!(
            "Could not find attestation {}",
            attestation_address
        )))
}
