# 💹 Substrate Standard Asset (SSA)

SSA is a asset tokenization standard for Substrate, based on and aims to be fully compatible with the (ASA standard of Algorand)[https://developer.algorand.org/docs/get-details/asa/].

## ASA Specification

### ASA ID

The ASA ID is a 64-bit unsigned integer. The ASA ID is used to identify the ASA in the Algorand network.

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

Note that for every asset an account creates or owns, its minimum balance is increased by 0.1 Algos (100,000 microAlgos)

#### Modifying an Asset

💂 Transaction authorizer: the asset manager.

#### Opting In to an Asset

💂 Transaction authorizer: any account with sufficient Algo balance.

#### Opting Out of an Asset

💂 Transaction authorizer: any account that has opted in to the asset.

#### Transferring an Asset

💂 Transaction authorizer: any account that has opted in to the asset and has sufficient (not-frozen) ASA balance plus ALGO balance to pay for transaction fee, plus the clawback address if the asset is frozen for the sender.

#### Freezing an Asset

💂 Transaction authorizer: the asset freeze address.

#### Revoking an Asset

💂 Transaction authorizer: the asset clawback address.

#### Destroying an Asset

💂 Transaction authorizer: the asset manager.

## References

- [Algorand ASA Standard](https://developer.algorand.org/docs/get-details/asa/)
- [ink!](https://use.ink/)
