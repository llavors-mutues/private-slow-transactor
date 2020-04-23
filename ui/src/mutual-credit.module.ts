import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/graphql';
import { MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
import {
  HolochainConnectionModule,
  createHolochainProvider,
} from '@uprtcl/holochain-provider';

import { CreateOffer } from './elements/hcmc-create-offer';
import { PendingOfferList } from './elements/hcmc-pending-offer-list';
import { TransactionList } from './elements/hcmc-transaction-list';

import en from './i18n/en.json';
import { mutualCreditTypeDefs } from './graphql/schema';
import { MutualCreditBindings } from './bindings';
import { resolvers } from './graphql/resolvers';
import { OfferDetail } from './elements/hcmc-offer-detail';

export class MutualCreditModule extends MicroModule {
  static id = Symbol('mutual-credit-module');

  dependencies = [HolochainConnectionModule.id];

  static bindings = MutualCreditBindings;

  constructor(protected instance: string) {
    super();
  }

  async onLoad(container: interfaces.Container) {
    const mutualCreditProvider = createHolochainProvider(
      this.instance,
      'transactor'
    );

    container
      .bind(MutualCreditBindings.MutualCreditProvider)
      .to(mutualCreditProvider);

    customElements.define('hcmc-transaction-list', TransactionList);
    customElements.define('hcmc-create-offer', CreateOffer);
    customElements.define('hcmc-pending-offer-list', PendingOfferList);
    customElements.define('hcmc-offer-detail', OfferDetail);
  }

  get submodules() {
    return [
      new GraphQlSchemaModule(mutualCreditTypeDefs, resolvers),
      new i18nextModule('mutual-credit', { en: en }),
    ];
  }
}
