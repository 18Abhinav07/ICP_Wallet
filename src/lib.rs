// src/lib.rs

use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;
use ic_cdk::storage;

#[derive(CandidType, Deserialize, Default)]
struct TokenWallet {
    balances: HashMap<Principal, u64>,
    owner: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
struct TransferArgs {
    to: Principal,
    amount: u64,
}

thread_local! {
    static STATE: std::cell::RefCell<TokenWallet> = std::cell::RefCell::new(TokenWallet::default());
}

#[init]
fn init() {
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        let mut wallet = state.borrow_mut();
        wallet.owner = Some(caller);
    });
}

#[update]
fn transfer(args: TransferArgs) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    STATE.with(|state| {
        let mut wallet = state.borrow_mut();
        
        // Check if sender has sufficient balance
        let sender_balance = wallet.balances.get(&caller).unwrap_or(&0);
        if *sender_balance < args.amount {
            return Err("Insufficient balance".to_string());
        }

        // Update balances
        *wallet.balances.entry(caller).or_insert(0) -= args.amount;
        *wallet.balances.entry(args.to).or_insert(0) += args.amount;

        Ok(())
    })
}

#[query]
fn get_balance(principal: Principal) -> u64 {
    STATE.with(|state| {
        let wallet = state.borrow();
        *wallet.balances.get(&principal).unwrap_or(&0)
    })
}

#[update]
fn mint(to: Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    STATE.with(|state| {
        let mut wallet = state.borrow_mut();
        
        // Only owner can mint
        if wallet.owner != Some(caller) {
            return Err("Only owner can mint tokens".to_string());
        }

        *wallet.balances.entry(to).or_insert(0) += amount;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        // Initialize wallet
        init();

        // Mint some tokens to test account
        let test_account = Principal::from_text("2vxsx-fae").unwrap();
        mint(test_account, 1000).unwrap();

        // Test transfer
        let transfer_args = TransferArgs {
            to: Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
            amount: 500,
        };
        transfer(transfer_args).unwrap();

        // Verify balances
        assert_eq!(get_balance(test_account), 500);
    }
}