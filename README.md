# Intmax Rollup Operator

Int max operator node

## Prepara
```sh
rustup override set nightly
rustup update
```

## How to Run
```sh
cd cli/node && cargo run
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
