import gql from 'graphql-tag';

export const mutualCreditTypeDefs = gql`
  scalar Date

  type Agent {
    id: ID!
  }

  enum TransactionState {
    RECEIVED
    PENDING
    DECLINED
    APPROVED
    COMPLETED
  }

  type Transaction {
    id: ID!

    debtor: Agent!
    creditor: Agent!
    amount: Float!
    timestamp: Date!

    executable: Boolean!
    counterpartyBalance: Float!
    valid: Boolean!

    state: TransactionState!
  }

  extend type Query {
    myTransactions: [Transaction!]!
    myOffers: [Transaction!]!
    myBalance: Float!
    transaction(transactionId: ID!): Transaction!
  }

  extend type Mutation {
    createOffer(creditorId: ID!, amount: Float!): ID!
    declineOffer(transactionId: ID!): ID!
    acceptOffer(transactionId: ID!): ID!
  }
`;
