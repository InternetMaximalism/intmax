# Eth Provider

## bin

### generate

Automatically generate Rust typed contracts from hardhat artifacts.

```
infra
└ eth-provider
　 └ artifacts
　 　 ├ mainnet
　 　 │ ├ .chainId
　 　 │ ├ XXX.json
　 　 │ └ YYY.json
　 　 ├ rinkeby
　 　 │ ├ .chainId
　 　 │ ├ XXX.json
　 　 │ └ YYY.json
　 　 └ ropsten
　 　 　 ├ .chainId
　 　 　 ├ XXX.json
　 　 　 └ YYY.json
```

How to use
1. Prepare artifacts
2. run generate command
```bash
cargo run --bin generate rinkeby
cargo run --bin generate mainnet
```
3. write `pub mod xxx.rs` in contracts.rs

Output
```
infra
└ eth-provider
　 ├ contracts
　 │ ├ xxx.rs
　 │ └ yyy.rs
　 └ contracts.rs <= Please add `pub mod xxx.rs`
```
