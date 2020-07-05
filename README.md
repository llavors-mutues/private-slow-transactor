# Private slow transactor

Mutual-credit holochain zome, implementing private transactions and slow peer-to-peer validation.

> This is a work in progress, not production ready. Contact us if you want to collaborate!

Design: https://hackmd.io/X9KFfDJZRS2vL9uLOq1oAg?both

## Usage 

You can clone this repository as a submodules in your `zomes` folder:

1. In your `zomes` folder, create a folder named `transactor`.
2. Clone this repository as a submodule: from the root folder of your DNA, run `git submodule add https://github.com/llavors-mutues/private-slow-transactor zomes/transactor`.
3. Check that the setup works by doing `hc package` from the root folder of your DNA.

You're ready to go! 

## Todo list:

- [x] Refactor code to use transactions as private entries and their headers to validate attestations by agents
- [x] Refactor to prevent "double-spending" (rolling your chain back and doing a new transaction) attack vectors
- [x] Create a reusable UI module
- [ ] Security audit to protect
- [ ] Generalize to include parameters such as: negative and positive credit limit, transaction size limit, etc.
- [ ] Publish to `npm` and `crates.io`?

## Developer setup

## Building

Run these commands:

```bash
nix-shell
cd example-dna
hc package
```

## Testing

Run these commands:

```bash
nix-shell
cd example-dna
hc test
```

## Running the UI

Requirements:

- You are inside the `nix-shell`.
- You have built the DNA as per `Building`.
- You are inside the UI folder.

Run this command to get two agents connected to each other ready to credit:

```bash
npm start
```
