# 💹 Substrate Standard Asset (SSA)

SSA is a asset tokenization standard for Substrate, based on and aims to be fully compatible with the (ASA standard of Algorand)[https://developer.algorand.org/docs/get-details/asa/].

## ASA Specification

### ASA ID

The ASA ID is a 64-bit unsigned integer. The ASA ID is used to identify the ASA in the Algorand network.

```rust
pub type AssetId = u64;
```

### ASA Parameters

#### ASA Immutable Parameters

- `total`: total number of tokens
- `decimals`: number of decimals
- `default_frozen`: whether the asset is frozen by default
- `unit_name`: name of a single unit of the asset
- `asset_name`: name of the asset
- `url`: URL where more information about the asset can be retrieved
- `metadata_hash`: a commitment to some unspecified asset metadata

#### ASA Mutable Parameters

- `manager`: address that can change reserve, freeze, clawback, and manager
- `reserve`: address that holds reserve (non-minted) tokens
- `freeze`: address that can freeze or unfreeze user asset holdings
- `clawback`: address that can revoke user asset holdings and send them to other addresses

### ASA Functions

- `create`: create a new ASA
- `modify`: modify the parameters of an ASA
- `opt_in`: opt-in to receive an ASA
- `opt_out`: opt-out of receiving an ASA
- `transfer`: transfer tokens between accounts
- `freeze`: freeze or unfreeze an account's asset holdings
- `revoke`: revoke tokens from an account and send them to another account
- `destroy`: destroy an ASA

#### Creating an Asset

💂 Transaction authorizer: any account with sufficient Algo balance.

Note that for every asset an account creates or owns, its minimum balance is increased by 0.1 Algos (100,000 microAlgos).

##### Asset Creation Transaction

```rust
pub struct AssetCreationTransaction {
    /// The total number of units of this asset.
    pub total: u64,
    /// The number of digits to use after the decimal point when displaying this asset.
    pub decimals: u8,
    /// Whether users of this asset must opt in before holding or sending.
    pub default_frozen: bool,
    /// The name of this asset.
    pub unit_name: String,
    /// The full name of this asset.
    pub asset_name: String,
    /// A URL where more information about the asset can be retrieved.
    pub url: String,
    /// A commitment to some unspecified asset metadata.
    pub metadata_hash: String,
    /// The address that can change the reserve, freeze, clawback, and manager addresses of the asset.
    pub manager: AccountId,
    /// The address that holds reserve (non-minted) units of the asset.
    pub reserve: AccountId,
    /// The address that can freeze or unfreeze user asset holdings.
    pub freeze: AccountId,
    /// The address that can revoke user asset holdings and send them to other addresses.
    pub clawback: AccountId,
}
```

##### Asset Creation Event

```rust
#[ink(event)]
pub struct Creation {
    #[ink(topic)]
    asset_id: AssetId,
    #[ink(topic)]
    asset_name: String,
    #[ink(topic)]
    creator: AccountId,
    #[ink(topic)]
    total: Balance,
}
```

#### Modifying an Asset

💂 Transaction authorizer: the asset manager.

##### Asset Modification Transaction

```rust
pub struct AssetModificationTransaction {
    /// The address that can change the reserve, freeze, clawback, and manager addresses of the asset.
    pub manager: Option<AccountId>,
    /// The address that holds reserve (non-minted) units of the asset.
    pub reserve: Option<AccountId>,
    /// The address that can freeze or unfreeze user asset holdings.
    pub freeze: Option<AccountId>,
    /// The address that can revoke user asset holdings and send them to other addresses.
    pub clawback: Option<AccountId>,
}
```

##### Asset Modification Event

```rust
#[ink(event)]
pub struct Modification {
    #[ink(topic)]
    asset_id: AssetId,
    #[ink(topic)]
    manager: Option<AccountId>,
    #[ink(topic)]
    reserve: Option<AccountId>,
    #[ink(topic)]
    freeze: Option<AccountId>,
    #[ink(topic)]
    clawback: Option<AccountId>,
}
```

#### Opting In to an Asset

💂 Transaction authorizer: any account with sufficient Algo balance.

##### Asset Opt-In Transaction

```rust
pub struct AssetOptInTransaction {
    /// The ASA ID of the asset to opt in to.
    pub asset_id: u64,
    /// The address to opt in to the asset.
    pub to: AccountId,
}
```

##### Asset Opt-In Event

```rust
#[ink(event)]
pub struct OptIn {
    #[ink(topic)]
    account: AccountId,
    #[ink(topic)]
    asset_id: AssetId,
}
```

#### Opting Out of an Asset

💂 Transaction authorizer: any account that has opted in to the asset.

##### Asset Opt-Out Transaction

```rust
pub struct AssetOptOutTransaction {
    /// The ASA ID of the asset to opt out of.
    pub asset_id: u64,
}
```

##### Asset Opt-Out Event

```rust
#[ink(event)]
pub struct OptOut {
    #[ink(topic)]
    account: AccountId,
    #[ink(topic)]
    asset_id: AssetId,
}
```

#### Transferring an Asset

💂 Transaction authorizer: any account that has opted in to the asset and has sufficient (not-frozen) ASA balance plus ALGO balance to pay for transaction fee, plus the clawback address if the asset is frozen for the sender.

##### Asset Transfer Transaction

```rust
pub struct AssetTransferTransaction {
    /// The ASA ID of the asset to transfer.
    pub asset_id: u64,
    /// The address to transfer the asset to.
    pub to: AccountId,
    /// The amount of the asset to transfer.
    pub amount: u64,
    /// The address to transfer the asset from.
    pub from: AccountId,
}
```

##### Asset Transfer Event

```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    sender: AccountId,
    #[ink(topic)]
    receiver: AccountId,
    #[ink(topic)]
    asset_id: AssetId,
    #[ink(topic)]
    amount: Option<Balance>,
}
```

#### Freezing (and Unfreezing) an Asset

💂 Transaction authorizer: the asset freeze address.

##### Asset Freeze Transaction

```rust
pub struct AssetFreezeTransaction {
    /// The ASA ID of the asset to freeze.
    pub asset_id: u64,
    /// The address to freeze the asset for.
    pub account: AccountId,
    /// Whether to freeze or unfreeze the asset.
    pub freeze: bool,
}
```

##### Asset Freeze Event

```rust
#[ink(event)]
pub struct Freeze {
    #[ink(topic)]
    account: AccountId,
    #[ink(topic)]
    asset_id: AssetId,
    #[ink(topic)]
    freeze: bool,
}
```

#### Revoking an Asset

💂 Transaction authorizer: the asset clawback address.

##### Asset Revoke Transaction

```rust
pub struct AssetRevokeTransaction {
    /// The ASA ID of the asset to revoke.
    pub asset_id: u64,
    /// The address to revoke the asset from.
    pub account: AccountId,
    /// The amount of the asset to revoke.
    pub amount: u64,
}
```

##### Asset Revoke Event

```rust
#[ink(event)]
pub struct Revoke {
    #[ink(topic)]
    account: AccountId,
    #[ink(topic)]
    asset_id: AssetId,
    #[ink(topic)]
    amount: Option<Balance>,
}
```

#### Destroying an Asset

💂 Transaction authorizer: the asset manager.

##### Asset Destroy Transaction

```rust
pub struct AssetDestroyTransaction {
    /// The ASA ID of the asset to destroy.
    pub asset_id: u64,
}
```

##### Asset Destroy Event

```rust
#[ink(event)]
pub struct Destruction {
    #[ink(topic)]
    asset_id: AssetId,
    #[ink(topic)]
    destroyer: AccountId,
}
```

## References

- [Algorand ASA Standard](https://developer.algorand.org/docs/get-details/asa/)
- [ink!](https://use.ink/)

## Testing 🧪

### Cargo unit tests

```bash
cargo test
```

### Testing in Rococo testnet

Rococo is a testnet for Polkadot and Kusama parachains. We have a live testnet named Contracts as a parachain online there. You can test SSA contract on Contracts parachain.

#### Steps

1. Create an account. This can be done via command-line tools (e.g. subxt) or via a wallet (e.g. with the polkadot-js browser extension). See [here](https://wiki.polkadot.network/docs/learn-account-generation) for a detailed guide.

2. Get some testnet tokens. You can get some testnet tokens from the [faucet](https://use.ink/faucet).

Alternatively, you can use the [Element chat room](https://wiki.polkadot.network/docs/learn-DOT#getting-tokens-on-the-rococo-testnet). You must send a message like this (Note the :1002 after the wallet address):

```
!drip YOUR_SS_58_ADDRESS:1002
```

The number 1002 is the parachain ID of Contracts on Rococo, by supplying it you instruct the faucet to teleport ROC tokens directly to your account on the parachain. If you have some tokens on the Rococo relay chain, you can teleport them to the Contracts parachain on your own. Read more on teleporting assets [here](https://wiki.polkadot.network/docs/learn-teleport).

3. Deploy SSA contract. You can deploy the contract via the [Contracts UI](https://use.ink/testnet#3-deploy-your-contract) or from the command-line via `cargo-contract`. Make sure you are in the folder of your contract and that it has been built recently. Then execute:

```bash
cargo contract upload --suri "your twelve or twenty-four words"
cargo contract instantiate --suri … --constructor new --args true
```

`new` in this case would be a constructor method exposed by the contract, `--args` would be any arguments the constructor expects.

##### --args

```rust
#[ink(constructor)]
        pub fn new(
            asset_name: String,
            unit_name: String,
            total: Balance,
            decimals: u32,
            default_frozen: bool,
            url: String,
            metadata_hash: [u8; 4],
            manager: Option<AccountId>,
            reserve: Option<AccountId>,
            freeze: Option<AccountId>,
            clawback: Option<AccountId>,
        ) -> Self {
            Self {
                creator: Self::env().caller(),
                asset_name,
                unit_name,
                total,
                decimals,
                default_frozen,
                url,
                metadata_hash,
                manager: manager.unwrap_or_else(|| AccountId::from([0x0; 32])),
                reserve: reserve.unwrap_or_else(|| AccountId::from([0x0; 32])),
                freeze: freeze.unwrap_or_else(|| AccountId::from([0x0; 32])),
                clawback: clawback.unwrap_or_else(|| AccountId::from([0x0; 32])),
                all_holders: Mapping::default(),
                accounts_opted_in: Mapping::default(),
                frozen_holders: Mapping::default(),
            }
        }
```

- [Rococo testnet](hhttps://wiki.polkadot.network/docs/build-pdk#rococo-testnet)
- [Testnet faucet](https://use.ink/faucet)
- [Contract deployment](https://use.ink/testnet#3-deploy-your-contract)
