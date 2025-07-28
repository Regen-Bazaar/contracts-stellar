# NFT Factory Contract

`stellar contract build` to build smart contracts

stellar contract deploy --wasm target/wasm32v1-none/release/NFT.wasm --source-account alice --network testnet -- --admin GADGVW7RXKGSXKWRQF2T6VFTQ4K2S2JOYUSZ7V2KVZ6RGLK32GRZXLRA --base_token_uri "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"

Deployed at CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW

stellar contract invoke --id CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW --source alice --network testnet -- token_count
returns "0"

stellar contract invoke --id CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW --source alice --network testnet -- base_uri
returns "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"

stellar contract deploy --wasm target/wasm32v1-none/release/NFTFactory.wasm --source-account alice --network testnet -- --admin GADGVW7RXKGSXKWRQF2T6VFTQ4K2S2JOYUSZ7V2KVZ6RGLK32GRZXLRA --nft_contract CCMKFWAW46KIJF3ZKZAMDUJXEKZUTOWCDZZZ3MLDZ7HOWUISLGTQHJCW

Deployed at CCBNGWVUOU7WOJELOVCCRQRYVJ4CKSNBRWCLEFIPDQPA6675NE6XA3FC
