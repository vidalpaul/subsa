#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// subsa smart contract
#[ink::contract]
mod subsa {
    use ink_storage::{traits::SpreadAllocate, Mapping};

    use scale::{Decode, Encode};

    pub type AssetId = AccountId;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Subsa {
        // immutable asset params
        asset_id: AssetId,
        creator: AccountId,
        asset_name: String,
        unit_name: String,
        total: Balance,
        decimals: u32,
        default_frozen: bool,
        url: String,
        metadata_hash: [u8; 4],
        // mutable asset params
        manager_id: AccountId,
        reserve_id: AccountId,
        freeze_id: AccountId,
        clawback_id: AccountId,
        balances: Mapping<AccountId, Balance>,
        accounts_opted_in: Mapping<AccountId, bool>,
        frozen_holders: Mapping<AccountId, bool>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotManagerId,
        NotReserveId,
        NotFreezeId,
        NotClawbackId,
        NotOptedIn,
        AlreadyOptedIn,
        NotFrozen,
        NotFreezable,
        AlreadyFrozen,
        FrozenAccount,
        NotEnoughBalance,
        NotAllAssetsOwnedByManager,
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
        freeze_id: AccountId,
        #[ink(topic)]
        freeze: bool,
    }

    /// Event emitted when an asset is reconfigured.
    /// Note: only the manager can reconfigure an asset.
    /// Note: the manager can change the reserve, freeze, and clawback addresses.
    #[ink(event)]
    pub struct Modify {
        #[ink(topic)]
        manager_id: AccountId,
        #[ink(topic)]
        reserve_id: AccountId,
        #[ink(topic)]
        freeze_id: AccountId,
        #[ink(topic)]
        clawback_id: AccountId,
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
        clawback_id: AccountId,
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

    impl Subsa {
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
                asset_id: Self::env().account_id(),
                manager_id: manager.unwrap_or_else(|| AccountId::from([0x0; 32])),
                reserve_id: reserve.unwrap_or_else(|| AccountId::from([0x0; 32])),
                freeze_id: freeze.unwrap_or_else(|| AccountId::from([0x0; 32])),
                clawback_id: clawback.unwrap_or_else(|| AccountId::from([0x0; 32])),
                balances: Mapping::default(),
                accounts_opted_in: Mapping::default(),
                frozen_holders: Mapping::default(),
            }
        }

        /// Returns the asset name.
        #[ink(message)]
        pub fn asset_name(&self) -> String {
            self.asset_name.clone()
        }

        /// Returns the asset unit name.
        #[ink(message)]
        pub fn unit_name(&self) -> String {
            self.unit_name.clone()
        }

        /// Returns the total supply of the asset.
        #[ink(message)]
        pub fn total(&self) -> Balance {
            self.total
        }

        /// Returns the number of decimals used to display the asset.
        #[ink(message)]
        pub fn decimals(&self) -> u32 {
            self.decimals
        }

        /// Returns whether the asset is frozen by default.
        #[ink(message)]
        pub fn default_frozen(&self) -> bool {
            self.default_frozen
        }

        /// Returns the URL of the asset.
        #[ink(message)]
        pub fn url(&self) -> String {
            self.url.clone()
        }

        /// Returns the metadata hash of the asset.
        #[ink(message)]
        pub fn metadata_hash(&self) -> [u8; 4] {
            self.metadata_hash
        }

        /// Returns the asset ID.
        /// Note: the asset ID is the address of the contract.
        #[ink(message)]
        pub fn asset_id(&self) -> AccountId {
            self.asset_id
        }

        /// Returns the manager address.
        #[ink(message)]
        pub fn manager_id(&self) -> AccountId {
            self.manager_id
        }

        /// Returns the reserve address.
        #[ink(message)]
        pub fn reserve_id(&self) -> AccountId {
            self.reserve_id
        }

        /// Returns the freeze address.
        #[ink(message)]
        pub fn freeze_id(&self) -> AccountId {
            self.freeze_id
        }

        /// Returns the clawback address.
        #[ink(message)]
        pub fn clawback_id(&self) -> AccountId {
            self.clawback_id
        }

        /// Returns the balance of `account`.
        /// Note: if the account has not opted in to this asset, NotOptedIn is returned.
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Result<Balance, Error> {
            let opted_in = self.accounts_opted_in.get(&account).unwrap_or(false);
            if !opted_in {
                return Err(Error::NotOptedIn);
            }

            Ok(self.balances.get(&account).unwrap_or(0))
        }

        /// Returns whether `account` is frozen.
        #[ink(message)]
        pub fn is_frozen(&self, account: AccountId) -> Result<bool, Error> {
            Ok(self.frozen_holders.get(&account).unwrap_or(false))
        }

        /// Returns whether `account` has opted in to this asset.
        #[ink(message)]
        pub fn is_opted_in(&self, account: AccountId) -> Result<bool, Error> {
            Ok(self.accounts_opted_in.get(&account).unwrap_or(false))
        }

        /// Returns wheter `creator's balance is equal to total supply.
        /// Note: an asset can only be destroyed if the creator's balance is equal to the total supply.
        #[ink(message)]
        pub fn is_destroyable(&self) -> bool {
            self.balances.get(&self.creator).unwrap_or(0) == self.total
        }

