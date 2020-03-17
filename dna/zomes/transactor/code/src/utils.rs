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

    let address = hdk::commit_entry_result(&entry, options)?;
    Ok(address.address())
}
