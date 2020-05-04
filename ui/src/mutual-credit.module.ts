import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/graphql';
import { MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
import {
  HolochainConnectionModule,
  createHolochainProvider,
} from '@uprtcl/holochain-provider';
import { ProfilesModule } from 'holochain-profiles';

import { MCCreateOffer } from './elements/hcmc-create-offer';
import { MCPendingOfferList } from './elements/hcmc-pending-offer-list';
import { MCTransactionList } from './elements/hcmc-transaction-list';

import en from './i18n/en.json';
import { mutualCreditTypeDefs } from './graphql/schema';
import { MutualCreditBindings } from './bindings';
import { resolvers } from './graphql/resolvers';
import { MCOfferDetail } from './elements/hcmc-offer-detail';
import { GetAllowedCreditors, allAgentsAllowed } from './types';
import { MCAgentList } from './elements/hcmc-agent-list';

export class MutualCreditModule extends MicroModule {
  static id = 'mutual-credit-module';

  dependencies = [HolochainConnectionModule.id, ProfilesModule.id];

  static bindings = MutualCreditBindings;

  constructor(
    protected instance: string,
    protected agentFilter: GetAllowedCreditors = allAgentsAllowed
  ) {
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

    container
      .bind(MutualCreditBindings.ValidAgentFilter)
      .toConstantValue(this.agentFilter);

    customElements.define('hcmc-transaction-list', MCTransactionList);
    customElements.define('hcmc-create-offer', MCCreateOffer);
    customElements.define('hcmc-pending-offer-list', MCPendingOfferList);
    customElements.define('hcmc-offer-detail', MCOfferDetail);
    customElements.define('hcmc-agent-list', MCAgentList);
  }

  get submodules() {
    return [
      new GraphQlSchemaModule(mutualCreditTypeDefs, resolvers),
      new i18nextModule('mutual-credit', { en: en }),
    ];
  }
}
