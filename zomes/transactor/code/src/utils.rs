use holochain_wasm_utils::api_serialization::commit_entry::CommitEntryOptions;

use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::Entry,
        signature::{Provenance, Signature},
    },
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
};

pub fn commit_with_provenance(entry: &Entry, provenance: Provenance) -> ZomeApiResult<Address> {
    let address = hdk::entry_address(&entry)?;

    let signature = hdk::sign(address)?;

    let my_provenance = Provenance::new(AGENT_ADDRESS.clone(), Signature::from(signature));

    let options = CommitEntryOptions::new(vec![provenance, my_provenance]);

    hdk::debug(format!("hihooo {:?}", options))?;
    let address = hdk::commit_entry_result(&entry, options)?;
    Ok(address.address())
}

/**
 * Go through all the agent's chain and find its AgentId entry, and return its agent_address
 */
pub fn get_chain_agent_id(chain_entries: &Vec<Entry>) -> ZomeApiResult<Address> {
    for entry in chain_entries.iter() {
        if let Entry::AgentId(agent_id) = entry {
            return Ok(agent_id.pub_sign_key.clone().into());
        }
    }

    return Err(ZomeApiError::from(String::from(
        "AgentId entry not found in source chain",
    )));
}
