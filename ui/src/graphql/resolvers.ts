import { HolochainProvider } from '@uprtcl/holochain-provider';

import { MutualCreditBindings } from '../bindings';

function offerToTransaction(id, offer) {
  const state = offer.state;
  return {
    id,
    transaction: {
      id,
      ...offer.transaction,
    },
    state: typeof state === 'object' ? Object.keys(state)[0] : state,
  };
}

export const resolvers = {
  Transaction: {
    creditor(parent) {
      return { id: parent.creditor_address };
    },
    debtor(parent) {
      return { id: parent.debtor_address };
    },
  },
  Offer: {
    async counterpartySnapshot(parent, _, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      return mutualCreditProvider.call('get_counterparty_snapshot', {
        transaction_address: parent.id,
      });
    },
  },
  CounterpartySnapshot: {
    lastHeaderId(parent) {
      return parent.last_header_address;
    },
  },
  Query: {
    async offer(_, { transactionId }, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      const offer = await mutualCreditProvider.call('query_offer', {
        transaction_address: transactionId,
      });
      return offerToTransaction(transactionId, offer);
    },
    async myTransactions(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      const transactions = await mutualCreditProvider.call(
        'query_my_transactions',
        {}
      );
      return transactions.map((t) => ({ id: t[0], ...t[1] }));
    },
    async myOffers(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      const offers = await mutualCreditProvider.call('query_my_offers', {});
      console.log(offers);
      return offers.map((offer) => offerToTransaction(offer[0], offer[1]));
    },
    async myBalance(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      const result = await mutualCreditProvider.call('query_my_balance', {});
      return result.hasOwnProperty('Ok') ? result.Ok : result;
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
    async acceptOffer(_, { transactionId, approvedHeaderId }, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditBindings.MutualCreditProvider
      );

      await mutualCreditProvider.call('accept_offer', {
        transaction_address: transactionId,
        approved_header_address: approvedHeaderId,
      });

      return transactionId;
    },
  },
};
