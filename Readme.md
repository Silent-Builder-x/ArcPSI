# ArcPSI: FHE-Native Private Set Intersection Protocol

## 🔍 Overview

**ArcPSI** is a high-performance, privacy-preserving contact discovery protocol built on **Arcium** and **Solana**.

Traditional social platforms require users to upload their entire raw address book to a central server to "find friends," creating a massive privacy vulnerability. **ArcPSI** solves this using **Fully Homomorphic Encryption (FHE)**. Users submit encrypted hashes of their contacts, and Arcium's **Multi-Party Execution (MXE)** environment computes the intersection without ever learning the contents of the user's list or the platform's database.

## 🚀 Live Deployment Status (Devnet)

The protocol is fully operational and verified on the Arcium Devnet.

- **MXE Address:** `BR1meYP2DW2e4YSvvpBUXFAijdpfkzVFE6a9QGacggZ2`
- **MXE Program ID:** `4g2oDfoiXaXYFrDu28PRPKb85Kh6kmK3pwgvhjd3xGZA`
- **Computation Definition:** `He1U93osbGxzNpUmMiQAvEJVGJhUJUwFrtM2Hap2yvn1`
- **Status:** `Active`

## 🧠 Core Innovation: "Silent Discovery"

ArcPSI implements a classic cryptographic primitive for the decentralized era:

- **Shielded Matching:** Utilizes Arcis FHE circuits to iterate through encrypted sets using constant-time multiplexers.
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

- **Engine:** `match_contacts` (Arcis-FHE Circuit)
- **Security:** Supported by Arcium's multi-party threshold signatures and recovery set.
- **Efficiency:** Optimized $O(N \times M)$ comparison logic in the encrypted domain.