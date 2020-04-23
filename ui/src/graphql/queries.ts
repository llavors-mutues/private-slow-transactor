import gql from 'graphql-tag';

export const GET_MY_TRANSACTIONS = gql`
  query GetMyTransactions {
    myTransactions {
      id
      debtor {
        id
      }
      creditor {
        id
      }
      amount
      timestamp
    }
  }
`;

export const GET_PENDING_OFFERS = gql`
  query GetPendingOffers {
    myOffers {
      id
      transaction {
        id
        debtor {
          id
        }
        creditor {
          id
        }
        amount
        timestamp
      }
      state
    }
  }
`;

export const GET_OFFER_DETAIL = gql`
  query GetOfferDetail($transactionId: String!) {
    offer(transactionId: $transactionId) {
      id
      transaction {
        id
        debtor {
          id
        }
        creditor {
          id
        }
        amount
        timestamp
      }

      counterpartySnapshot {
        executable
        valid
        balance
      }

      state

    }
  }

`

export const CREATE_OFFER = gql`
  mutation CreateOffer($creditorId: ID!, $amount: Float!) {
    createOffer(creditorId: $creditorId, amount: $amount)
  }
`;