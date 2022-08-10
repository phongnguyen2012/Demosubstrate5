#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod kitty {

    use ink_storage::{traits::SpreadAllocate, Mapping,};
    use scale::{Decode, Encode,};
    pub type KittyId = u32;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Kitty {
        /// Stores a single `bool` value on the storage.
        //value: bool,
       kitty_owner: Mapping<KittyId, AccountId>,
       owned_kitty_count: Mapping<AccountId, u32>,
       kitty_approvals: Mapping<KittyId, AccountId>,
       operator_approvals: Mapping<(AccountId, AccountId), ()>,
    }
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// The account already owns this token.
        AlreadyOwned,
        /// The account does not own this token.
        NotOwned,
        /// The token does not exist.
        TokenDoesNotExist,
        /// The account does not have enough balance to transfer.
        InsufficientBalance,
        /// The account does not have enough balance to transfer.
        InsufficientAllowance,
    }
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        kitty_id: KittyId,
    }
    impl Kitty {
        /// Create a new `Kitty` token contract
        #[ink(constructor)]
        pub fn new() -> Self {
           ink_lang::utils::initialize_contract(|_| {})
        }

        fn balacne_of_or_zero(&self, of: &AccountId) -> u32 {
            self.owned_kitty_count.get(of).unwrap_or(0)
        }
        fn transfer_kitty_from(&mut self, from: &AccountId, to: &AccountId, kitty_id: KittyId) -> Result<(), Error> {
            let caller = self.env().caller();
            if !self.exists(kitty_id) {
                return Err(Error::TokenDoesNotExist)
            };
            if !self.approve_or_owner(Some(caller), kitty_id){
                return Err(Error::NotOwned)
            };
            self.clear_approval(kitty_id);
            self.remove_kitty_from(from, kitty_id)?;
            self.add_kitty_to(to, kitty_id)?;
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                kitty_id,
            });
            Ok(())
        }
        fn add_kitty_to(&mut self, to: &AccountId, kitty_id: KittyId) -> Result<(), Error> {
           let Self {
                kitty_owner,
                owned_kitty_count,
                ..
              } = self;
           
           if kitty_owner.contains(&kitty_id) {
               return Err(Error::AlreadyOwned);
           };
           if *to == AccountId::from([0x0; 32]) {
               return Err(Error::NotOwned);
           };
           let count = owned_kitty_count.get(to).map(|c| c +1).unwrap_or(1);

           owned_kitty_count.insert(to, &count);
           kitty_owner.insert(&kitty_id, to);
            Ok(())
        }
        fn exists(&self, kitty_id: KittyId) -> bool {
            self.kitty_owner.contains(&kitty_id)
        }
        fn approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.operator_approvals.contains((&owner, &operator))
        }
        fn approve_or_owner(&mut self, from: Option<AccountId>, kitty_id: KittyId) -> bool {
            let owner = self.owner_of(kitty_id);
            from != Some(AccountId::from([0x0; 32])) && (from == owner
                || from == self.kitty_approvals.get(&kitty_id)
                || self.approved_for_all(owner.expect("Error with AccountId"), from.expect("Error with AccountId"),
            ))
        }
        fn remove_kitty_from(&mut self, from: &AccountId, kitty_id: KittyId) -> Result<(), Error> {
            let Self {
                kitty_owner,
                owned_kitty_count,
                ..
            } = self;   
            
            if !kitty_owner.contains(&kitty_id) {
                return Err(Error::NotOwned);
            };
            let count = owned_kitty_count.get(&from).map(|c| c -1).ok_or(Error::NotOwned)?;
            owned_kitty_count.insert(&from, &count);
            kitty_owner.remove(&kitty_id);
    
            Ok(())
        }
        fn clear_approval(&mut self, kitty_id: KittyId) {
            self.kitty_approvals.remove(&kitty_id);
        }
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balacne_of_or_zero(&owner)
        }
        
        #[ink(message)]
        pub fn owner_of(&self, kitty_id: KittyId) -> Option<AccountId> {
            self.kitty_owner.get(&kitty_id)
        }

        #[ink(message)]
        pub fn transfer(&mut self, destination: AccountId, kitty_id: KittyId) -> Result<(), Error> {
            let caller = self.env().caller();
            self.transfer_kitty_from(&caller, &destination, kitty_id)?;
            Ok(())
        }
        #[ink(message)]
        pub fn mint(&mut self, kitty_id: KittyId) -> Result<(), Error> {
            let caller = self.env().caller();
            self.add_kitty_to(&caller, kitty_id)?;
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                kitty_id,
            });
            Ok(())
        }
      
    }

    // Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // module and test functions are marked with a `#[test]` attribute.
    // The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    // }
}
