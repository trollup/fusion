use fusion_api::SignedTx;

#[tarpc::service]
pub trait FusionRPC {
    async fn submit_transaction(tx: SignedTx) -> Result<(), String>;
}
