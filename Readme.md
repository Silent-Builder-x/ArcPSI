# ArcPSI: MPC-Native Private Set Intersection Protocol

<div align="center">
<img src="https://www.google.com/search?q=https://img.shields.io/badge/Arcium-MPC-orange%3Fstyle%3Dfor-the-badge" />
<img src="https://www.google.com/search?q=https://img.shields.io/badge/Solana-Anchor-blue%3Fstyle%3Dfor-the-badge%26logo%3Dsolana" />
<img src="https://www.google.com/search?q=https://img.shields.io/badge/Privacy-PSI-green%3Fstyle%3Dfor-the-badge" />
</div>

## 🔍 Overview

**ArcPSI** is a high-performance, privacy-preserving contact discovery protocol built on **Arcium** and **Solana**.

Traditional social platforms require users to upload their entire raw address book to a central server to "find friends," creating a massive privacy vulnerability. **ArcPSI** solves this using **Secure Multi-Party Computation (MPC)**. Users submit encrypted hashes of their contacts (as secret shares), and Arcium's **Multi-Party Execution (MXE)** environment computes the intersection without ever learning the contents of the user's list or the platform's database.

## 🚀 Live Deployment Status (Devnet v0.8.3)

The protocol is fully operational and verified on the Arcium Devnet.

### 🖥️ Interactive Demo

[Launch ArcPSI Terminal](https://silent-builder-x.github.io/ArcPSI/)

## 🧠 Core Innovation: "Silent Discovery"

ArcPSI implements a classic cryptographic primitive for the decentralized era:

- **Shielded Matching:** Utilizes Arcis MPC circuits to iterate through secret-shared sets using constant-time multiplexers.
- **Zero-Leakage Onboarding:** Only mutually shared contacts are identified; non-matching contacts remain cryptographically hidden from node operators and the server.
- **Solana Settlement:** Final discovery proofs are committed to the Solana ledger via verified MXE callbacks.

## 🛠 Build & Implementation

```
# Compile Arcis circuits and Anchor program
arcium build

# Deploy to Cluster 456
arcium deploy --cluster-offset 456 --recovery-set-size 4 --keypair-path ~/.config/solana/id.json -u d

```

## 📄 Technical Specification

- **Engine:** `match_contacts` (Arcis-MPC Circuit)
- **Security:** Supported by Arcium's multi-party threshold signatures and recovery set.
- **Efficiency:** Optimized $O(N \times M)$ comparison logic in the encrypted domain.