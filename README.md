# ⚛️ OMWEI / SAMS Ecosystem

<div align="center">

![Performance Badge](https://img.shields.io/badge/Latency-5.00ns-verified-brightgreen)
![Version](https://img.shields.io/badge/Version-v0.2.0-blue)
![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)

**Hardware Trust Hierarchy & Post-Quantum Cryptography for Industrial IoT**

[🚀 Quick Start](#-quick-start) • [📚 Documentation](#-documentation) • [🔧 Architecture](#-architecture) • [📦 Crates](#-crates)

</div>

## 🏗️ Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                    OMWEI/SAMS v0.2.0 Ecosystem                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐    │
│  │ omwei-atom  │───▶│ sams-logic   │───▶│ sams-industrial │    │
│  │   (Core)    │    │   -gate      │    │ -ecosystem      │    │
│  │ Trust Hierarchy│   │ Validation   │    │ Master Wrapper  │    │
│  │ 5ns Logic   │    │  Hardware    │    │                 │    │
│  └─────────────┘    └──────────────┘    └─────────────────┘    │
│         │                   │                     │              │
│         ▼                   ▼                     ▼              │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐    │
│  │sams-blackbox│    │sams-ghost    │    │ cyber-monitor   │    │
│  │   Logging    │    │   -node      │    │  Telemetry & UI │    │
│  │ Trust-Aware  │    │ PQC Storage  │    │                 │    │
│  └─────────────┘    └──────────────┘    └─────────────────┘    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## 📦 Crates

| Crate | Version | Role | Key Features |
|-------|---------|------|--------------|
| **[omwei-atom](crates/omwei-atom/)** | v0.1.2 | Core Trust Hierarchy | 5ns Bit 31 logic, Managed/Community Space |
| **[sams-logic-gate](crates/sams-logic-gate/)** | v0.2.0 | Hardware Validation | Zero-latency filtering, AX buffer ops |
| **[sams-blackbox](crates/sams-blackbox/)** | v0.2.0 | Trust-Aware Logging | Sincerity tagging, audit trails |
| **[sams-ghost-node](crates/sams-ghost-node/)** | v0.2.0 | PQC Cryptography | CRYSTALS-Dilithium, quantum-resistant |
| **[cyber-monitor](crates/sams-2.0/)** | v0.2.0 | Telemetry & UI | Real-time monitoring, web interface |
| **[sams-industrial-ecosystem](crates/sams-industrial-ecosystem/)** | v0.2.0 | Master Wrapper | Unified API, complete stack |

## 🚀 Quick Start

### Build Entire Ecosystem
```bash
# Clone and build everything
git clone https://github.com/LelloOmwei/sams-industrial-ecosystem.git
cd sams-industrial-ecosystem
cargo build --workspace

# Run verification examples
cargo run --example verify_v02
cargo run --example bench_trust

# Test all components
cargo test --workspace
```

### Use in Your Project
```toml
[dependencies]
# Complete ecosystem
sams-industrial-ecosystem = "0.2.0"

# Or individual components
omwei-atom = "0.1.2"
sams-logic-gate = "0.2.0"
```

### Basic Usage
```rust
use sams_industrial_ecosystem::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create the synchronized stack
    let stack = SincereStack::new().await?;
    
    // Process atoms with 5ns trust determination
    let atom = omwei_atom::Atom::new(0x12345678, [0x42; 28]);
    let result = stack.process_atom(&atom).await?;
    
    println!("Atom processed: {:?}", result);
    Ok(())
}
```

## ⚡ Performance

- **🔥 Trust Determination:** 5.00ns (hardware-optimized)
- **🚀 PQC Operations:** Sub-millisecond with Silicon Catalyst
- **📊 Logging Overhead:** <100ns per entry
- **🔄 Validation Pipeline:** <1μs end-to-end

## 🔐 Security Features

- **🛡️ Post-Quantum Cryptography:** CRYSTALS-Dilithium & Kyber
- **🔑 Hardware Trust Hierarchy:** Bit 31 Sincerity Protocol
- **📝 Zero-Knowledge Proofs:** Optional privacy features
- **🔒 Silicon Catalyst Integration:** Hardware acceleration

## 📚 Documentation

- **[API Docs](https://docs.rs/sams-industrial-ecosystem)** - Complete API reference
- **[Examples](examples/)** - Usage examples and benchmarks
- **[Architecture Guide](crates/omwei-atom/README.md)** - Core trust hierarchy
- **[PQC Guide](crates/sams-ghost-node/README.md)** - Cryptographic operations

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Documentation tests
cargo test --doc --workspace

# Benchmark performance
cargo run --example bench_trust
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is dual-licensed under **MIT** or **Apache-2.0** - your choice!

## 🙏 Acknowledgments

- **32BSA Consortium** - Semantic Atom standard
- **OMWEI Foundation** - Trust hierarchy research  
- **Silicon Catalyst** - Hardware acceleration
- **CRYSTALS Project** - Post-quantum cryptography

---

<div align="center">

**🚀 Built for the future of Industrial IoT & Agentic AI**

[🌐 Website](https://www.equinibrium.eu) • [📖 Documentation](https://docs.rs/sams-industrial-ecosystem) • [🐛 Issues](https://github.com/LelloOmwei/sams-industrial-ecosystem/issues)

</div>
