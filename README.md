# saehrimnir 🐖

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

## Etymology
[🇸🇪 **Särimner** (🇬🇧 _Sæhrímnir_)](https://en.wikipedia.org/wiki/S%C3%A6hr%C3%ADmnir) is the eternal pig in **Asatro** (_Norse Mythology_)]([Norse_mythology](https://en.wikipedia.org/wiki/Norse_mythology)), that gets eaten every night by **Asarna** (_the gods_) and fallen heros brought to **Valhall** (_Valhalla_) and is brought back to life again to provide sustenance for the following day. A source of infinite pork. Read: an infinite source of sustenance - just like hierarchical deterministic account derivation!

# bacon 🥓

**b**abylon **a**ccount **c**reati**on**
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

## no-pager

> [!IMPORTANT]  
> This is not safe, your mnemonic and your derived keys WILL be present in your shells command history and output.
> ONLY Use this for mnemonics and accounts you really do not care about.

```sh
bacon no-pager \
--mnemonic  "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote" \
--passphrase "secret" \
--network "mainnet" \
--start 1 \
--count 2
```

### Short

```sh
bacon no-pager -m  "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote" -p "secret" -n "mainnet" -s 1 -c 2
```

### Help

```sh
bacon no-pager --help
```

## Etymology
Tasty derivative of `saehrimnir` 🐖, the infinite source of pork.

# License

```
Copyright 2023 Radix Publishing Limited

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software for non-production informational and educational purposes without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

This notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL AND EDUCATIONAL PURPOSES ONLY. 

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES, COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE. 

```