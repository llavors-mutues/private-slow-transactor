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
