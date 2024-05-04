use tokio::sync::mpsc;

use fusion_api::*;
use fusion_config::Config;
use fusion_types::*;

use fusion_sequencer::sequencer::*;
use fusion_sequencer::server::*;

use fusion_state::{apply_tx, State};

use ruint::aliases::U256;

fn state_update_test() {
    let pre_state = State::default();

    let tx = Tx {
        kind: TxKind::Transfer,
        sender: PublicKey::from("0"),
        to: PublicKey::from("0"),
        nonce: U256::ZERO,
        value: U256::from_limbs([1, 0, 0, 0]),
    };

    let post_state = apply_tx(pre_state.clone(), &tx);

    let _ = fusion_prover::prove(&tx, &pre_state, &post_state);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("../fusion.toml".to_string());
    let (sx, rx): (mpsc::Sender<SignedTx>, mpsc::Receiver<SignedTx>) = mpsc::channel(1024);

    let socket_address = config.socket_address.to_string();
    tokio::spawn(async move {
        run_server(sx, socket_address, config.socket_port)
            .await
            .unwrap();
    });

    run_sequencer(&config, rx).await
}