        /// Transfer `amount` of tokens from `sender` to `receiver`.
        #[ink(message)]
        pub fn transfer(&mut self, receiver: AccountId, amount: Balance) -> Result<(), Error> {
            let sender = self.env().caller();

            // check if sender has enough balance
            let sender_balance = self.balances.get(&sender).unwrap_or(0);
            if sender_balance < amount {
                return Err(Error::NotEnoughBalance);
            }

            // check if receiver has opted in
            let receiver_opted_in = self.accounts_opted_in.get(&receiver).unwrap_or(false);
            if !receiver_opted_in {
                return Err(Error::NotOptedIn);
            }

            // update sender and receiver balances
            self.balances.insert(&sender, &(sender_balance - amount));
            self.balances.insert(
                &receiver,
                &(self.balances.get(&receiver).unwrap_or(0) + amount),
            );

            // emit transfer event
            self.env().emit_event(Transfer {
                sender,
                receiver,
                asset_id: self.asset_id,
                amount: Some(amount),
            });

            Ok(())
        }

        // OptIn to receive an asset
        #[ink(message)]
        pub fn opt_in(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();

            // check if caller has already opted in
            let caller_opted_in = self.accounts_opted_in.get(&caller).unwrap_or(false);
            if caller_opted_in {
                return Err(Error::AlreadyOptedIn);
            }

            // update caller's opt in status
            self.accounts_opted_in.insert(&caller, &true);

            // emit opt in event
            self.env().emit_event(OptIn {
                asset_id: self.asset_id,
                account: caller,
            });

            Ok(())
        }

        // OptOut of receiving an asset
        #[ink(message)]
        pub fn opt_out(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();

            // check if caller has opted in
            let caller_opted_in = self.accounts_opted_in.get(&caller).unwrap_or(false);
            if !caller_opted_in {
                return Err(Error::NotOptedIn);
            }

            // update caller's opt in status
            self.accounts_opted_in.insert(&caller, &false);

            // emit opt out event
            self.env().emit_event(OptOut {
                asset_id: self.asset_id,
                account: caller,
            });

            Ok(())
        }

        // Freeze an account
        #[ink(message)]
        pub fn freeze(&mut self, account: AccountId, freeze: bool) -> Result<(), Error> {
            let caller = self.env().caller();

            // check if token can be frozen
            if !self.default_frozen {
                return Err(Error::NotFreezable);
            }

            // check if caller is the freeze address
            if caller != self.freeze_id {
                return Err(Error::NotFreezeId);
            }

            // check if account is already frozen
            let account_frozen = self.frozen_holders.get(&account).unwrap_or(false);
            if account_frozen {
                return Err(Error::AlreadyFrozen);
            }

            // update account's frozen status
            self.frozen_holders.insert(&account, &freeze);

            // emit freeze event
            self.env().emit_event(Freeze {
                asset_id: self.asset_id,
                account,
                freeze,
                freeze_id: self.freeze_id,
            });

            Ok(())
        }

        // Modify/Reconfigure an asset
        // Note: only the manager can modify an asset
        // Note: only mutable asset params can be modified
        // List of mutable asset params:
        // - managerId, reserveId, freezeId, clawbackId
        #[ink(message)]
        pub fn modify_asset(
            &mut self,
            manager: Option<AccountId>,
            reserve: Option<AccountId>,
            freeze: Option<AccountId>,
            clawback: Option<AccountId>,
        ) -> Result<(), Error> {
            let caller = self.env().caller();

            // check if caller is the manager
            if caller != self.manager_id {
                return Err(Error::NotManagerId);
            }

            // update asset params
            self.manager_id = manager.unwrap_or_else(|| AccountId::from([0x0; 32]));
            self.reserve_id = reserve.unwrap_or_else(|| AccountId::from([0x0; 32]));
            self.freeze_id = freeze.unwrap_or_else(|| AccountId::from([0x0; 32]));
            self.clawback_id = clawback.unwrap_or_else(|| AccountId::from([0x0; 32]));

            // emit modify asset event
            self.env().emit_event(Modify {
                manager_id: self.manager_id,
                reserve_id: self.reserve_id,
                freeze_id: self.freeze_id,
                clawback_id: self.clawback_id,
            });

            Ok(())
        }

        // Revoke an asset
        // Note: only the clawback address can revoke an asset
        // Note: must specify amount, revocation target id, and receiver
        #[ink(message)]
        pub fn revoke_asset(
            &mut self,
            receiver: AccountId,
            recovation_target: AccountId,
            amount: Balance,
        ) -> Result<(), Error> {
            let caller = self.env().caller();

            // check if caller is the clawback address
            if caller != self.clawback_id {
                return Err(Error::NotClawbackId);
            }

            // check if receiver has opted in
            let receiver_opted_in = self.accounts_opted_in.get(&receiver).unwrap_or(false);
            if !receiver_opted_in {
                return Err(Error::NotOptedIn);
            }

            // check if recovation target account has enough balance
            let receiver_balance = self.balances.get(&receiver).unwrap_or(0);
            if receiver_balance < amount {
                return Err(Error::NotEnoughBalance);
            }

            // update recovation target balance
            self.balances
                .insert(&recovation_target, &(receiver_balance - amount));

            // update receiver balance
            self.balances
                .insert(&receiver, &(receiver_balance + amount));

            // emit revoke asset event
            self.env().emit_event(Revoke {
                asset_id: self.asset_id,
                from: receiver,
                amount: Some(amount),
                clawback_id: self.clawback_id,
            });

            Ok(())
        }

        // Destroy an asset
        // Note: only the manager can destroy an asset
        // Note: all asset holdings are transferred to the manager
        #[ink(message)]
        pub fn destroy_asset(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();

            // check if caller is the manager
            if caller != self.manager_id {
                return Err(Error::NotManagerId);
            }

            // check if manager balance is equal to total supply
            let manager_balance = self.balances.get(&self.manager_id).unwrap_or(0);
            if manager_balance != self.total {
                return Err(Error::NotAllAssetsOwnedByManager);
            }

            // emit destroy asset event
            self.env().emit_event(Destruction {
                asset_id: self.asset_id,
                destroyer: self.manager_id,
            });

            // terminate contract
            self.env().terminate_contract(self.manager_id);
        }
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
