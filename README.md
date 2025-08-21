# ⏳ Anchor Chrono Vault

A cross-platform Solana program and client for time-locked vaults, written in 🦀 Rust and ⚡️ TypeScript, powered by the [Anchor framework](https://book.anchor-lang.com/).

---

## 🕰 Overview

**anchor-chrono-vault** is a secure, programmable vault for locking tokens or NFTs on the Solana blockchain until a specified unlock time. It enables use cases such as vesting, delayed payments, or time-based access control—all handled trustlessly on-chain.

---

## ✨ Features

- ⏰ Time-locked vaults for SPL tokens and NFTs
- 🔒 Trustless, on-chain access control
- 🛠 Built with Anchor for secure, maintainable development
- 🟦 TypeScript client for easy integration with dApps and UIs

---

## 🛠 Getting Started

### 📋 Prerequisites

- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- ☀️ [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- ⚓️ [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
- 🟩 [Node.js](https://nodejs.org/) (for TypeScript client)

### ⬇️ Installation

```bash
git clone https://github.com/bytehash69/anchor-chrono-vault.git
cd anchor-chrono-vault
```

### 🏗 Build the Program

```bash
anchor build
```

### 🧪 Run Tests

```bash
anchor test
```

### 🛳 Deploy to Localnet

```bash
anchor localnet
```

---

## 📦 Usage

### 📝 Rust (On-Chain Program)

- Core logic is implemented in `programs/anchor-chrono-vault`.
- Add or customize instructions for creating vaults, locking/unlocking assets, and more.

### 🟦 TypeScript (Client SDK)

- The TypeScript client (see `tests/` or `client/`) lets you interact with the program.
- Example scripts show how to lock tokens, query vault status, and unlock assets after the time elapses.

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests. For big changes, start a discussion first.

---

## 📄 License

Licensed under the MIT License.

---

## ✉️ Contact

Maintained by [bytehash69](https://github.com/bytehash69).

---

> ⚠️ **This project is not audited. Use at your own risk!**
