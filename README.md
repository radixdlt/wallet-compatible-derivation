# saehrimnir

This repo is a package containing two crates - a library named `saehrimnir` and binary named `bacon`.

> [!IMPORTANT]  
> You are responsible for retaining sole possession and ownership of, and for securing
> the mnemonics (seed phrase(s)) you use with this software.

```rust
extern crate saehrimnir;
use saehrimnir::prelude::*;

// Create an hierarchical deterministic derivation path.
let path = AccountPath::new(
	NetworkID::Mainnet, // Mainnet or Stokenet (testnet)
	0 // Account Index, 0 is first.
);

// 24 word BIP39 English mnemonic
let mnemonic: Mnemonic24Words = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();

// Derive Babylon Radix account...
let account = Account::derive(
	&mnemonic, 
	"radix", // BIP39 passphrase (can be empty string)
	&path
);

// ... containing the Account Address
assert_eq!(account.address, "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8");

// ... and its private key, public key, ....
assert_eq!(account.private_key.to_hex(), "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003");
```

# bacon

**b**abylon **a**ccount **c**reati**on** - Tasty derivation using `saehrimnir`.
alt: **b**abylon **a**ccount **c**reation **o**n **n**etwork (accounts are virtual, so nothing is created "On Ledger", but rather we describe the fact that you select which network you wanna create the account for.)

`bacon` is a CLI tool using `saehrimnir`, allowing you to derive keys and account address from a Mnemonic, optional BIP39 passphrase, network id and an account index.

## pager

```sh
bacon pager
```

Which is the default, so you can just run:

```sh
bacon
```

Will start a pager, for security reasons, and this is **very much** the recommended way of running `bacon`.

### Demo

![pager](./.github/readme_assets/bacon_pager.gif)

## no-pager

> [!IMPORTANT]  
> This is not safe, your mnemonic and your derived keys WILL be present in your shells command history and output.
> ONLY Use this for mnemonics and accounts you really do not care about.

```sh
bacon no-pager --mnemonic "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote" --passphrase "" --network_id "mainnet" --index 1

Factor Source ID: 3bf4636876a9c795486194d2eaff32790961ed9005e18a7ebe677f0947b54087
Address: account_rdx129vlwaav373ucq6jewq6z722de5yd4ulklguv87u0ql0hmw5redatp
Network: Mainnet
HD Path: m/44H/1022H/1H/525H/1460H/1H
PrivateKey: af64f29665576e01e3fb10f9836e4b0fa066efe7a88f867f917be00620386f0b
PublicKey: bb09890daf2ed7a89bcd69eb56f56bc9208a37a147c1d9804db4f12d185a46a6
```

### Help

```sh
bacon no-pager --help
```
