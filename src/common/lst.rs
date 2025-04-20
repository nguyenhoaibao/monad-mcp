use core::fmt;
use std::sync::Arc;

use anyhow::Context;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionReceipt, U256},
    utils::{format_units, hex::encode_prefixed, parse_units},
};
use rmcp::{
    Error as McpError, RoleServer, ServerHandler, model::*, schemars, service::RequestContext, tool,
};

use crate::{
    bindings::{aprmon, erc20, gmon, gmonstakemanager, shmon},
    services::constants::{
        APRMON_ADDRESS, GMON_ADDRESS, GMON_STAKEMANAGER_ADDRESS, MONAD_TESTNET_CHAIN_ID,
        SHMON_ADDRESS,
    },
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub enum LstProtocol {
    #[serde(rename = "aprMON")]
    AprMON,
    #[serde(rename = "gMON")]
    GMON,
    #[serde(rename = "shMON")]
    SHMON,
}

impl fmt::Display for LstProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LstProtocol::AprMON => write!(f, "aprMON"),
            LstProtocol::GMON => write!(f, "gMON"),
            LstProtocol::SHMON => write!(f, "shMON"),
        }
    }
}

impl TryFrom<&str> for LstProtocol {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, &'static str> {
        match value {
            "aprMON" => Ok(LstProtocol::AprMON),
            "gMON" => Ok(LstProtocol::GMON),
            "shMON" => Ok(LstProtocol::SHMON),
            _ => Err("Invalid LST protocol"),
        }
    }
}

impl LstProtocol {
    fn address(&self) -> Address {
        match self {
            LstProtocol::AprMON => *APRMON_ADDRESS,
            LstProtocol::GMON => *GMON_STAKEMANAGER_ADDRESS,
            LstProtocol::SHMON => *SHMON_ADDRESS,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            LstProtocol::AprMON => {
                "aPriori is the leading MEV-powered liquid staking platform on Monad."
            }
            LstProtocol::GMON => {
                "Magma enables Monad token holders to earn staking rewards while remaining liquid through Magma's Liquid Staking token, gMON."
            }
            LstProtocol::SHMON => {
                "shMONAD is an innovative Liquid Staking Token (LST) built on top of MON (Monad). Designed for users who wish to stake their MON while retaining liquidity, shMONAD allows holders to convert MON into shMON, bond their tokens within distinct policies, and later unbond them after an escrow period."
            }
        }
    }

    fn token_address(&self) -> Address {
        match self {
            LstProtocol::AprMON => *APRMON_ADDRESS,
            LstProtocol::GMON => *GMON_ADDRESS,
            LstProtocol::SHMON => *SHMON_ADDRESS,
        }
    }

    pub async fn read_balance(
        &self,
        provider: Arc<Provider<Http>>,
        owner: Address,
    ) -> anyhow::Result<U256> {
        Ok(match self {
            LstProtocol::AprMON => {
                let contract = aprmon::aprMON::new(self.token_address(), provider.clone());
                contract
                    .balance_of(owner)
                    .call()
                    .await
                    .context("Failed to get balance")?
            }
            LstProtocol::GMON => {
                let contract = gmon::g_mon::gMON::new(self.token_address(), provider.clone());
                contract
                    .balance_of(owner)
                    .call()
                    .await
                    .context("Failed to get balance")?
            }
            LstProtocol::SHMON => {
                let contract = erc20::erc20::new(self.token_address(), provider.clone());
                contract
                    .balance_of(owner)
                    .call()
                    .await
                    .context("Failed to get balance")?
            }
        })
    }

    pub async fn stake(
        &self,
        signer: Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
        signer_address: Address,
        amount: U256,
    ) -> anyhow::Result<Option<TransactionReceipt>> {
        let receipt = match self {
            LstProtocol::AprMON => {
                let contract = aprmon::aprMON::new(self.address(), signer.clone());
                contract
                    .deposit(amount, signer_address)
                    .value(amount)
                    .send()
                    .await
                    .context("Failed to deposit")?
                    .confirmations(1)
                    .await
                    .context("Failed to confirm deposit")?
            }
            LstProtocol::GMON => {
                let contract = gmonstakemanager::g_mon_stake_manager::gMONStakeManager::new(
                    self.address(),
                    signer.clone(),
                );
                contract
                    .deposit_mon()
                    .value(amount)
                    .send()
                    .await
                    .context("Failed to deposit")?
                    .confirmations(1)
                    .await
                    .context("Failed to confirm deposit")?
            }
            LstProtocol::SHMON => {
                let contract = shmon::shMON::new(self.address(), signer.clone());
                contract
                    .deposit(amount, signer_address)
                    .value(amount)
                    .send()
                    .await
                    .context("Failed to deposit")?
                    .confirmations(1)
                    .await
                    .context("Failed to confirm deposit")?
            }
        };

        Ok(receipt)
    }

