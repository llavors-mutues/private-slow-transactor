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

export const CREATE_OFFER = gql`
  mutation CreateOffer($creditorId: ID!, $amount: Float!) {
    createOffer(creditorId: $creditorId, amount: $amount) {
      id
    }
  }
`;