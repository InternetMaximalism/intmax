# Intmax Rollup Operator

Int max operator node

## Prepara

Install [rustup](https://www.rust-lang.org/tools/install).

```sh
rustup override set nightly
rustup update nightly
cargo install --force cargo-make
makers install_dep
```

## How to Run
```sh
makers start
```

## How to Ping
```sh
> curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "rpc_methods", "id": 1 }' 127.0.0.1:8081
{"jsonrpc":"2.0","result":{"methods":["eth_sendTransaction"],"version":1},"id":1}
```

# Directory
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── cli
│   └── node
├── core
│   ├── commiter
│   ├── executor
│   ├── exitor
│   ├── query-receiver
│   ├── su-receiver
│   └── tx-receiver
├── infra
│   └── json-rpc
├── primitives
│   ├── db
│   ├── verkle
│   └── zk
└── target
```
