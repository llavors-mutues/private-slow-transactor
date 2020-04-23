import { HolochainProvider } from '@uprtcl/holochain-provider';

import { MutualCreditBindings } from '../bindings';

export const resolvers = {
  Transaction: {
    creditor(parent) {
      return parent.creditor_address;
    },
    debtor(parent) {
      return parent.debtor_address;
    },
  },
  Agent: {
    id(parent) {
      return parent;
    },
  },
  Query: {
    async myTransactions(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      return mutualCreditProvider.call('query_my_transactions', {});
    },
    async myOffers(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      const offers = await mutualCreditProvider.call('query_my_offers', {});
      console.log(offers);
      return offers.map((offer) => ({
        id: offer[0],
        ...offer[1].transaction,
        state: offer[1].state,
      }));
    },
    async myBalance(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      return mutualCreditProvider.call('query_my_balance', {});
    },
  },
  Mutation: {
    async createOffer(_, { creditorId, amount }, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      return mutualCreditProvider.call('create_offer', {
        creditor_address: creditorId,
        amount,
        timestamp: Math.floor(Date.now() / 1000),
      });
    },
    async acceptOffer(_, { transactionId }, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      return mutualCreditProvider.call('accept_offer', {
        transaction_address: transactionId,
      });
    },
  },
};