    pub async fn unstake(
        &self,
        signer: Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
        signer_address: Address,
        amount: U256,
    ) -> anyhow::Result<Option<TransactionReceipt>> {
        let receipt = match self {
            LstProtocol::AprMON => {
                let contract = aprmon::aprMON::new(self.address(), signer.clone());
                contract
                    .request_redeem(amount, signer_address, signer_address)
                    .send()
                    .await
                    .context("Failed to request redeem")?
                    .confirmations(1)
                    .await
                    .context("Failed to confirm request redeem tx")?
            }
            LstProtocol::GMON => {
                let contract = gmonstakemanager::g_mon_stake_manager::gMONStakeManager::new(
                    self.address(),
                    signer.clone(),
                );
                contract
                    .withdraw_mon(amount)
                    .send()
                    .await
                    .context("Failed to deposit")?
                    .confirmations(1)
                    .await
                    .context("Failed to confirm deposit")?
            }
            LstProtocol::SHMON => {
                let contract = shmon::shMON::new(self.address(), signer.clone());
                contract
                    .redeem(amount, signer_address, signer_address)
                    .send()
                    .await
                    .context("Failed to request redeem")?
                    .confirmations(1)
                    .await
                    .context("Failed to confirm request redeem tx")?
            }
        };

        Ok(receipt)
    }

