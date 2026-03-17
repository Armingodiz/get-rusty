# Rust EVM Transaction Enrichment Engine

## Overview
Build a Rust-based engine that converts raw EVM blockchain transaction data into a **wallet-friendly, structured transaction format**.

The system fetches data from RPC nodes (transaction, receipt, logs, traces) and transforms it into normalized actions such as transfers, approvals, swaps, and smart-wallet executions.

## Goal
Provide a reusable Rust core library and a thin API that helps wallet systems understand **what actually happened in a transaction** and present it clearly in wallet history.

## Core Responsibilities
- Fetch blockchain data (transaction, receipt, logs, traces)
- Decode ABI events and transaction inputs
- Parse internal calls and native transfers from traces
- Normalize actions into a consistent transaction model
- Classify transaction intent (transfer, approval, swap, contract call, etc.)
- Generate a human-readable summary

## Example Output
A normalized transaction result should include:

- Transaction metadata (hash, block, status, gas)
- Participants (initiator, sender, contracts involved)
- Asset movements (ETH, ERC20, NFTs)
- Approvals and permissions granted
- High-level classification of the transaction
- Human-readable summary for wallet UI

Example structure:

```json
{
  "chain_id": 1,
  "tx_hash": "0x...",
  "status": "success",
  "timestamp": 1712345678,
  "from": "0x...",
  "to": "0x...",
  "kind": "swap",
  "summary": "Swapped 120 USDC for 0.043 ETH",
  "actions": [],
  "approvals": [],
  "native_transfers": [],
  "gas": {},
  "counterparties": []
}