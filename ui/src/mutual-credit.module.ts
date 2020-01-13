import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/common';
import {
  ElementsModule,
  MicroModule,
  i18nextModule
} from '@uprtcl/micro-orchestrator';
import {
  HolochainConnectionModule,
  createHolochainProvider
} from '@uprtcl/connections';

import { MyTransactions } from './elements/mutual-credit-my-transactions';

import en from '../i18n/en.json';
import { mutualCreditTypeDefs } from './graphql/schema';
import { MutualCreditTypes } from './types';
import { resolvers } from './graphql/resolvers';

export class MutualCreditModule extends MicroModule {
  static id = Symbol('mutual-credit-module');

  dependencies = [HolochainConnectionModule.id];

  static types = MutualCreditTypes;

  constructor(protected instance: string) {
    super();
  }

  async onLoad(container: interfaces.Container) {
    const mutualCreditProvider = createHolochainProvider(
      this.instance,
      'transactor'
    );

    container
      .bind(MutualCreditTypes.MutualCreditProvider)
      .to(mutualCreditProvider);
  }

  submodules = [
    new GraphQlSchemaModule(mutualCreditTypeDefs, resolvers),
    new i18nextModule('mutual-credit', { en: en }),
    new ElementsModule({
      'mutual-credit-my-transaction': MyTransactions
    })
  ];
}
