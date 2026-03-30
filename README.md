# ChainCircle Contracts

> **Save Together. Trust the Chain.**

Soroban smart contracts for the ChainCircle platform — a Stellar-based community savings and lending circle system.

[![CI](https://github.com/RubysWorld1/ChainCircle-Contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/RubysWorld1/ChainCircle-Contracts/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Contracts

| Contract | Description | Status |
|---|---|---|
| `savings_circle` | Core savings circle logic — create, contribute, payout rotation | 🚧 Scaffolded |
| `credit_score` | On-chain credit scoring based on contribution and repayment history | 🚧 Scaffolded |
| `microloan` | Microloan disbursement, repayment, and default tracking | 🚧 Scaffolded |

---

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- WASM target:
```bash
rustup target add wasm32-unknown-unknown
```
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup):
```bash
cargo install --locked soroban-cli
```

---

## Getting Started

```bash
git clone https://github.com/RubysWorld1/ChainCircle-Contracts.git
cd ChainCircle-Contracts

# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Run all tests
cargo test
```

---

## Project Structure

```
contracts/
├── savings_circle/
│   ├── src/lib.rs       # Circle creation, contributions, payout rotation
│   └── Cargo.toml
├── credit_score/
│   ├── src/lib.rs       # Score tracking, loan eligibility check
│   └── Cargo.toml
└── microloan/
    ├── src/lib.rs       # Loan disbursement, repayment, default marking
    └── Cargo.toml
Cargo.toml               # Workspace config
```

---

## Open TODOs (Wave Bounty Issues)

These are scoped tasks available for contributors to earn Drips Wave rewards:

| TODO | Contract | Complexity |
|---|---|---|
| Integrate Stellar token contract for contributions | `savings_circle` | Advanced |
| Implement multi-sig approval before payout release | `savings_circle` | Advanced |
| Cross-contract call to credit score on contribution | `savings_circle` | Intermediate |
| Add oracle/admin verification before marking default | `microloan` | Intermediate |
| Cross-contract call to credit score on repay/default | `microloan` | Intermediate |
| Add multi-sig or oracle verification before penalizing | `credit_score` | Intermediate |
| Write full test suite for all edge cases | All contracts | Beginner–Intermediate |
| Deploy contracts to Stellar testnet + document addresses | All contracts | Beginner |

---

## Related Repositories

- [ChainCircle Backend](https://github.com/RubysWorld1/ChainCircle-Backend) — NestJS API

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

[MIT](LICENSE)
