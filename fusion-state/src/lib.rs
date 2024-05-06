#![no_std]

pub mod merkle_tree;
pub mod state;

pub use crate::state::{Account, State};

use fusion_api::*;
use fusion_types::PublicKey;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use bitmaps::Bitmap;

pub fn apply_tx(mut state: State, tx: &Tx) -> State {
    let sender_pk: PublicKey = tx.sender.clone();
    let sender_addr = sender_pk.address();

    let to_pk: PublicKey = tx.to.clone();
    let to_addr = to_pk.address();

    let account_sender = state.get(&sender_addr);
    let account_to = state.get(&to_addr);

    let new_account_sender = match tx.kind {
        TxKind::Deposit => Account::new(sender_addr, account_sender.balance + tx.value, tx.nonce),
        TxKind::Transfer | TxKind::Withdraw => {
            Account::new(sender_addr, account_sender.balance - tx.value, tx.nonce)
        }
    };
    let new_account_to = match tx.kind {
        TxKind::Transfer => Account::new(to_addr, account_to.balance + tx.value, account_to.nonce),
        TxKind::Withdraw | TxKind::Deposit => account_to,
    };

    state.update(&sender_addr, new_account_sender);
    state.update(&to_addr, new_account_to);
    state
}

trait ToVecBool {
    fn to_vec_bool(&self) -> Vec<bool>;
}

impl ToVecBool for Bitmap<256> {
    fn to_vec_bool(&self) -> Vec<bool> {
        let mut v: Vec<bool> = vec![];
        (0..256).for_each(|b| {
            v.push(self.get(b));
        });
        v
    }
}
