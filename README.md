# 🧩 steinlib — A Rust Parser for STP Files (Steiner Tree Problem Format)

<!-- [![Crates.io](https://img.shields.io/crates/v/steinlib.svg)](https://crates.io/crates/steinlib)
[![Documentation](https://docs.rs/steinlib/badge.svg)](https://docs.rs/steinlib)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) -->

**steinlib** is a Rust library for parsing `.stp` files — the standard format used in the [SteinLib](https://steinlib.zib.de/) collection for **Steiner Tree Problem (STP)** instances.

It provides a convenient and type-safe way to load and work with STP graph data, terminals, and metadata, enabling research, algorithm development, and experimentation in combinatorial optimization.

---

## ✨ Features

- ✅ Parses **Graph** and **Terminals** sections of `.stp` files  
- ✅ Returns structured, strongly typed `SteinerInstance` data  
- ✅ Supports edge costs as `f64`  
- ✅ Robust handling of whitespace and formatting variations  
- ✅ Tested against canonical SteinLib examples  
- 🧪 Easy to integrate with algorithmic solvers

---

## 📦 Installation

Add the following line to your `Cargo.toml`:

```toml
[dependencies]
steinlib = { git = "https://github.com/Leon-Degel-Koehn/steinlib.git" }
