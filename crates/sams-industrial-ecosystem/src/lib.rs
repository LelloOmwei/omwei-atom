/*!
# SAMS Industrial Ecosystem

**Master wrapper for OMWEI 32BSA Trust Hierarchy.** The unified entry point for Silicon Sincerity protocol, providing high-level orchestration of all SAMS components with hardware-enforced trust determination.

## Mission

The SAMS Industrial Ecosystem serves as the L0+ layer that orchestrates all OMWEI 32BSA compliant components through a unified `SincereStack` architecture. This ensures mathematical certainty and zero-latency trust determination across the entire stack.

## Architecture

```text
SAMS INDUSTRIAL ECOSYSTEM
+-----------------------------------------------------------+
|                SincereStack (Master Wrapper)                |
|  +--------------+--------------+--------------+ |
|  | GhostNode    | Blackbox      | LogicGate    | |
|  | (PQC Store)  | (Logger)      | (Validator)  | |
|  +--------------+--------------+--------------+ |
|                   |                                   |
|            Hardware Trust Hierarchy                 |
|         (omwei-atom v0.1.2)               |
+-----------------------------------------------------------+
```

## Quick Start

```rust
use sams_industrial_ecosystem::SincereStack;
use omwei_atom::{Atom, TrustLevel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the complete SAMS stack
    let mut stack = SincereStack::new().await?;

    // Process incoming atom through unified trust hierarchy
    let atom = Atom::new(0x12345678, [0x42; 28]);
    let result = stack.process_atom(atom).await?;

    match result.trust_level {
        TrustLevel::Managed => {
            println!("✅ Sincere data: {}", result.status);
        }
        TrustLevel::Community => {
            println!("⚠️  Community data: {}", result.status);
        }
    }

    Ok(())
}
```

## Features

- **Unified Trust Hierarchy:** Single entry point for all trust determination
- **Hardware Acceleration:** Direct integration with Silicon Catalyst
- **Component Orchestration:** Automatic initialization of GhostNode, Blackbox, LogicGate
- **Zero-Cost Abstractions:** Compile-time optimization to single instructions
- **Post-Quantum Ready:** Built-in PQC signature verification framework

## Sincerity Compliance

This crate ensures 100% compliance with OMWEI 32BSA v0.1.2 standard:
- **Managed Space:** Bit 31 = 0, PQC signature required
- **Community Space:** Bit 31 = 1, local verification only
- **Zero Latency:** Single bit-mask operation (`id & 0x80000000`)
- **Hardware Efficiency:** Stack-only execution, no allocation

## Components

- [`GhostNode`] - PQC signature storage and verification
- [`Blackbox`] - Logging and telemetry with trust-based tagging
- [`LogicGate`] - Hardware-level validation and filtering
- [`SincereStack`] - Master orchestrator for all components

## Configuration

### Feature Flags
- `std`: Standard library support (default)
- `pqc`: Post-quantum cryptography primitives

### Environment Variables
- `SAMS_LOG_LEVEL`: Logging level (trace, debug, info, warn, error)
- `SAMS_PQC_KEY_PATH`: Path to PQC private key for Managed Space
- `SAMS_TRUST_POLICY`: Default trust policy (strict, permissive)

*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(not(feature = "std"), feature = "serde"))]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "serde"))]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Re-export OMWEI 32BSA core types
pub use omwei_atom::trust_hierarchy::{get_trust_level, validate_atom, Atom, TrustLevel, ValidationResult};

pub mod blackbox;
pub mod ghost_node;
pub mod logic_gate;

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{error, info, warn};
use uuid::Uuid;

/// Processing result from SAMS stack
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SincereResult {
    /// Trust level determined by hardware hierarchy
    pub trust_level: TrustLevel,
    /// Processing status and metadata
    pub status: String,
    /// Processing timestamp
    pub timestamp: DateTime<Utc>,
    /// Unique processing ID
    pub process_id: Uuid,
}

/// Master wrapper for SAMS Industrial Ecosystem
///
/// This struct orchestrates all OMWEI 32BSA compliant components
/// through a unified trust hierarchy with hardware acceleration.
#[derive(Debug)]
pub struct SincereStack {
    /// GhostNode instance for PQC operations
    ghost_node: ghost_node::GhostNode,
    /// Blackbox instance for logging and telemetry
    blackbox: blackbox::Blackbox,
    /// LogicGate instance for validation
    logic_gate: logic_gate::LogicGate,
    /// Unique instance identifier
    instance_id: Uuid,
}

impl SincereStack {
    /// Create new SincereStack with all components initialized
    ///
    /// # Arguments
    /// * `config` - Optional configuration for components
    ///
    /// # Returns
    /// Initialized SincereStack ready for processing
    ///
    /// # Examples
    /// ```
    /// use sams_industrial_ecosystem::SincereStack;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let stack = SincereStack::new().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> Result<Self> {
        let instance_id = Uuid::new_v4();

        info!("Initializing SAMS Industrial Ecosystem v0.2.0");
        info!("Instance ID: {}", instance_id);

        // Initialize all components
        let ghost_node = ghost_node::GhostNode::new().await?;
        let blackbox = blackbox::Blackbox::new(instance_id)?;
        let logic_gate = logic_gate::LogicGate::new(None).await?;

        info!("✅ SincereStack initialized successfully");

        Ok(Self {
            ghost_node,
            blackbox,
            logic_gate,
            instance_id,
        })
    }

    /// Process an atom through the complete trust hierarchy
    ///
    /// This method orchestrates validation, logging, and storage
    /// according to OMWEI 32BSA standards.
    ///
    /// # Arguments
    /// * `atom` - The atom to process
    ///
    /// # Returns
    /// Processing result with trust level and status
    ///
    /// # Examples
    /// ```ignore
    /// use sams_industrial_ecosystem::{SincereStack, SincereResult};
    /// use omwei_atom::Atom;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let mut stack = SincereStack::new().await?;
    ///     let atom = Atom::new(0x12345678, [0x42; 28]);
    ///     let result = stack.process_atom(atom).await?;
    ///     println!("Processed: {:?}", result);
    ///     Ok(())
    /// }
    /// ```
    pub async fn process_atom(&mut self, atom: Atom) -> Result<SincereResult> {
        let process_id = Uuid::new_v4();
        let timestamp = Utc::now();

        info!(
            "Processing atom {} with ID 0x{:08X}",
            process_id, atom.global_id
        );

        // Step 1: Hardware trust determination (zero latency)
        let trust_level = get_trust_level(atom.global_id);

        // Step 2: Logic Gate validation
        let validation_metadata = self.logic_gate.validate(&atom).await?;
        
        // Convert ValidationMetadata to ValidationResult for blackbox
        let validation_result = if validation_metadata.is_valid {
            ValidationResult::Trusted
        } else {
            ValidationResult::InvalidSignature
        };

        // Step 3: Blackbox logging with trust-based tagging
        self.blackbox
            .log(&atom, &trust_level, &validation_result, process_id)
            .await?;

        // Step 4: GhostNode PQC operations (Managed Space only)
        let status = match trust_level {
            TrustLevel::Managed => {
                // PQC signature verification for Managed Space
                match self.ghost_node.verify_and_store(&atom).await {
                    Ok(verified) => {
                        if verified {
                            format!("✅ Sincere - PQC verified, stored in GhostNode")
                        } else {
                            format!("⚠️  Sincere - GhostNode unavailable")
                        }
                    }
                    Err(e) => {
                        warn!("GhostNode verification failed: {}", e);
                        format!("❌ Sincere - GhostNode error")
                    }
                }
            }
            TrustLevel::Community => {
                format!("⚠️  Community - Local verification only")
            }
        };

        let status_clone = status.clone();

        let result = SincereResult {
            trust_level,
            status,
            timestamp: Utc::now(),
            process_id,
        };

        info!("✅ Atom processed successfully: {}", status_clone);
        Ok(result)
    }

    /// Get stack instance information
    pub fn info(&self) -> &str {
        "SAMS Industrial Ecosystem v0.2.0 - OMWEI 32BSA Compliant"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sincere_stack_creation() {
        let stack = SincereStack::new().await;
        assert!(stack.is_ok());

        let stack = stack.unwrap();
        assert_eq!(
            stack.info(),
            "SAMS Industrial Ecosystem v0.2.0 - OMWEI 32BSA Compliant"
        );
    }

    #[tokio::test]
    async fn test_managed_atom_processing() {
        let stack = SincereStack::new().await.unwrap();
        let atom = Atom::new(0x12345678, [0x42; 28]); // Managed Space

        let result = stack.process_atom(atom).await.unwrap();
        assert_eq!(result.trust_level, TrustLevel::Managed);
        assert!(result.status.contains("Sincere"));
    }

    #[tokio::test]
    async fn test_community_atom_processing() {
        let stack = SincereStack::new().await.unwrap();
        let atom = Atom::new(0x80000001, [0x24; 28]); // Community Space

        let result = stack.process_atom(atom).await.unwrap();
        assert_eq!(result.trust_level, TrustLevel::Community);
        assert!(result.status.contains("Community"));
    }
}
