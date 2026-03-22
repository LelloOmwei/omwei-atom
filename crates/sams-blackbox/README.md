# SAMS Blackbox

**Trust-aware logging and audit trail system for OMWEI 32BSA ecosystem with Sincerity Bit tagging.** Provides comprehensive logging with automatic trust-level classification and audit trail generation.

## 🎯 Mission

SAMS Blackbox serves as the central logging and audit trail component for the OMWEI 32BSA ecosystem, providing detailed, trust-aware logging with automatic classification based on the Sincerity Bit.

## 🏗️ Architecture

```
SAMS BLACKBOX ARCHITECTURE
┌─────────────────────────────────────────────────────┐
│            Trust-Aware Logging Engine          │
│  ┌──────────────┬──────────────────┐      │
│  │ Sincerity    │ Audit Trail     │      │
│  │ Tagging       │ Generation      │      │
│  └──────────────┴──────────────────┘      │
│                   │                          │
│         Trust-Level Classification          │
│    (Managed vs Community)              │
└─────────────────────────────────────────────────────┘
```

## ⚡ Features

- **Trust-Level Tagging:** Automatic classification based on Sincerity Bit
- **Audit Trail Generation:** Complete processing history with traceability
- **High-Performance Storage:** Optimized for embedded systems
- **Compression Support:** LZ4 compression for long-term storage
- **Database Integration:** SQLite backend for persistent storage

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
sams-blackbox = "0.2.0"
omwei-atom = "0.1.2"
```

### Industrial-Grade Implementation

```rust
use sams_blackbox::{SamsBlackbox, LogCategory};
use omwei_atom::{Atom, ValidationResult};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the blackbox
    let mut blackbox = SamsBlackbox::new(None).await?;
    
    // Process atoms with trust-aware logging
    let atom = Atom::new(0x12345678, [0x42; 28]);
    let result = ValidationResult::Trusted;
    
    // Log with automatic trust-level tagging
    blackbox.log_atom(
        &atom, 
        &result, 
        8600, 
        LogCategory::Audit, 
        "Atom processed successfully".to_string()
    ).await?;
    
    // Query by trust level
    let sincere_logs = blackbox.query_by_trust_level(
        omwei_atom::TrustLevel::Managed
    );
    
    println!("✅ Sincere logs: {}", sincere_logs.len());
    
    Ok(())
}
```

## 🛡️ Trust-Level Logging

### Sincere Logs (Managed Space)
- **Tag:** "Sincere/Verified"
- **Metadata:** PQC verification status
- **Category:** Audit, Security, Performance
- **Priority:** High (global trust)

### Community Logs (Community Space)
- **Tag:** "Experimental/Unverified"
- **Metadata:** Local verification only
- **Category:** Info, Warning, Development
- **Priority:** Medium (local trust)

## 📊 Log Categories

### Audit Trail
- Complete processing history
- Trust level changes
- Node-to-node transfers
- PQC verification results

### Security Events
- Invalid PQC signatures
- Trust level violations
- Authentication attempts
- Access control events

### Performance Metrics
- Processing latency
- Trust determination speed
- PQC verification time
- Resource utilization

### System Events
- Component initialization
- Configuration changes
- Health checks
- Error conditions

## 🔧 Configuration

### Basic Configuration
```rust
use sams_blackbox::{SamsBlackbox, BlackboxConfig};

let config = BlackboxConfig {
    max_entries: 10000,
    max_trail_length: 100,
    database_url: None,
};

let blackbox = SamsBlackbox::new(Some(config)).await?;
```

### Database Integration
```toml
[dependencies]
sams-blackbox = { version = "0.2.0", features = ["database"] }
```

```rust
let config = BlackboxConfig {
    max_entries: 100000,
    max_trail_length: 1000,
    database_url: Some("sqlite:///tmp/sams_blackbox.db".to_string()),
};

let blackbox = SamsBlackbox::new(Some(config)).await?;
```

### Feature Flags
- `std`: Standard library support (default)
- `database`: SQLite backend for persistent storage
- `compression`: LZ4 compression for long-term storage
- `pqc`: Post-quantum cryptography logging

## 📈 Query Interface

### Trust-Level Queries
```rust
// Query all sincere logs
let sincere_logs = blackbox.query_by_trust_level(TrustLevel::Managed);

