# Monad MCP - Liquid Staking Service

A Model Context Protocol (MCP) service implementation for interacting with liquid staking protocols on the Monad blockchain testnet.

## Overview

This project provides a client-server implementation using the Model Context Protocol (MCP) to interact with Liquid Staking Token (LST) protocols on the Monad testnet. It allows users to:

- View information about available LST protocols
- Stake native MON tokens to receive LST tokens
- Unstake LST tokens to receive native MON tokens
- Check balances and TVL (Total Value Locked)

Currently, the service supports the following LST protocols:
- **aprMON** - aPriori's MEV-powered liquid staking token
- **gMON** - Magma's liquid staking token
- **shMON** - FastLane's liquid staking token

## Prerequisites

- Rust 1.70+ and Cargo
- Access to Monad testnet
- Private key for signing transactions

## Installation

1. Clone the repository:

```bash
git clone https://github.com/your-username/monad-mcp.git
cd monad-mcp
```

2. Build the project:

```bash
cargo build --release
```

## Usage

### Running the Server

Start the MCP server that provides the LST staking services:

```bash
cargo run --bin server
```

By default, the server binds to `127.0.0.1:8989`.

### Running the Client

The client can connect to the server and perform various operations:

```bash
# Set your private key for transactions (optional)
export PRIVATE_KEY=0x...

# Run the client
cargo run --bin client
```

### Available Resources

The MCP service provides access to the following resource endpoints:

- `evm://networks` - List supported networks
- `evm://{network}/lsts` - List available LST protocols
- `evm://{network}/lsts/{lst}` - Get information about a specific LST protocol
- `evm://{network}/lsts/{lst}/tvl` - Get the Total Value Locked for a specific LST protocol
- `evm://{network}/address/{address}/lsts/{lst}/balance` - Get the LST token balance for a specific address

### Available Tools

The MCP service provides the following tools:

- `stake` - Stake native MON tokens to receive LST tokens
- `unstake` - Unstake LST tokens to receive native MON tokens

## Architecture

This project follows a client-server architecture using the Model Context Protocol:

1. **Server**: Implements the MCP server that provides LST services and resources
2. **Client**: Connects to the server and interacts with the provided services
3. **Common**: Contains shared code and implementations for LST protocols

### Key Components

- **LST Service**: Provides staking and unstaking functionality for different LST protocols
- **Resource Endpoints**: Expose information about supported networks, protocols, and balances
- **Tools**: Expose functionality to interact with the blockchain (stake, unstake)

## Development

### Project Structure

- `src/bin/client/` - Client implementation
- `src/bin/server/` - Server implementation
- `src/common/lst.rs` - LST protocol implementations
- `src/bindings/` - Contract bindings for interacting with smart contracts

### Adding New LST Protocols

To add a new LST protocol:

1. Add the contract binding in `src/bindings/`
2. Add a new variant to the `LstProtocol` enum in `src/common/lst.rs`
3. Implement the required methods for the new protocol

## License

MIT.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

- [Monad](https://monad.xyz/) - For providing the testnet infrastructure
- [Rust MCP](https://github.com/modelcontextprotocol/rust-sdk) - For the Rust MCP implementation
