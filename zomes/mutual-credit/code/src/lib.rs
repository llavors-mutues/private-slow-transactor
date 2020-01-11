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

use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};
use hdk::{AGENT_ADDRESS, entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

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
    fn send_amout(
        recipient_address: Address,
        amount: usize,
        timestamp: usize,
    ) -> ZomeApiResult<Address> {
        let transaction = Transaction {
            recipient_address,
            issuer_address: AGENT_ADDRESS.clone(),
            amount,
            timestamp
        };

        let entry = Entry::App("my_entry".into(), transaction.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[receive]
    pub fn receive(address: Address, message: JsonString) -> String {
        
    }
}
