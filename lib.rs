#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// SSA smart contract
#[ink::contract]
mod ssa {
    use ink_storage::{traits::SpreadAllocate, Mapping};

    use scale::{Decode, Encode};

    pub type AssetId = u64;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Ssa {
        // immutable asset params
        creator: AccountId,
        asset_name: String,
        unit_name: String,
        total: Balance,
        decimals: u32,
        default_frozen: bool,
        url: String,
        metadata_hash: [u8; 4],
        // mutable asset params
        manager: AccountId,
        reserve: AccountId,
        freeze: AccountId,
        clawback: AccountId,
        all_holders: Mapping<AccountId, u32>,
        accounts_opted_in: Mapping<AccountId, bool>,
        frozen_holders: Mapping<AccountId, u32>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotManager,
        NotReserve,
        NotFreeze,
        NotClawback,
        NotOptedIn,
        NotFrozen,
        Frozen,
        NotEnoughBalance,
        ZeroAmount,
    }

    /// Event emitted when a token transfer occurs.
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

    /// Event emitted when an asset is created.
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

    /// Event emitted when an asset is frozen.
    /// Note: only the freeze account can freeze an account.
    #[ink(event)]
    pub struct Freeze {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        freeze_address: AccountId,
        #[ink(topic)]
        freeze: bool,
    }

    /// Event emitted when an asset is reconfigured.
    /// Note: only the manager can reconfigure an asset.
    /// Note: the manager can change the reserve, freeze, and clawback addresses.
    #[ink(event)]
    pub struct Modify {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        reserve: AccountId,
        #[ink(topic)]
        freeze: AccountId,
        #[ink(topic)]
        clawback: AccountId,
    }

    /// Event emitted when an account opts in to receive an asset.
    #[ink(event)]
    pub struct OptIn {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        account: AccountId,
    }

    /// Event emitted when an account opts out of receiving an asset.
    /// Note: only accounts that have opted in can opt out.
    #[ink(event)]
    pub struct OptOut {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        account: AccountId,
    }

    /// Event emitted when an asset is revoked.
    /// Note: only the manager address can revoke an asset.
    #[ink(event)]
    pub struct Revoke {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        clawback: AccountId,
        #[ink(topic)]
        amount: Option<Balance>,
    }

    /// Event emitted when an asset is destroyed.
    /// Note: this can only happen if there are no remaining asset holdings.
    /// Note: only the manager can destroy an asset.
    #[ink(event)]
    pub struct Destruction {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        destroyer: AccountId,
    }

    impl Ssa {
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

        /// A message that returns all the data of the asset.
        #[ink(message)]
        pub fn transfer_from(&mut self) {}
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {}
    }
}
