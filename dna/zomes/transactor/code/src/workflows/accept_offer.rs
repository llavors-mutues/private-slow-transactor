use hdk::prelude::*;
use crate::{offer, message::*, transaction::Transaction};
use hdk::holochain_core_types::{chain_header::ChainHeader, signature::{Signature,Provenance}};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct AcceptOfferRequest {
    offer_address: Address,
    last_header_address: Address,
    timestamp: usize,
}

/**
 * Accepts the offer, verifying that the source chain of the sender agent has not changed,
 * and creating the transaction privately
 */
pub fn accept_offer(offer_address: Address, last_header_address: Address, timestamp: usize) -> ZomeApiResult<Address> {

    let accept_offer_request = AcceptOfferRequest {
        offer_address: offer_address.clone(),
        last_header_address: last_header_address.clone(),
        timestamp
    };

    let message = MessageBody::AcceptOffer(OfferMessage::Request(accept_offer_request));

    let response = match message {
        MessageBody::AcceptOffer(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!("AcceptOffer response is not valid")))
    }?;

    match response {
        OfferResponse::OfferPending((chain_header, signature)) => {
            let offer = offer::get_offer(&offer_address)?;
            let transaction = offer.to_transaction(timestamp);

            validate_transaction_header(chain_header, signature, &last_header_address, &transaction)?;


            offer::complete_offer(&offer_address)?;

            create_transaction_and_attestations(transaction)
        },
        OfferResponse::OfferNotPending => {
            offer::cancel_offer(&offer_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
    }
}

fn validate_transaction_header(chain_header: ChainHeader, signature: Signature, last_header_address: &Address, transaction: &Transaction) -> ZomeApiResult<()> {
    if chain_header.entry_address().clone() != transaction.address()? {
        return Err(ZomeApiError::from(format!("Received transaction address is not correct")));
    }
    
    if chain_header.link().unwrap() != last_header_address.clone() {
        return Err(ZomeApiError::from(format!("Received chain header does not reference the last viewed header: there are new transactions")));
    }

    let chain_header_address = chain_header.address();
    hdk::verify_signature(Provenance::new(transaction.sender_address.clone(), signature), chain_header_address)?;
    

    Ok(())
}

fn create_transaction_and_attestations(transaction: Transaction) -> ZomeApiResult<Address> {
    hdk::commit_entry(&transaction.entry())
}