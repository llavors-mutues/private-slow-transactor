use crate::message::MessageBody;
use hdk::holochain_core_types::time::Timeout;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub enum OfferState {
    Pending,
    Declined,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Offer {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub amount: f64,
    pub state: OfferState,
}

impl Offer {
    pub fn entry(self) -> Entry {
        Entry::App("offer".into(), self.into())
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "offer",
        description: "offer private entry to temporarily store the data of a transaction before accepting it",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Offer>| {
            Ok(())
        }
    )
}

/**
 * Sends an offer to the receiver address, and when Creates a private offer to the given receiver address, setting up the transaction
 * Also send a direct message to the receiver notifying the offer
 */
pub fn send_offer_to(receiver_address: Address, amount: f64) -> ZomeApiResult<Address> {
    let offer = Offer {
        sender_address: AGENT_ADDRESS.clone(),
        receiver_address: receiver_address.clone(),
        amount,
        state: OfferState::Pending,
    };

    let message = MessageBody::SendOffer(offer);

    let result = hdk::send(
        receiver_address,
        JsonString::from(message).to_string(),
        Timeout::default(),
    )?;

    if !result.contains("Ok") {
        return Err(ZomeApiError::from(format!(
            "Received error when offering credits, {:?}",
            result
        )));
    }

    // Create our private entry
    let offer_entry = offer.entry();

    hdk::commit_entry(&offer_entry)
}

/**
 * Receive and offer, check that it's valid, and store it privately
 */
pub fn receive_offer(offer: Offer) -> ZomeApiResult<()> {
    if offer.receiver_address != AGENT_ADDRESS.clone() {
        return Err(ZomeApiError::from(format!("This offer is not for me")));
    }
    match offer.state {
        OfferState::Pending => Ok(()),
        _ => Err(ZomeApiError::from(format!("The offer must be pending"))),
    }?;

    hdk::commit_entry(&offer.entry())?;
    Ok(())
}