// Query all community logs
let community_logs = blackbox.query_by_trust_level(TrustLevel::Community);
```

### Category Queries
```rust
// Query security events
let security_logs = blackbox.query_by_category(LogCategory::Security);

// Query performance metrics
let performance_logs = blackbox.query_by_category(LogCategory::Performance);
```

### Time Range Queries
```rust
use chrono::{Utc, Duration};

let start = Utc::now() - Duration::hours(24);
let end = Utc::now();

let recent_logs = blackbox.query_by_time_range(start, end);
```

### Audit Trail Queries
```rust
// Get complete audit trail for atom
let trail = blackbox.get_audit_trail(atom_id);

if let Some(trail) = trail {
    for step in trail {
        println!("Step: {:?}, Result: {:?}", step.step, step.result);
    }
}
```

## 📊 Statistics and Reporting

### Trust Statistics
```rust
let stats = blackbox.get_trust_statistics();

println!("Total atoms: {}", stats.total_atoms);
println!("Managed atoms: {}", stats.managed_atoms);
println!("Community atoms: {}", stats.community_atoms);
println!("Managed ratio: {:.2}%", stats.get_managed_ratio() * 100.0);
println!("Avg processing time: {:.2} ns", stats.avg_processing_time_ns);
```

### Audit Reports
```rust
use chrono::{Utc, Duration};

let start = Utc::now() - Duration::days(7);
let end = Utc::now();

let report = blackbox.generate_audit_report(start, end);

println!("Audit Report:");
println!("Period: {} to {}", report.period_start, report.period_end);
println!("Total entries: {}", report.total_entries);
println!("Sincere: {}, Community: {}", report.managed_count, report.community_count);
println!("Average latency: {:.2} ns", report.avg_latency_ns);
```

## 🔬 Advanced Features

### Structured Logging
```rust
// Log with rich metadata
blackbox.log_atom(
    &atom,
    &result,
    8600,
    LogCategory::Security,
    "PQC signature verification completed".to_string()
).await?;

// The log automatically includes:
// - Trust level (Managed/Community)
// - Sincerity Bit value
// - PQC verification status
// - Processing latency
// - Node ID and chain ID
```

### Custom Metadata
```rust
// The blackbox automatically captures:
let metadata = serde_json::json!({
    "blackbox_id": blackbox_id,
    "payload_size": atom.payload.len(),
    "sincerity_bit": (atom.global_id & 0x80000000) != 0,
    "pqc_verified": trust_metadata.pqc_verified,
});
```

### Performance Optimization
```rust
// High-performance logging with minimal overhead
// - Stack-only processing
// - Zero-copy serialization
// - Batch database writes
// - LZ4 compression for storage
```

## 📄 License

Licensed under **MIT OR Apache-2.0** license - chosen for maximum compatibility in industrial and aerospace applications.

## 🎯 Strategic Context

**SAMS Blackbox v0.2.0** provides the critical audit and logging infrastructure for OMWEI 32BSA deployments, ensuring complete traceability with trust-level awareness.

**Use Cases:**
- **Autonomous Vehicle Systems:** Complete audit trails for safety compliance
- **Industrial IoT Operations:** Trust-aware logging for regulatory compliance
- **Aerospace Mission Control:** Detailed telemetry logging with trust metadata
- **Medical Device Systems:** HIPAA-compliant audit trails with trust verification

**Version:** `0.2.0` - Synchronization Release

---

## 🏭 Sincerity Compliance Badge

[![Sincerity Compliant](https://img.shields.io/badge/Sincerity-OMWEI%2032BSA%20v0.1.2-blue)](https://github.com/LelloOmwei/omwei-atom)
[![Trust Logging](https://img.shields.io/badge/Logging-Aware%20Trust%20Levels-green)](https://github.com/LelloOmwei/omwei-atom)
[![Audit Trail](https://img.shields.io/badge/Audit-Complete%20Traceability-orange)](https://github.com/LelloOmwei/omwei-atom)

**SAMS Blackbox** - Recording the journey of silicon sincerity, one trust bit at a time.

---

**OMWEI Project** - [https://github.com/LelloOmwei/omwei-atom](https://github.com/LelloOmwei/omwei-atom)
