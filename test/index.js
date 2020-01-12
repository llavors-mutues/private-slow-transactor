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

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,

    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor
  )
});

const dna = Config.dna(dnaPath, "scaffold-test");
const conductorConfig = Config.gen({ transactor: dna });

orchestrator.registerScenario("description of example test", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );

  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call("transactor", "transactor", "send_amount", {
    recipient_address: bob.instance("transactor").agentAddress,
    amount: 10,
    timestamp: Math.floor(Date.now() / 1000)
  });

  // Wait for all network activity to settle
  await s.consistency();

  t.ok(addr.Ok);
});

orchestrator.run();
