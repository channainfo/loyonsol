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

### Airdrop

#### Using our local testnet

```sh
# airdrop 2 sols
solana airdrop 2

# verify your balance
solana balance
```

#### Using Public cluster

Here is the devnet endpoint: <https://api.devnet.solana.com>. We can just switch the Solana CLI to connect to the devnet

```sh
solana config set --url https://api.devnet.solana.com

# verify it
solana config get
```

Now you can perform the solana command as usual, for example, to airdrop 5 sol to your own account

```sh
solana airdrop 5

```

Public clusters:

- Test: <https://api.testnet.solana.com>
- Dev: <https://api.devnet.solana.com>

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

Go to <https://solscan.io/> and search for your program by the program id above

## Anchor Framework

Our project is based on the Anchor framework located under the anchor directory.

```sh
# init the anchor project
anchor init loyonsol

# rename the folder to anchor and cd to the it
cd anchor

# build the app
anchor build

# run test
anchor test

# show the list
anchor keys list
```
