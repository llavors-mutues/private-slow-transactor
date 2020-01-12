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
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/hc-mutual-credit.dna.json");

const dna = Config.dna(dnaPath, "scaffold-test");
const conductorConfig = Config.gen(
  { transactor: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);

const orchestrator = new Orchestrator({
  waiter: {
    softTimeout: 20000,
    hardTimeout: 30000
  }
});

function sendAmount(to, amount) {
  return caller =>
    caller.call("transactor", "transactor", "send_amount", {
      receiver_address: to,
      amount,
      timestamp: Math.floor(Date.now() / 1000)
    });
}

orchestrator.registerScenario("description of example test", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );

  const aliceAddress = alice.instance("transactor").agentAddress;
  const bobAddress = bob.instance("transactor").agentAddress;

  let result = await sendAmount(bobAddress, 10)(alice);
  await s.consistency();
  t.ok(result.Ok);

  result = await sendAmount(bobAddress, 10)(alice);
  await s.consistency();
  t.ok(result.Ok);

  result = await sendAmount(aliceAddress, 10)(bob);
  await s.consistency();
  t.ok(result.Ok);

  result = await sendAmount(bobAddress, 91)(alice);
  await s.consistency();
  t.notOk(result.Ok);
});

orchestrator.run();
