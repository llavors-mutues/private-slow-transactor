import gql from 'graphql-tag';

export const mutualCreditTypeDefs = gql`
  scalar Date

  type Agent {
    id: ID!
  }

  type Transaction implements Entity {
    id: ID!

    sender: Agent!
    receiver: Agent!
    amount: Int!
    timestamp: Date!
  }

  extend type Query {
    myTransactions: [Transaction!]!
  }

  extend type Mutation {
    sendAmount(receiverId: ID!, amount: Int!): BadgeClass!
  }
`;
