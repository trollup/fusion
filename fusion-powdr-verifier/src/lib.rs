#![no_std]

use powdr_riscv_runtime::input::get_data_serde;

use ruint::aliases::U256;

extern crate alloc;

use fusion_api::Tx;
use fusion_state::{apply_tx, State};

#[no_mangle]
fn main() {
    let pre_state: State = get_data_serde(42);
    let tx: Tx = get_data_serde(43);
    let post_root: U256 = get_data_serde(44);

    let post_state = apply_tx(pre_state, &tx);
    assert_eq!(post_root, post_state.root());
}
