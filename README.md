# Holochain generic mutual-credit

Generic mutual-credit implementation in Holochain.

This zome is a generic implementation of a mutual credit system. It's just a piece of the puzzle, and maybe best used with some other zome that implements better membranes.

> This is a work in progress, not production ready. Contact us if you want to collaborate!

Design: https://hackmd.io/X9KFfDJZRS2vL9uLOq1oAg?both

## Todo list:

* [x] Refactor code to use transactions as private entries and their headers to validate attestations by agents
* [ ] Security audit to protect from "double-spending" (rolling your chain back and doing a new transaction) attack vectors
* [ ] Create a reusable UI module
* [ ] Add the ability to pass in the currency name and credit limit as parameter
* [ ] Publish to `npm` and `crates.io`?

## Building

Run these commands:

```bash
nix-shell
cd dna
hc package
```
## Testing

Run these commands:

```bash
nix-shell
cd dna
hc test
```

