# ICP Token Wallet

A secure token wallet implementation for the Internet Computer Protocol (ICP) blockchain, written in Rust. This wallet supports IRCRC2 token operations including sending, receiving, and balance checking.

## Features

- Send tokens to other addresses
- Receive tokens from other addresses
- Check wallet balance
- Mint tokens (owner only)
- Secure transaction handling
- Comprehensive test suite

## Prerequisites

- Rust (latest stable version)
- DFX (ICP SDK)
- Node.js and npm (for local development)

## Setup Instructions

1. Install the Internet Computer SDK (DFX):

```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

2. Clone the repository:

```bash
git clone https://github.com/yourusername/icp-token-wallet
cd icp-token-wallet
```

3. Start the local ICP network:

```bash
dfx start --background
```

4. Deploy the canister:

```bash
dfx deploy
```

## Testing

Run the test suite:

```bash
cargo test
```

## Smart Contract Functions

### transfer

Transfers tokens from the caller's address to another address.

Parameters:

- `to`: Principal ID of the recipient
- `amount`: Amount of tokens to transfer

### get_balance

Queries the token balance for a given address.

Parameters:

- `principal`: Principal ID to check balance for

Returns:

- `u64`: Current balance

### mint

Mints new tokens (only callable by owner).

Parameters:

- `to`: Principal ID of the recipient
- `amount`: Amount of tokens to mint

## Security Features

1. Balance validation before transfers
2. Owner-only minting
3. Secure state management
4. Principal-based authentication

## Best Practices

1. Always verify transaction success
2. Keep private keys secure
3. Test transactions with small amounts first
4. Monitor balance changes after transactions

## Error Handling

The wallet implements comprehensive error handling:

- Insufficient balance checks
- Invalid transfer amount validation
- Authorization checks for privileged operations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License
