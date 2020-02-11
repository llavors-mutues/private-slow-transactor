# hc-mutual-credit
Generic mutual-credit implementation in Holochain

> Work in progress: This is not production ready!

This zome is a generic implementation of a mutual credit system. It's just a piece of the puzzle, and maybe best used with some other zome that implements better membranes.

Next tasks:

* [ ] Security audit to protect from "double-spending" (rolling your chain back and doing a new transaction) attack vectors
* [ ] Create a reusable UI module
* [ ] Add the ability to pass in the currency name and credit limit as parameter
* [ ] Make it optional to have the entries private

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

