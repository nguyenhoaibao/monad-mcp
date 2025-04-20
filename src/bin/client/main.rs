use anyhow::Result;
use ethers::{
    signers::{LocalWallet, Signer},
    utils::to_checksum,
};
use rmcp::{
    Peer, RoleClient, ServiceExt,
    model::{
        CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation,
        ReadResourceRequestParam,
    },
    transport::SseTransport,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let transport = SseTransport::start("http://127.0.0.1:8989/sse").await?;
    let client_info = ClientInfo {
        protocol_version: Default::default(),
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "test sse client".to_string(),
            version: "0.0.1".to_string(),
        },
    };
    let client = client_info.serve(transport).await.inspect_err(|e| {
        tracing::error!("client error: {:?}", e);
    })?;

    // Initialize
    let server_info = client.peer_info();
    tracing::info!("Connected to server: {server_info:#?}");

    // List tools
    let tools = client.list_tools(Default::default()).await?;
    tracing::info!("Available tools: {tools:#?}");

    // List resources
    let resources = client.list_resources(Default::default()).await?;
    tracing::info!("Available resources: {resources:#?}");

    // List resources template
    let resource_templates = client.list_resource_templates(Default::default()).await?;
    tracing::info!("Available resource templates: {resource_templates:#?}");

    let supported_networks = client
        .read_resource(ReadResourceRequestParam {
            uri: "evm://networks".to_string(),
        })
        .await?;
    println!("Supported networks: {supported_networks:#?}");

    let lst_protocols = client
        .read_resource(ReadResourceRequestParam {
            uri: "evm://monadTestnet/lsts".to_string(),
        })
        .await?;
    println!("LST protocols: {lst_protocols:#?}");

    let protocol_name = "shMON";
    let private_key = std::env::var("PRIVATE_KEY").unwrap_or_default();

    test(client.clone(), protocol_name, private_key.clone()).await?;

    client.cancel().await?;
    Ok(())
}

async fn test(client: Peer<RoleClient>, protocol_name: &str, private_key: String) -> Result<()> {
    println!("Testing {protocol_name}...");

    let lst_protocol = client
        .read_resource(ReadResourceRequestParam {
            uri: format!("evm://monadTestnet/lsts/{}", protocol_name),
        })
        .await?;
    println!("LST protocol: {lst_protocol:#?}");

    if private_key.is_empty() {
        tracing::warn!("No private key provided. Skipping stake/unstake.");
    } else {
        println!("Staking on {protocol_name}...");

        let tool_result = client
            .call_tool(CallToolRequestParam {
                name: "stake".into(),
                arguments: serde_json::json!({
                    "protocol": protocol_name,
                    "private_key": private_key,
                    "amount": "0.005",
                })
                .as_object()
                .cloned(),
            })
            .await?;
        tracing::info!("Tool result: {tool_result:#?}");

        // Wait for the transaction to be mined
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let wallet = private_key.parse::<LocalWallet>().unwrap();

        let balance = client
            .read_resource(ReadResourceRequestParam {
                uri: format!(
                    "evm://monadTestnet/address/{}/lsts/{}/balance",
                    to_checksum(&wallet.address(), None),
                    protocol_name,
                ),
            })
            .await?;
        println!("Balance: {balance:#?}");
    }

    let tvl = client
        .read_resource(ReadResourceRequestParam {
            uri: format!("evm://monadTestnet/lsts/{}/tvl", protocol_name),
        })
        .await?;
    println!("TVL: {tvl:#?}");

    Ok(())
}
