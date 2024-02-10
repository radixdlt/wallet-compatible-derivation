# saehrimnir

This repo is a package containing two crates - a library named `saehrimnir` and binary named `bacon`.

```rust
extern crate saehrimnir;
use saehrimnir::prelude::*;

// Create an hierarchical deterministic derivation path.
let path = AccountPath::new(
	    NetworkID::Mainnet, // Mainnet or Stokenet (testnet)
	    0 // Account Index, 0 is first.
);

// 24 word BIP39 English mnemonic
let mnemonic: Mnemonic = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();

// Derive Babylon Radix account...
let account = derive_account(&mnemonic, "radix", &path).unwrap();

// ... containing the Account Address
assert_eq!(account.address, "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8");

// ... and its private key, public key, ....
assert_eq!(account.private_key.to_hex(), "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003");
```

# bacon
