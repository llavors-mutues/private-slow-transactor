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

console.log(process.env);
(async function () {
  const connection = new HolochainConnection({
    host: `ws://localhost:${process.env.HOST}`,
  });

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
