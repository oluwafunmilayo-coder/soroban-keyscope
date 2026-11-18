# soroban-keyscope

Storage key collision and namespace isolation verifier for Soroban smart contracts.

## Overview

Soroban smart contracts store data across three storage domains: **Instance**, **Persistent**, and **Temporary**. When data keys are structured dynamically (e.g., tuples, symbols, custom datatypes), unintentional namespace overlap can lead to critical state overwrites, storage integrity bugs, or unexpected collision hazards.

`soroban-keyscope` is a static analysis and verification tool that inspects contract storage schemas to ensure clean namespace segregation, collision resistance, and optimal ledger footprint efficiency.

### Key Capabilities

- **Key Collision Detection**: Scans contract data structures to detect overlapping key spaces between distinct logical entities (e.g., user balances vs. administrative configuration).
- **Storage Tier Validation**: Enforces appropriate tier usage (Instance vs. Persistent vs. Temporary) based on data lifetime expectations.
- **Footprint Optimization**: Analyzes storage key byte overhead and footprint access patterns for execution cost reduction.
- **Enum Namespace Verification**: Asserts proper enum-based key prefixing conventions to guarantee mathematical key isolation.

## Getting Started

### Installation

```bash
cargo install --path .
```

### Usage

```bash
# Analyze a Soroban contract workspace
soroban-keyscope analyze --path ./contracts/my_contract

# Check specific WASM bytecode for storage key safety
soroban-keyscope verify --wasm ./target/wasm32-unknown-unknown/release/contract.wasm
```

## Roadmap

- [ ] Storage key parser and type discriminator
- [ ] Namespace collision detection across contract storage tiers
- [ ] Enum-based key isolation verifier
- [ ] Storage footprint efficiency and ledger TTL audit reporter

## License

Apache-2.0 or MIT
