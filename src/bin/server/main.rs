use std::sync::Arc;

use ethers::providers::{Http, Provider};
use rmcp::transport::sse_server::SseServer;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

use monad_mcp::common::lst::Lst;

const BIND_ADDRESS: &str = "0.0.0.0:8989";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let provider = Provider::<Http>::try_from("https://testnet-rpc.monad.xyz")
        .expect("Failed to create provider");
    let provider = Arc::new(provider);

    let ct = SseServer::serve(BIND_ADDRESS.parse()?)
        .await?
        .with_service({
            let lst_service = Lst::new(provider);
            move || lst_service.clone()
        });

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
