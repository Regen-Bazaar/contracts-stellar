# Soroban Project

`stellar contract build`

stellar contract deploy --wasm target/wasm32v1-none/release/NFT.wasm --source-account alice --network testnet -- --admin GADGVW7RXKGSXKWRQF2T6VFTQ4K2S2JOYUSZ7V2KVZ6RGLK32GRZXLRA --base_token_uri "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"

Deployed at CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW

stellar contract invoke --id CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW --source alice --network testnet -- token_count
returns "0"

stellar contract invoke --id CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW --source alice --network testnet -- base_uri
returns "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"

stellar contract deploy --wasm target/wasm32v1-none/release/NFTFactory.wasm --source-account alice --network testnet -- --admin GADGVW7RXKGSXKWRQF2T6VFTQ4K2S2JOYUSZ7V2KVZ6RGLK32GRZXLRA --nft_contract CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW

Deployed at CCBNGWVUOU7WOJELOVCCRQRYVJ4CKSNBRWCLEFIPDQPA6675NE6XA3FC

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── src
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
