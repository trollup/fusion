#![cfg_attr(not(feature = "std"), no_std)]

use fusion_types::*;
use ruint::aliases::U256;
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::string::String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub kind: TxKind,
    pub sender: PublicKey,
    pub to: PublicKey,
    pub nonce: U256,
    pub value: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TxKind {
    Transfer,
    Deposit,
    Withdraw,
}

const ONE: U256 = U256::from_limbs([1, 0, 0, 0]);
const TWO: U256 = U256::from_limbs([2, 0, 0, 0]);

impl TxKind {
    pub fn to_u256(&self) -> U256 {
        match self {
            TxKind::Transfer => U256::ZERO,
            TxKind::Deposit => ONE,
            TxKind::Withdraw => TWO,
        }
    }
}

impl From<u8> for TxKind {
    fn from(k: u8) -> Self {
        match k {
            0 => TxKind::Transfer,
            1 => TxKind::Deposit,
            2 => TxKind::Withdraw,
            _ => panic!(),
        }
    }
}

impl From<U256> for TxKind {
    fn from(k: U256) -> Self {
        match k {
            U256::ZERO => TxKind::Transfer,
            ONE => TxKind::Deposit,
            TWO => TxKind::Withdraw,
            _ => panic!(),
        }
    }
}

pub fn hash_tx(tx: &Tx) -> U256 {
    let sender_pk: PublicKey = tx.sender.clone();
    let sender_addr = sender_pk.address();

    let to_pk: PublicKey = tx.to.clone();
    let to_addr = to_pk.address();

    fusion_poseidon::poseidon_sponge(&[tx.kind.to_u256(), sender_addr, to_addr, tx.nonce, tx.value])
}

//#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: String,
}

/*
#[tarpc::service]
pub trait FusionRPC {
    async fn submit_transaction(tx: SignedTx) -> Result<(), String>;
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash() {
        let tx = Tx {
            sender: PublicKey::from(
                "11693830015789570214896451416834991706586932551962432904221523856506008194081",
            ),
            to: PublicKey::from(
                "11693830015789570214896451416834991706586932551962432904221523856506008194081",
            ),
            nonce: U256::ZERO,
            value: U256::ZERO,
            kind: TxKind::Transfer,
        };
        assert_eq!(hash_tx(&tx), U256::from_str_radix("0", 10).unwrap());
    }
}
