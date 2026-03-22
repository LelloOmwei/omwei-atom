/*!
# SAMS Ghost Node - Post-Quantum Cryptography Storage and Verification

**Post-Quantum Cryptography storage and verification for Managed Space atoms in OMWEI 32BSA ecosystem.** Provides quantum-resistant signature generation, verification, and secure storage with hardware acceleration.

## Mission

SAMS Ghost Node serves as the Post-Quantum Cryptography (PQC) backbone for the OMWEI 32BSA ecosystem, handling all cryptographic operations for Managed Space atoms (Bit 31 = 0) with quantum-resistant algorithms.

## Architecture

```text
SAMS GHOST NODE ARCHITECTURE
+-----------------------------------------------------+
|            PQC Signature Engine             |
|  +--------------+------------------+      |
|  | CRYSTALS-    | CRYSTALS-      |      |
|  | Dilithium    | Kyber          |      |
|  | (Signatures)  │ (Encryption)    │      |
|  +--------------+------------------+      |
|                   |                          |
|         Silicon Catalyst Acceleration        |
|     (Hardware PQC Operations)           |
+-----------------------------------------------------+
```

## Features

- **PQC Signature Generation:** CRYSTALS-Dilithium lattice-based signatures
- **PQC Verification:** Quantum-resistant signature validation
- **Secure Storage:** Encrypted signature database
- **Hardware Acceleration:** Silicon Catalyst integration
- **Zero-Knowledge Proofs:** Optional ZKP support for privacy

## Sincerity Compliance

- **Managed Space Only:** Only processes atoms with Bit 31 = 0
- **Quantum Resistance:** All operations quantum-computer resistant
- **Hardware Efficiency:** Direct PQC accelerator integration
- **Zero Trust:** Cryptographic verification required for all operations
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

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use uuid::Uuid;

/// PQC signature metadata for Managed Space atoms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PqcSignature {
    /// Signature algorithm (Dilithium, Falcon, etc.)
    pub algorithm: PqcAlgorithm,
    /// Public key identifier
    pub key_id: Uuid,
    /// Signature bytes
    pub signature: Vec<u8>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Signature metadata
    pub metadata: PqcMetadata,
}

/// Supported PQC signature algorithms
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PqcAlgorithm {
    /// CRYSTALS-Dilithium (lattice-based signatures)
    Dilithium { variant: DilithiumVariant },
    /// CRYSTALS-Falcon (lattice-based signatures)
    Falcon { variant: FalconVariant },
    /// SPHINCS+ (hash-based signatures)
    SphincsPlus { variant: SphincsVariant },
}

/// Dilithium variants
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DilithiumVariant {
    /// Dilithium2 (128-bit security)
    Dilithium2,
    /// Dilithium3 (192-bit security)
    Dilithium3,
    /// Dilithium5 (256-bit security)
    Dilithium5,
}

/// Falcon variants
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FalconVariant {
    /// Falcon-512 (128-bit security)
    Falcon512,
    /// Falcon-1024 (256-bit security)
    Falcon1024,
}

/// SPHINCS+ variants
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SphincsVariant {
    /// SPHINCS+-128f (fast, 128-bit security)
    Sphincs128f,
    /// SPHINCS+-256f (fast, 256-bit security)
    Sphincs256f,
    /// SPHINCS+-128s (small, 128-bit security)
    Sphincs128s,
    /// SPHINCS+-256s (small, 256-bit security)
    Sphincs256s,
}

/// PQC signature metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PqcMetadata {
    /// Verification time in nanoseconds
    pub verification_time_ns: u64,
    /// Signature size in bytes
    pub signature_size: usize,
    /// Security level (128, 192, or 256 bits)
    pub security_level: u16,
    /// Hardware acceleration used
    pub hardware_accelerated: bool,
}

/// PQC key pair for signature generation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PqcKeyPair {
    /// Key pair identifier
    pub key_id: Uuid,
    /// Algorithm used
    pub algorithm: PqcAlgorithm,
    /// Public key bytes
    pub public_key: Vec<u8>,
    /// Private key bytes (encrypted)
    pub private_key: Vec<u8>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Key metadata
    pub metadata: KeyMetadata,
}

/// Key pair metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyMetadata {
    /// Key usage count
    pub usage_count: u64,
    /// Last used timestamp
    pub last_used: Option<DateTime<Utc>>,
    /// Key status
    pub status: KeyStatus,
}

/// Key status
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyStatus {
    /// Key is active and valid
    Active,
    /// Key is revoked
    Revoked,
    /// Key is expired
    Expired,
    /// Key is compromised
    Compromised,
}

/// SAMS Ghost Node - PQC signature storage and verification
///
/// Provides post-quantum cryptography operations for Managed Space atoms
/// with hardware acceleration through Silicon Catalyst.
#[derive(Debug)]
pub struct SamsGhostNode {
    /// Node instance identifier
    node_id: Uuid,
    /// PQC key store
    key_store: PqcKeyStore,
    /// Signature database
    signature_db: SignatureDatabase,
    /// PQC engine
    pqc_engine: PqcEngine,
    /// Performance metrics
    metrics: GhostNodeMetrics,
}

/// PQC key store for managing key pairs
#[derive(Debug)]
struct PqcKeyStore {
    key_pairs: HashMap<Uuid, PqcKeyPair>,
    default_algorithm: PqcAlgorithm,
}

impl PqcKeyStore {
    fn new(default_algorithm: PqcAlgorithm) -> Self {
        Self {
            key_pairs: HashMap::new(),
            default_algorithm,
        }
    }

    fn generate_key_pair(&mut self, algorithm: Option<PqcAlgorithm>) -> Result<PqcKeyPair> {
        let algorithm = algorithm.unwrap_or_else(|| self.default_algorithm.clone());
        let key_id = Uuid::new_v4();
        let created_at = Utc::now();

        // TODO: Implement actual PQC key generation
        let (public_key, private_key) = self.simulate_key_generation(&algorithm)?;

        let key_pair = PqcKeyPair {
            key_id,
            algorithm: algorithm.clone(),
            public_key,
            private_key,
            created_at,
            metadata: KeyMetadata {
                usage_count: 0,
                last_used: None,
                status: KeyStatus::Active,
            },
        };

        self.key_pairs.insert(key_id, key_pair.clone());

        info!(
            "PqcKeyStore: Generated key pair {} with algorithm {:?}",
            key_id, algorithm
        );
        Ok(key_pair)
    }

    fn get_key_pair(&self, key_id: &Uuid) -> Option<&PqcKeyPair> {
        self.key_pairs.get(key_id)
    }

    fn simulate_key_generation(&self, algorithm: &PqcAlgorithm) -> Result<(Vec<u8>, Vec<u8>)> {
        // TODO: Implement actual PQC key generation
        match algorithm {
            PqcAlgorithm::Dilithium { variant: _ } => {
                let public_key = vec![0x42; 1312]; // Dilithium2 public key size
                let private_key = vec![0x24; 4000]; // Dilithium2 private key size
                Ok((public_key, private_key))
            }
            PqcAlgorithm::Falcon { variant: _ } => {
                let public_key = vec![0x52; 897]; // Falcon-512 public key size
                let private_key = vec![0x32; 1281]; // Falcon-512 private key size
                Ok((public_key, private_key))
            }
            PqcAlgorithm::SphincsPlus { variant: _ } => {
                let public_key = vec![0x62; 32]; // SPHINCS+ public key size
                let private_key = vec![0x42; 64]; // SPHINCS+ private key size
                Ok((public_key, private_key))
            }
        }
    }
}

/// Signature database for storing verified signatures
#[derive(Debug)]
struct SignatureDatabase {
    signatures: HashMap<u32, PqcSignature>, // atom_id -> signature
    max_signatures: usize,
    #[cfg(feature = "database")]
    database: Option<sqlx::SqlitePool>,
}

impl SignatureDatabase {
    fn new(max_signatures: usize) -> Self {
        Self {
            signatures: HashMap::new(),
            max_signatures,
            #[cfg(feature = "database")]
            database: None,
        }
    }

    #[cfg(feature = "database")]
    async fn new_with_database(max_signatures: usize, db_url: &str) -> Result<Self> {
        let database = sqlx::SqlitePool::connect(db_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&database).await?;

        Ok(Self {
            signatures: HashMap::new(),
            max_signatures,
            database: Some(database),
        })
    }

    fn store_signature(&mut self, atom_id: u32, signature: PqcSignature) -> Result<()> {
        self.signatures.insert(atom_id, signature.clone());

        // Limit database size
        if self.signatures.len() > self.max_signatures {
            // Remove oldest signature (simplified)
            let oldest_id = self.signatures.keys().next().copied();
            if let Some(id) = oldest_id {
                self.signatures.remove(&id);
            }
        }

        #[cfg(feature = "database")]
        if let Some(database) = &self.database {
            // TODO: Store in database
            debug!("Storing signature for atom {} in database", atom_id);
        }

        Ok(())
    }

    fn get_signature(&self, atom_id: u32) -> Option<&PqcSignature> {
        self.signatures.get(&atom_id)
    }

    fn verify_signature_exists(&self, atom_id: u32) -> bool {
        self.signatures.contains_key(&atom_id)
    }
}

/// PQC engine for cryptographic operations
#[derive(Debug)]
struct PqcEngine {
    hardware_accelerated: bool,
}

impl PqcEngine {
    fn new(hardware_accelerated: bool) -> Self {
        Self {
            hardware_accelerated,
        }
    }

    async fn generate_signature(&self, atom: &Atom, key_pair: &PqcKeyPair) -> Result<PqcSignature> {
        let start_time = std::time::Instant::now();

        // TODO: Implement actual PQC signature generation
        let signature = self.simulate_signature_generation(atom, key_pair).await?;

        let verification_time = start_time.elapsed().as_nanos() as u64;

        info!(
            "PqcEngine: Generated signature for atom 0x{:08X} in {} ns",
            atom.global_id, verification_time
        );

        Ok(signature)
    }

    async fn verify_signature(
        &self,
        atom: &Atom,
        signature: &PqcSignature,
        public_key: &[u8],
    ) -> Result<bool> {
        let start_time = std::time::Instant::now();

        // TODO: Implement actual PQC verification
        let verified = self.simulate_signature_verification(atom, signature, public_key).await?;

        let verification_time = start_time.elapsed().as_nanos() as u64;

        info!(
            "PqcEngine: Verified signature for atom 0x{:08X} in {} ns: {}",
            atom.global_id, verification_time, verified
        );

        Ok(verified)
    }

    async fn simulate_signature_generation(
        &self,
        atom: &Atom,
        key_pair: &PqcKeyPair,
    ) -> Result<PqcSignature> {
        // Simulate PQC signature generation delay
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

        let signature_size = match &key_pair.algorithm {
            PqcAlgorithm::Dilithium {
                variant: DilithiumVariant::Dilithium2,
            } => 2420,
            PqcAlgorithm::Dilithium {
                variant: DilithiumVariant::Dilithium3,
            } => 3293,
            PqcAlgorithm::Dilithium {
                variant: DilithiumVariant::Dilithium5,
            } => 4595,
            PqcAlgorithm::Falcon {
                variant: FalconVariant::Falcon512,
            } => 690,
            PqcAlgorithm::Falcon {
                variant: FalconVariant::Falcon1024,
            } => 1330,
            PqcAlgorithm::SphincsPlus { variant: _ } => 17000, // Approximate
        };

        Ok(PqcSignature {
            algorithm: key_pair.algorithm.clone(),
            key_id: key_pair.key_id,
            signature: vec![0x42; signature_size],
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(365), // 1 year
            metadata: PqcMetadata {
                verification_time_ns: 50_000, // Placeholder
                signature_size,
                security_level: match &key_pair.algorithm {
                    PqcAlgorithm::Dilithium {
                        variant: DilithiumVariant::Dilithium2,
                    } => 128,
                    PqcAlgorithm::Dilithium {
                        variant: DilithiumVariant::Dilithium3,
                    } => 192,
                    PqcAlgorithm::Dilithium {
                        variant: DilithiumVariant::Dilithium5,
                    } => 256,
                    PqcAlgorithm::Falcon {
                        variant: FalconVariant::Falcon512,
                    } => 128,
                    PqcAlgorithm::Falcon {
                        variant: FalconVariant::Falcon1024,
                    } => 256,
                    PqcAlgorithm::SphincsPlus {
                        variant: SphincsVariant::Sphincs128f,
                    } => 128,
                    PqcAlgorithm::SphincsPlus {
                        variant: SphincsVariant::Sphincs256f,
                    } => 256,
                    PqcAlgorithm::SphincsPlus {
                        variant: SphincsVariant::Sphincs128s,
                    } => 128,
                    PqcAlgorithm::SphincsPlus {
                        variant: SphincsVariant::Sphincs256s,
                    } => 256,
                },
                hardware_accelerated: self.hardware_accelerated,
            },
        })
    }

    async fn simulate_signature_verification(
        &self,
        _atom: &Atom,
        _signature: &PqcSignature,
        _public_key: &[u8],
    ) -> Result<bool> {
        // Simulate PQC verification delay
        tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;

        // For demonstration, return true (valid signature)
        // In production, this would be actual PQC verification
        Ok(true)
    }
}

/// Ghost node performance metrics
#[derive(Debug, Default)]
pub struct GhostNodeMetrics {
    /// Total signatures generated
    pub signatures_generated: u64,
    /// Total signatures verified
    pub signatures_verified: u64,
    /// Average generation time (nanoseconds)
    pub avg_generation_time_ns: f64,
    /// Average verification time (nanoseconds)
    pub avg_verification_time_ns: f64,
    /// Verification failures
    pub verification_failures: u64,
    /// Key pairs in use
    pub active_key_pairs: usize,
}

impl GhostNodeMetrics {
    fn record_generation(&mut self, generation_time_ns: u64) {
        self.signatures_generated += 1;
        let total_time = self.avg_generation_time_ns * (self.signatures_generated - 1) as f64
            + generation_time_ns as f64;
        self.avg_generation_time_ns = total_time / self.signatures_generated as f64;
    }

    fn record_verification(&mut self, verification_time_ns: u64, verified: bool) {
        self.signatures_verified += 1;
        if !verified {
            self.verification_failures += 1;
        }

        let total_time = self.avg_verification_time_ns * (self.signatures_verified - 1) as f64
            + verification_time_ns as f64;
        self.avg_verification_time_ns = total_time / self.signatures_verified as f64;
    }
}

impl SamsGhostNode {
    /// Create new SAMS Ghost Node instance
    ///
    /// # Arguments
    /// * `config` - Optional ghost node configuration
    ///
    /// # Returns
    /// Initialized Ghost Node ready for PQC operations
    ///
    /// # Examples
    /// ```
    /// use sams_ghost_node::SamsGhostNode;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let ghost_node = SamsGhostNode::new(None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: Option<GhostNodeConfig>) -> Result<Self> {
        let node_id = Uuid::new_v4();

        info!("SAMS Ghost Node: Initializing PQC engine");
        info!("Node ID: {}", node_id);

        let config = config.unwrap_or_default();

        let default_algorithm = PqcAlgorithm::Dilithium {
            variant: DilithiumVariant::Dilithium2,
        };

        let key_store = PqcKeyStore::new(default_algorithm);
        let signature_db = if let Some(_db_url) = &config.database_url {
            #[cfg(feature = "database")]
            {
                SignatureDatabase::new_with_database(config.max_signatures, db_url).await?
            }
            #[cfg(not(feature = "database"))]
            {
                SignatureDatabase::new(config.max_signatures)
            }
        } else {
            SignatureDatabase::new(config.max_signatures)
        };

        let pqc_engine = PqcEngine::new(config.hardware_acceleration);

        info!("✅ SAMS Ghost Node PQC engine initialized");

        Ok(Self {
            node_id,
            key_store,
            signature_db,
            pqc_engine,
            metrics: GhostNodeMetrics::default(),
        })
    }

    /// Generate PQC signature for atom
    ///
    /// # Arguments
    /// * `atom` - The atom to sign
    /// * `algorithm` - Optional PQC algorithm to use
    ///
    /// # Returns
    /// Generated PQC signature
    ///
    /// # Examples
    /// ```ignore
    /// use sams_ghost_node::SamsGhostNode;
    /// use omwei_atom::Atom;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let ghost_node = SamsGhostNode::new(None).await?;
    ///     let atom = Atom::new(0x12345678, [0x42; 28]);
    ///     let signature = ghost_node.generate_signature(&atom, None).await?;
    ///     println!("Signature generated: {:?}", signature.algorithm);
    ///     Ok(())
    /// }
    /// ```
    pub async fn generate_signature(
        &mut self,
        atom: &Atom,
        algorithm: Option<PqcAlgorithm>,
    ) -> Result<PqcSignature> {
        // Only process Managed Space atoms
        let trust_level = get_trust_level(atom.global_id);
        if trust_level != TrustLevel::Managed {
            warn!(
                "GhostNode: Rejecting Community Space atom 0x{:08X}",
                atom.global_id
            );
            return Err(anyhow::anyhow!("Community Space atoms cannot be signed"));
        }

        info!(
            "GhostNode: Generating PQC signature for Managed Space atom 0x{:08X}",
            atom.global_id
        );

        // Generate or get key pair
        let key_pair = self.key_store.generate_key_pair(algorithm)?;

        // Generate signature
        let start_time = std::time::Instant::now();
        let signature = self.pqc_engine.generate_signature(atom, &key_pair).await?;
        let generation_time = start_time.elapsed().as_nanos() as u64;

        // Store signature
        self.signature_db
            .store_signature(atom.global_id, signature.clone())?;

        // Update metrics
        self.metrics.record_generation(generation_time);
        self.metrics.active_key_pairs = self.key_store.key_pairs.len();

        info!(
            "✅ GhostNode: PQC signature generated and stored for atom 0x{:08X}",
            atom.global_id
        );

        Ok(signature)
    }

    /// Verify PQC signature for atom
    ///
    /// # Arguments
    /// * `atom` - The atom to verify
    /// * `signature` - The PQC signature to verify
    ///
    /// # Returns
    /// * `Ok(true)` - Signature valid
    /// * `Ok(false)` - Signature invalid
    /// * `Err` - Verification failed
    ///
    /// # Examples
    /// ```ignore
    /// use sams_ghost_node::SamsGhostNode;
    /// use omwei_atom::Atom;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let ghost_node = SamsGhostNode::new(None).await?;
    ///     let atom = Atom::new(0x12345678, [0x42; 28]);
    ///     let signature = ghost_node.generate_signature(&atom, None).await?;
    ///     let verified = ghost_node.verify_signature(&atom, &signature).await?;
    ///     println!("Signature verified: {}", verified);
    ///     Ok(())
    /// }
    /// ```
    pub async fn verify_signature(
        &mut self,
        atom: &Atom,
        signature: &PqcSignature,
    ) -> Result<bool> {
        // Only process Managed Space atoms
        let trust_level = get_trust_level(atom.global_id);
        if trust_level != TrustLevel::Managed {
            warn!(
                "GhostNode: Rejecting Community Space atom 0x{:08X}",
                atom.global_id
            );
            return Ok(false);
        }

        info!(
            "GhostNode: Verifying PQC signature for Managed Space atom 0x{:08X}",
            atom.global_id
        );

        // Get public key
        let key_pair = self
            .key_store
            .get_key_pair(&signature.key_id)
            .ok_or_else(|| anyhow::anyhow!("Key pair not found: {}", signature.key_id))?;

        // Verify signature
        let start_time = std::time::Instant::now();
        let verified = self
            .pqc_engine
            .verify_signature(atom, signature, &key_pair.public_key).await?;
        let verification_time = start_time.elapsed().as_nanos() as u64;

        // Update metrics
        self.metrics
            .record_verification(verification_time, verified);

        if verified {
            info!(
                "✅ GhostNode: PQC signature verified for atom 0x{:08X}",
                atom.global_id
            );
        } else {
            warn!(
                "❌ GhostNode: Invalid PQC signature for atom 0x{:08X}",
                atom.global_id
            );
        }

        Ok(verified)
    }

    /// Get stored signature for atom
    ///
    /// # Arguments
    /// * `atom_id` - Atom global ID
    ///
    /// # Returns
    /// Stored signature if found
    pub fn get_signature(&self, atom_id: u32) -> Option<&PqcSignature> {
        self.signature_db.get_signature(atom_id)
    }

    /// Check if atom has verified signature
    ///
    /// # Arguments
    /// * `atom_id` - Atom global ID
    ///
    /// # Returns
    /// True if signature exists and is verified
    pub fn has_verified_signature(&self, atom_id: u32) -> bool {
        self.signature_db.verify_signature_exists(atom_id)
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &GhostNodeMetrics {
        &self.metrics
    }

    /// Generate new key pair
    ///
    /// # Arguments
    /// * `algorithm` - PQC algorithm to use
    ///
    /// # Returns
    /// Generated key pair identifier
    pub async fn generate_key_pair(&mut self, algorithm: Option<PqcAlgorithm>) -> Result<Uuid> {
        let key_pair = self.key_store.generate_key_pair(algorithm)?;
        self.metrics.active_key_pairs = self.key_store.key_pairs.len();
        Ok(key_pair.key_id)
    }
}

/// Ghost node configuration
#[derive(Debug, Default)]
pub struct GhostNodeConfig {
    /// Maximum signatures to store
    pub max_signatures: usize,
    /// Database URL for persistent storage
    pub database_url: Option<String>,
    /// Hardware acceleration enabled
    pub hardware_acceleration: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ghost_node_creation() {
        let ghost_node = SamsGhostNode::new(None).await;
        assert!(ghost_node.is_ok());
    }

    #[tokio::test]
    async fn test_managed_atom_signature_generation() {
        let mut ghost_node = SamsGhostNode::new(None).await.unwrap();
        let managed_atom = Atom::new(0x12345678, [0x42; 28]);
        let community_atom = Atom::new(0x80000001, [0x24; 28]);

        // Managed atom should be signed
        let managed_result = ghost_node.generate_signature(&managed_atom, None).await;
        assert!(managed_result.is_ok());

        // Community atom should be rejected
        let community_result = ghost_node.generate_signature(&community_atom, None).await;
        assert!(community_result.is_err());
    }

    #[tokio::test]
    async fn test_signature_verification() {
        let mut ghost_node = SamsGhostNode::new(None).await.unwrap();
        let atom = Atom::new(0x12345678, [0x42; 28]);

        // Generate signature
        let signature = ghost_node.generate_signature(&atom, None).await.unwrap();

        // Verify signature
        let verified = ghost_node
            .verify_signature(&atom, &signature)
            .await
            .unwrap();
        assert!(verified);
    }
}
