# Soroban Storage Layout Specification

## Overview
Soroban smart contracts utilize three storage tiers:
- **Instance**: Bound to the contract instance footprint. Suitable for admin addresses, token metadata, and state switches.
- **Persistent**: Long-lived data that requires active TTL renewal. Used for balances, user records, and liquidity pools.
- **Temporary**: Ephemeral storage cleared on ledger boundary expiration. Used for nonces, signatures, and time-bounded sessions.

## Key Framing Rules
1. Keys must use structured enum variants or tuple headers to prevent unintended key overwrites across components.
2. Tuple keys must lead with a discriminant to avoid collisions across multiple mappings.
