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
    amount: Int!
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
  }

  extend type Mutation {
    offerCredits(creditorId: ID!, amount: Float!): Transaction!
    declineOffer(transactionId: ID!): ID!
    acceptOffer(transactionId: ID!): ID!
  }
`;
