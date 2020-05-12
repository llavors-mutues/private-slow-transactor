import gql from 'graphql-tag';

export const mutualCreditTypeDefs = gql`
  scalar Date

  enum OfferState {
    Received
    Pending
    Canceled
    Approved
    Completed
  }

  type Transaction {
    id: ID!

    debtor: Agent!
    creditor: Agent!
    amount: Float!
    timestamp: Date!
  }

  type CounterpartySnapshot {
    executable: Boolean!
    balance: Float!
    invalidReason: String
    valid: Boolean!
    lastHeaderId: ID!
  }

  type Counterparty {
    online: Boolean!
    consented: Boolean
    snapshot: CounterpartySnapshot
  }

  type Offer {
    id: ID!

    transaction: Transaction!

    counterparty: Counterparty!

    state: OfferState!
  }

  extend type Me {
    transactions: [Transaction!]!
    offers: [Offer!]!
    balance: Float!
  }

  extend type Query {
    offer(transactionId: ID!): Offer!
  }

  extend type Mutation {
    createOffer(creditorId: ID!, amount: Float!): ID!
    consentForOffer(transactionId: ID!): ID!
    cancelOffer(transactionId: ID!): ID!
    acceptOffer(transactionId: ID!, approvedHeaderId: ID!): ID!
  }
`;
