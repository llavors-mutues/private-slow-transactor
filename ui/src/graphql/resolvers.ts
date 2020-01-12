import { HolochainProvider } from '@uprtcl/connections';

import { MutualCreditTypes } from '../types';

export const resolvers = {
  Transaction: {
    id(parent) {
      return parent;
    }
  },
  Query: {
    async myTransactions(_, __, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditTypes.MutualCreditProvider
      );

      return mutualCreditProvider.call('get_my_transactions', {});
    }
  },
  Mutation: {
    async sendAmount(_, { receiverId, amount }, { container }) {
      const mutualCreditProvider: HolochainProvider = container.get(
        MutualCreditTypes.MutualCreditProvider
      );

      return mutualCreditProvider.call('send_amount', {
        receiver_address: receiverId,
        amount,
        timestamp: Math.floor(Date.now() / 1000)
      });
    }
  }
};
