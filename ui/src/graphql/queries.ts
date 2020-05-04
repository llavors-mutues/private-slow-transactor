import gql from 'graphql-tag';

export const GET_MY_BALANCE = gql`
  query GetMyBalance {
    myBalance
  }
`;

export const GET_MY_TRANSACTIONS = gql`
  query GetMyTransactions {
    me {
      id
    }
    myTransactions {
      id
      debtor {
        id
        username
      }
      creditor {
        id
        username
      }
      amount
      timestamp
    }
  }
`;

export const GET_PENDING_OFFERS = gql`
  query GetPendingOffers {
    me {
      id
    }
    myOffers {
      id
      transaction {
        id
        debtor {
          id
          username
        }
        creditor {
          id
          username
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
    me {
      id
    }
    
    offer(transactionId: $transactionId) {
      id
      transaction {
        id
        debtor {
          id
          username
        }
        creditor {
          id
          username
        }
        amount
        timestamp
      }

      counterpartySnapshot {
        executable
        valid
        balance
        lastHeaderId
      }

      state
    }
  }
`;

export const CREATE_OFFER = gql`
  mutation CreateOffer($creditorId: ID!, $amount: Float!) {
    createOffer(creditorId: $creditorId, amount: $amount)
  }
`;

export const ACCEPT_OFFER = gql`
  mutation AcceptOffer($transactionId: ID!, $approvedHeaderId: ID!) {
    acceptOffer(
      transactionId: $transactionId
      approvedHeaderId: $approvedHeaderId
    )
  }
`;
