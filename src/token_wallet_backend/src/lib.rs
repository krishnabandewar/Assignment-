use ic_cdk_macros::{update, query};
use ic_cdk::export::candid::{CandidType, Principal as CandidPrincipal}; // Import CandidPrincipal from ic_cdk
use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

thread_local! {
    static WALLET: RefCell<Wallet> = RefCell::new(Wallet::default());
}

#[derive(Default, CandidType, Deserialize, Serialize)]
struct Wallet {
    balances: HashMap<String, u64>,
}

#[update]
fn send_token(to: String, amount: u64, caller: Option<CandidPrincipal>) -> Result<String, String> {
    let caller = caller.unwrap_or_else(|| ic_cdk::caller());
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        let sender = caller.to_text();
        
        let sender_balance = wallet.balances.entry(sender.clone()).or_insert(0);
        
        if *sender_balance < amount {
            return Err(format!("Insufficient funds. Current balance: {}", *sender_balance));
        }

        *sender_balance -= amount;
        let receiver_balance = wallet.balances.entry(to.clone()).or_insert(0);
        *receiver_balance += amount;

        Ok(format!("{} tokens sent to {}", amount, to))
    })
}

#[query]
fn check_balance(account: String) -> u64 {
    WALLET.with(|wallet| {
        let wallet = wallet.borrow();
        *wallet.balances.get(&account).unwrap_or(&0)
    })
}

#[update]
fn receive_token(from: String, amount: u64, caller: Option<CandidPrincipal>) -> Result<String, String> {
    let caller = caller.unwrap_or_else(|| ic_cdk::caller());
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        let receiver = caller.to_text();
        
        let receiver_balance = wallet.balances.entry(receiver.clone()).or_insert(0);
        *receiver_balance += amount;

        Ok(format!("{} tokens received from {}", amount, from))
    })
}

#[update]
fn init_balance(amount: u64) -> Result<String, String> {
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        let caller = ic_cdk::caller().to_text();
        
        wallet.balances.insert(caller.clone(), amount);
        Ok(format!("Initialized balance of {} tokens for {}", amount, caller))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::candid::Principal as CandidPrincipal; // Import CandidPrincipal from ic_cdk

    #[test]
    fn test_send_and_receive() {
        // Initialize user1's balance
        let user1 = CandidPrincipal::from_slice(&[0u8; 29]); // Example of a valid Principal (29 bytes)
        let user2 = CandidPrincipal::from_slice(&[1u8; 29]); // Another valid Principal for user2

        // Set initial balance for user1
        WALLET.with(|wallet| {
            let mut wallet = wallet.borrow_mut();
            wallet.balances.insert(user1.to_text(), 100); // Use user1's Principal text
        });

        // Send tokens from user1 to user2
        let response = send_token(user2.to_text(), 50, Some(user1));

        // Use user2's text representation in the assertion
        assert_eq!(response, Ok(format!("50 tokens sent to {}", user2.to_text()))); // Corrected assertion

        // Check balances
        assert_eq!(check_balance(user1.to_text()), 50); // user1 should have 50 left
        assert_eq!(check_balance(user2.to_text()), 50); // user2 should have received 50
    }
}