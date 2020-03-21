use super::{sender, BalanceSnapshot, TransactionsSnapshot};
use crate::{
    accept_offer, attestation,
    message::{send_message, MessageBody, OfferMessage, OfferResponse},
    offer, transaction,
};
use hdk::{prelude::*, AGENT_ADDRESS};

/*** Receiver of the offer checks that the balance of the sender is correct with the credit limit or in his opinion ***/

/**
 * Get the transactions snapshot from the sender of the transaction
 * Then it returns offer balance, whether it's executable, and the last_header_address of the chain of that agent
 */
pub fn get_sender_balance(transaction_address: Address) -> ZomeApiResult<BalanceSnapshot> {
    let offer = offer::query_offer(&transaction_address)?;

    match offer.state {
        offer::OfferState::Pending => Ok(()),
        _ => Err(ZomeApiError::from(format!(
            "Offer is not pending: cannot get balance"
        ))),
    }?;

    let transactions_snapshot =
        match offer.transaction.sender_address.clone() == AGENT_ADDRESS.clone() {
            true => sender::get_my_transactions_snapshot(),
            false => {
                let snapshot = request_sender_transactions(
                    &transaction_address,
                    &offer.transaction.sender_address,
                )?;

                validate_snapshot_is_valid(&offer.transaction.sender_address, &snapshot)?;

                Ok(snapshot)
            }
        }?;

    let balance = transaction::compute_balance(
        &offer.transaction.sender_address,
        &transactions_snapshot.transactions,
    );

    // Add offer to the transaction list to verify that it is valid
    let mut transactions_to_validate = transactions_snapshot.transactions;
    transactions_to_validate.push(offer.transaction.clone());

    let executable = transaction::are_transactions_valid(
        &offer.transaction.sender_address,
        &transactions_to_validate,
    )?;

    Ok(BalanceSnapshot {
        sender_balance: balance,
        executable,
        last_header_address: transactions_snapshot.last_header_address,
    })
}

/**
 * Requests the transactions for the given offer_address from the sender_address agent, requesting their last header address for later validation
 */
fn request_sender_transactions(
    transaction_address: &Address,
    sender_address: &Address,
) -> ZomeApiResult<TransactionsSnapshot> {
    let message =
        MessageBody::GetTransactionsSnapshot(OfferMessage::Request(transaction_address.clone()));

    let result = send_message(sender_address.clone(), message)?;

    let response = match result {
        MessageBody::GetTransactionsSnapshot(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "Error getting the transaction for agent {}",
            sender_address
        ))),
    }?;

    match response {
        OfferResponse::OfferPending(transactions_snapshot) => Ok(transactions_snapshot),
        OfferResponse::OfferCompleted(transaction_proof) => {
            let offer = offer::query_offer(transaction_address)?;

            accept_offer::receiver::complete_transaction(offer.transaction, transaction_proof)?;
            Err(ZomeApiError::from(format!(
                "Transaction had already been executed"
            )))
        }
        OfferResponse::OfferCanceled => {
            offer::cancel_offer(transaction_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
    }
}

/**
 * Validate that the transaction snapshot received by the sender of the offer is valid with their attestations in the DHT
 */
fn validate_snapshot_is_valid(
    agent_address: &Address,
    transaction_snapshot: &TransactionsSnapshot,
) -> ZomeApiResult<()> {
    // Get transaction addresses for the agent
    let mut attestations = attestation::get_attestations_for_agent(&agent_address)?;

    attestation::validate_transactions_against_attestations(
        &mut attestations,
        &transaction_snapshot.transactions,
    )
}
