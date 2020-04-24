/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor,
} = require("@holochain/tryorama");

process.on("unhandledRejection", (error) => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/dna.dna.json");

const dna = Config.dna(dnaPath, "scaffold-test");
const conductorConfig = Config.gen(
  { transactor: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000",
    },
  }
);

const orchestrator = new Orchestrator({
  waiter: {
    softTimeout: 20000,
    hardTimeout: 30000,
  },
});

function offerCredits(to, credits) {
  return (caller) =>
    caller.call("transactor", "transactor", "create_offer", {
      creditor_address: to,
      amount: credits,
      timestamp: Math.floor(Date.now() / 1000),
    });
}

function getCounterpartyBalance(transactionAddress) {
  return (caller) =>
    caller.call("transactor", "transactor", "get_counterparty_balance", {
      transaction_address: transactionAddress,
    });
}

function acceptOffer(transactionAddress, lastHeaderAddress) {
  return (caller) =>
    caller.call("transactor", "transactor", "accept_offer", {
      transaction_address: transactionAddress,
      approved_header_address: lastHeaderAddress,
    });
}

orchestrator.registerScenario("description of example test", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );

  const aliceAddress = alice.instance("transactor").agentAddress;
  const bobAddress = bob.instance("transactor").agentAddress;

  let result = await offerCredits(bobAddress, 10)(alice);
  await s.consistency();
  t.ok(result.Ok);

  let transactionAddress = result.Ok;

  result = await getCounterpartyBalance(transactionAddress)(alice);
  t.equal(result.Ok.balance, 0);
  t.equal(result.Ok.executable, true);

  result = await getCounterpartyBalance(transactionAddress)(bob);
  t.equal(result.Ok.balance, 0);
  t.equal(result.Ok.executable, true);

  result = await acceptOffer(
    transactionAddress,
    result.Ok.last_header_address
  )(bob); // Alice has -10, Bob has +10
  await s.consistency();
  t.ok(result);

  result = await offerCredits(bobAddress, 10)(alice);
  await s.consistency();
  t.ok(result.Ok);

  transactionAddress = result.Ok;

  result = await getCounterpartyBalance(transactionAddress)(alice);
  t.equal(result.Ok.balance, 10);
  t.equal(result.Ok.executable, true);

  result = await getCounterpartyBalance(transactionAddress)(bob);
  t.equal(result.Ok.balance, -10);
  t.equal(result.Ok.executable, true);
  t.equal(result.Ok.valid, true);

  result = await acceptOffer(
    transactionAddress,
    result.Ok.last_header_address
  )(bob); // Alice has -20, Bob has +20
  await s.consistency();
  t.ok(result);

  result = await offerCredits(bobAddress, 81)(alice);
  await s.consistency();
  t.ok(result.Ok);

  transactionAddress = result.Ok;

  result = await getCounterpartyBalance(transactionAddress)(bob);
  t.equal(result.Ok.balance, -20);
  t.equal(result.Ok.executable, false);
  t.equal(result.Ok.valid, true);

  result = await acceptOffer(
    transactionAddress,
    result.Ok.last_header_address
  )(bob);
  await s.consistency();
  t.notOk(result.Ok);

  result = await getCounterpartyBalance(transactionAddress)(bob);
  t.notOk(result.Ok);
});

orchestrator.run();
