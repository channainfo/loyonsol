# Getting started

## Setup local environment

### Install Solana

Download a prebuilt binary from here: <https://github.com/solana-labs/solana/releases> and point your PATH to the downloaded folder

```sh
export PATH="$HOME/Downloads/solana-release/bin:$PATH"

# make sure it points to the correct location
which solana

# make sure the version is correct
solana --version
```

I am having homebrew intel I download the x86_84 to use cargo build-bpf, however run validator I have to use aarm64 installation instead.

### Install Anchor Version Manager

The instruction is listed here: <https://solana.com/developers/guides/getstarted/setup-local-development#4-install-anchor-for-solana>

```sh
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# verify the anchor version
anchor --version
```

### Setup local test validator

Open a terminal tab and run the following

```sh
solana-test-validator
```

Now config Solana to use local validator

```sh
solana config set --url localhost

#verify config
solana config get
```

### Create wallet

Generate keygen default to ~/.config/solana/id.json

```sh
solana-keygen new --force > wallet-test.txt
```

Tell solana to use the wallet

```sh
solana config set -k ~/.config/solana/id.json
```

Topup some sol to your wallet

```sh
# airdrop 2 sols
solana airdrop 2

# verify your balance
solana balance
```

### Build the project

Install rust and cargo <https://solana.com/developers/guides/getstarted/local-rust-hello-world#install-rust-and-cargo>

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Build you solana program

```sh
cargo build-bpf
```

You program will be built to **target/deploy/loy.so**

Now you can deploy your program to the chain

```sh

solana program deploy ./target/deploy/loy.so
# > Program Id: CPdff6rNwxkgEQDGQoqztqzrKwGTJKkvmMHjha9yRvmw

```