    pub async fn tvl(&self, provider: Arc<Provider<Http>>) -> anyhow::Result<U256> {
        let tvl = match self {
            LstProtocol::AprMON => {
                let contract = aprmon::aprMON::new(self.token_address(), provider.clone());
                contract
                    .total_assets()
                    .call()
                    .await
                    .context("Failed to get total assets")?
            }
            LstProtocol::GMON => {
                let contract = gmonstakemanager::g_mon_stake_manager::gMONStakeManager::new(
                    self.address(),
                    provider.clone(),
                );
                contract
                    .calculate_tvl()
                    .call()
                    .await
                    .context("Failed to get total supply")?
            }
            LstProtocol::SHMON => {
                let contract = shmon::shMON::new(self.token_address(), provider.clone());
                contract
                    .total_assets()
                    .call()
                    .await
                    .context("Failed to get total supply")?
            }
        };

        Ok(tvl)
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct StakeRequest {
    pub protocol: LstProtocol,
    pub private_key: String,
    pub amount: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UnstakeRequest {
    pub protocol: LstProtocol,
    pub private_key: String,
    pub amount: String,
}

#[derive(Clone)]
pub struct Lst {
    provider: Arc<Provider<Http>>,
}

#[tool(tool_box)]
impl Lst {
    #[allow(dead_code)]
    pub fn new(provider: Arc<Provider<Http>>) -> Self {
        Lst { provider }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    async fn read_balance(&self, protocol: LstProtocol, owner: Address) -> anyhow::Result<U256> {
        protocol
            .read_balance(self.provider.clone(), owner)
            .await
            .context("Failed to read balance")
    }

    async fn protocol_tvl(&self, protocol: LstProtocol) -> anyhow::Result<U256> {
        protocol
            .tvl(self.provider.clone())
            .await
            .context("Failed to get TVL")
    }

    #[tool(description = "Stake LST tokens")]
    async fn stake(
        &self,
        #[tool(aggr)] StakeRequest {
            protocol,
            private_key,
            amount,
        }: StakeRequest,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Staking {} LST tokens using protocol {}", amount, protocol);

        let signer = private_key
            .parse::<LocalWallet>()
            .map_err(|e| {
                ErrorData::invalid_params(format!("Failed to parse private key: {}", e), None)
            })?
            .with_chain_id(MONAD_TESTNET_CHAIN_ID);
        let signer_address = signer.address();
        let signer = Arc::new(SignerMiddleware::new(self.provider.clone(), signer));

        let parsed_amount = parse_units(&amount, "ether").map_err(|e| {
            ErrorData::invalid_params(format!("Failed to parse amount '{}': {}", amount, e), None)
        })?;
        let amount_u256: U256 = parsed_amount.into();

        let receipt = protocol
            .stake(signer.clone(), signer_address, amount_u256)
            .await
            .map_err(|e| ErrorData::internal_error(format!("Staking failed: {}", e), None))?
            .ok_or_else(|| {
                ErrorData::internal_error("Staking failed: no receipt returned".to_string(), None)
            })?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Staked {} {} tokens successfully. Transaction hash: {}",
            amount,
            protocol,
            encode_prefixed(receipt.transaction_hash)
        ))]))
    }

    #[tool(description = "Unstake LST tokens")]
    async fn unstake(
        &self,
        #[tool(aggr)] StakeRequest {
            protocol,
            private_key,
            amount,
        }: StakeRequest,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Unstaking {} LST tokens using protocol {}",
            amount,
            protocol
        );

        let signer = private_key
            .parse::<LocalWallet>()
            .map_err(|e| {
                ErrorData::invalid_params(format!("Failed to parse private key: {}", e), None)
            })?
            .with_chain_id(MONAD_TESTNET_CHAIN_ID);
        let signer_address = signer.address();
        let signer = Arc::new(SignerMiddleware::new(self.provider.clone(), signer));

        let parsed_amount = parse_units(&amount, "ether").map_err(|e| {
            ErrorData::invalid_params(format!("Failed to parse amount '{}': {}", amount, e), None)
        })?;
        let amount_u256: U256 = parsed_amount.into();

        let receipt = protocol
            .unstake(signer.clone(), signer_address, amount_u256)
            .await
            .map_err(|e| ErrorData::internal_error(format!("Staking failed: {}", e), None))?
            .ok_or_else(|| {
                ErrorData::internal_error("Staking failed: no receipt returned".to_string(), None)
            })?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Unstaked {} {} tokens successfully. Transaction hash: {}",
            amount,
            protocol,
            encode_prefixed(receipt.transaction_hash)
        ))]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Lst {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("This server provides a LST (Liquid Staking Token) tool that can be used by staking native token and receive a LST token.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().enable_resources().build(),
            ..Default::default()
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![self._create_resource_text("evm://networks", "Get supported networks")],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        // Exact match for networks resource
        if uri == "evm://networks" {
            return Ok(ReadResourceResult {
                contents: vec![ResourceContents::text(
                    "Supported networks: monadTestnet",
                    uri,
                )],
            });
        }

        // Parse the URI into parts
        let parts: Vec<&str> = uri.split('/').collect();

        // Check if the URI starts with evm://
        if parts.len() >= 2 && parts[0] == "evm:" {
            let network = parts[2];

            // Validate network
            if network != "monadTestnet" {
                return Err(McpError::resource_not_found(
                    "resource_not_found",
                    Some(serde_json::json!({
                        "uri": uri,
                        "error": "Unsupported network",
                    })),
                ));
            }

            // Pattern: evm://{network}/lsts
            if parts.len() == 4 && parts[3] == "lsts" {
                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        "Available LST protocols: aprMON, gMON, shMON",
                        uri,
                    )],
                });
            }

            // Pattern: evm://{network}/lsts/{lst}
            if parts.len() == 5 && parts[3] == "lsts" {
                let lst_name = parts[4];
                let protocol: LstProtocol = lst_name.try_into().map_err(|e| {
                    ErrorData::invalid_params(
                        format!("Failed to parse protocol '{}': {}", lst_name, e),
                        None,
                    )
                })?;

                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(protocol.description(), uri)],
                });
            }

            // Pattern: evm://{network}/lsts/{lst}/tvl
            if parts.len() == 6 && parts[3] == "lsts" && parts[5] == "tvl" {
                let lst_name = parts[4];
                let protocol: LstProtocol = lst_name.try_into().map_err(|e| {
                    ErrorData::invalid_params(
                        format!("Failed to parse protocol '{}': {}", lst_name, e),
                        None,
                    )
                })?;

                let tvl = self.protocol_tvl(protocol).await.map_err(|e| {
                    ErrorData::internal_error(format!("Failed to get TVL: {}", e), None)
                })?;

                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!("TVL: {} ether", format_units(tvl, "ether").unwrap()),
                        uri,
                    )],
                });
            }

            // Pattern: evm://{network}/address/{address}/lsts/{lst}/balance
            if parts.len() == 8
                && parts[3] == "address"
                && parts[5] == "lsts"
                && parts[7] == "balance"
            {
                let address_str = parts[4];
                let lst_name = parts[6];

                // Parse the address
                let address = match address_str.parse::<Address>() {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(McpError::invalid_params(
                            "invalid_address",
                            Some(serde_json::json!({
                                "uri": uri,
                                "error": format!("Invalid address: {}", e),
                            })),
                        ));
                    }
                };

                let protocol: LstProtocol = lst_name.try_into().map_err(|e| {
                    ErrorData::invalid_params(format!("Failed to parse protocol: {}", e), None)
                })?;

                // Get balance
                let balance = self.read_balance(protocol, address).await.map_err(|e| {
                    ErrorData::internal_error(format!("Failed to get balance: {}", e), None)
                })?;

                let formatted_balance = format_units(balance, "ether").map_err(|e| {
                    ErrorData::internal_error(format!("Failed to format balance: {}", e), None)
                })?;

                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!("Balance: {} {}", formatted_balance, lst_name),
                        uri,
                    )],
                });
            }
        }

        // No match found
        tracing::warn!("No match found for URI: {}", uri);

        Err(McpError::resource_not_found(
            "resource_not_found",
            Some(serde_json::json!({
                "uri": uri
            })),
        ))
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: vec![
                ResourceTemplate {
                    raw: RawResourceTemplate {
                        uri_template: "evm://{network}/lsts".to_string(),
                        name: "List of available LST protocols".to_string(),
                        description: None,
                        mime_type: Some("text".to_string()),
                    },
                    annotations: None,
                },
                ResourceTemplate {
                    raw: RawResourceTemplate {
                        uri_template: "evm://{network}/lsts/{lst}".to_string(),
                        name: "Details of a specific LST protocol".to_string(),
                        description: None,
                        mime_type: Some("text".to_string()),
                    },
                    annotations: None,
                },
                ResourceTemplate {
                    raw: RawResourceTemplate {
                        uri_template: "evm://{network}/address/{address}/lsts/{lst}/balance"
                            .to_string(),
                        name: "Get balance of LST token for a given address".to_string(),
                        description: None,
                        mime_type: Some("text".to_string()),
                    },
                    annotations: None,
                },
            ],
        })
    }
}
