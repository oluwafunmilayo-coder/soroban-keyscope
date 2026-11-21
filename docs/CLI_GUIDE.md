# Soroban Keyscope CLI Guide

`soroban-keyscope` inspects, analyzes, and visualizes Soroban smart contract storage key footprints, domain distribution, and TTL expiration alerts.

---

## 1. Commands & Options

```bash
# Analyze contract storage key layout
keyscope analyze --contract <CONTRACT_ID> --rpc-url <RPC_URL>

# Export footprint report as ASCII or HTML visualizer
keyscope analyze --contract <CONTRACT_ID> --format html --out report.html
keyscope analyze --contract <CONTRACT_ID> --format csv --out report.csv

# Run TTL expiration linting with minimum ledger threshold
keyscope lint --contract <CONTRACT_ID> --min-ttl-ledgers 50000
```

---

## 2. Storage Domains & Quota Guidance

| Domain | Best Use Cases | TTL Lifecycle |
|---|---|---|
| **Instance** | Small singleton admin configs, fee rates, protocol state | Tied to contract instance; keep footprint minimal (< 256 B) |
| **Persistent** | Balances, escrow records, immutable historical positions | Independent TTL; requires periodic rent bump |
| **Temporary** | Replay nonces, time-bound approvals, short-lived session auth | Auto-evicted on expiration without archiving |

---

## 3. TTL Alert Levels

- **Safe**: Key remaining lifetime $\ge$ configured threshold.
- **Warning**: Remaining lifetime is below threshold but above critical threshold.
- **Critical**: Lifetime is below $50\%$ of threshold or already expired. Immediate rent bump required.
