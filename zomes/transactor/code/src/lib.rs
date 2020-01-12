#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    entry::Entry,
    signature::{Provenance, Signature},
    time::Timeout,
};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};
use holochain_wasm_utils::api_serialization::commit_entry::CommitEntryOptions;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

use std::convert::TryInto;

// see https://developer.holochain.org/api/0.0.42-alpha3/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Transaction {
    pub issuer_address: Address,
    pub recipient_address: Address,
    pub timestamp: usize,
    pub amount: usize,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageBody {
    pub transaction: Transaction,
    pub signature: String,
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn transaction_entry_def() -> ValidatingEntryType {
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
                        let sources = validation_data.sources();

                        if !sources.contains(&entry.recipient_address) || !sources.contains(&entry.issuer_address) {
                            return Err(String::from("Transaction must be signed by issuer and recipient"));
                        }

                        Ok(())
                    },
                _ => Err(String::from("Only create transaction is allowed"))
                }
            }
        )
    }

    #[zome_fn("hc_public")]
    fn send_amount(
        recipient_address: Address,
        amount: usize,
        timestamp: usize,
    ) -> ZomeApiResult<Address> {
        let transaction = Transaction {
            recipient_address: recipient_address.clone(),
            issuer_address: AGENT_ADDRESS.clone(),
            amount,
            timestamp,
        };

        let entry = transaction_entry(&transaction);
        let address = hdk::entry_address(&entry)?;

        let signature = hdk::sign(address)?;

        let message = MessageBody {
            transaction,
            signature,
        };

        let signature = hdk::send(
            recipient_address.clone(),
            JsonString::from(message).to_string(),
            Timeout::default(),
        )?;

        let transaction_address = commit_with_provenance(
            &entry,
            Provenance::new(recipient_address, Signature::from(signature)),
        )?;

        Ok(transaction_address)
    }

    #[receive]
    pub fn receive(address: Address, message: JsonString) -> String {
        let success: Result<MessageBody, _> = JsonString::from_json(&message).try_into();
        match success {
            Err(err) => format!("error: {}", err),
            Ok(message) => {
                /* let r = hdk::emit_signal(
                    message.signal.as_str(),
                    JsonString::from_json(&format!("{{message: {:?}}}", message)),
                );
                json!(r).to_string() */
                match validate_and_commit_transaction(address, message) {
                    Ok(signature) => signature,
                    Err(err) => format!("There was an error validating the transaction: {}", err),
                }
            }
        }
    }
}

pub fn transaction_entry(transaction: &Transaction) -> Entry {
    Entry::App("transaction".into(), transaction.clone().into())
}

pub fn validate_and_commit_transaction(
    address: Address,
    message: MessageBody,
) -> ZomeApiResult<String> {
    let entry = transaction_entry(&message.transaction);

    let entry_address = hdk::entry_address(&entry)?;

    let provenance = Provenance::new(address.clone(), Signature::from(message.signature));

    let valid = hdk::verify_signature(provenance.clone(), entry_address.clone())?;

    if !valid {
        return Err(ZomeApiError::from(String::from("Signature not valid")));
    }

    commit_with_provenance(&entry, provenance)?;

    hdk::sign(entry_address)
}

pub fn commit_with_provenance(entry: &Entry, provenance: Provenance) -> ZomeApiResult<Address> {
    let address = hdk::entry_address(&entry)?;

    let signature = hdk::sign(address)?;

    let my_provenance = Provenance::new(AGENT_ADDRESS.clone(), Signature::from(signature));

    let options = CommitEntryOptions::new(vec![provenance, my_provenance]);
    let address = hdk::commit_entry_result(&entry, options)?;
    Ok(address.address())
}
