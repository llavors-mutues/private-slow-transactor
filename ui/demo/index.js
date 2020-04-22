import {
  MicroOrchestrator,
  i18nextBaseModule,
} from '@uprtcl/micro-orchestrator';
import { ApolloClientModule } from '@uprtcl/graphql';
import { MutualCreditModule } from '../dist/hc-mutual-credit.es5';
import {
  HolochainConnectionModule,
  HolochainConnection,
} from '@uprtcl/holochain-provider';
console.log('hihi');
(async function () {
  const connection = new HolochainConnection({ host: 'ws://localhost:8888' });

  const hcConnectionModule = new HolochainConnectionModule(connection);

  const mutualCredit = new MutualCreditModule('test-instance');

  const orchestrator = new MicroOrchestrator();
  await orchestrator.loadModules([
    new i18nextBaseModule(),
    new ApolloClientModule(),
    hcConnectionModule,
    mutualCredit,
  ]);
})();
