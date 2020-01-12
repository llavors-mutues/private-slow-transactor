import gql from 'graphql-tag';

export const GET_MY_TRANSACTION = gql`
  query GetMyTransactions() {
    myTransactions {
      id
      sender {
        id
      }
      receiver {
        id
      }
      amount
      timestamp
    }
  }
`;
