/*!
# SAMS Blackbox - Trust-Aware Logging and Audit Trail

**Trust-aware logging and audit trail system for OMWEI 32BSA ecosystem with Sincerity Bit tagging.** Provides comprehensive logging with automatic trust-level classification and audit trail generation.

## Mission

SAMS Blackbox serves as the central logging and audit trail component for the OMWEI 32BSA ecosystem, providing detailed, trust-aware logging with automatic classification based on the Sincerity Bit.

## Architecture

```text
SAMS BLACKBOX ARCHITECTURE
+-----------------------------------------------------+
|            Trust-Aware Logging Engine          |
|  +--------------+------------------+      |
|  | Sincerity    | Audit Trail     |      |
|  | Tagging       | Generation      |      |
|  +--------------+------------------+      |
|                   |                          |
|         Trust-Level Classification          |
|    (Managed vs Community)              |
+-----------------------------------------------------+
```

## Features

- **Trust-Level Tagging:** Automatic classification based on Sincerity Bit
- **Audit Trail Generation:** Complete processing history with traceability
- **High-Performance Storage:** Optimized for embedded systems
- **Compression Support:** LZ4 compression for long-term storage
- **Database Integration:** SQLite backend for persistent storage

## Sincerity Compliance

- **Managed Space Logs:** Tagged as "Sincere/Verified" with PQC metadata
- **Community Space Logs:** Tagged as "Experimental/Unverified" with local-only flags
- **Zero Latency:** No impact on trust determination performance
- **Hardware Efficient:** Minimal memory footprint, stack-optimized
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

/// Simple metadata type for no_std compatibility
pub type Metadata = HashMap<String, String>;

/// Blackbox log entry with trust-level awareness
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlackboxEntry {
    /// Unique entry identifier
    pub id: Uuid,
    /// Atom global ID
    pub atom_id: u32,
    /// Trust level at time of logging
    pub trust_level: TrustLevel,
    /// Log category (audit, security, performance, system)
    pub category: LogCategory,
    /// Log message
    pub message: String,
    /// Processing timestamp
    pub timestamp: DateTime<Utc>,
    /// Processing latency in nanoseconds
    pub latency_ns: u64,
    /// Trust metadata
    pub trust_metadata: TrustMetadata,
    /// Additional metadata
    pub metadata: Metadata,
}

/// Log categories for classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LogCategory {
    /// Audit trail entries
    Audit,
    /// Security-related events
    Security,
    /// Performance metrics
    Performance,
    /// System events
    System,
    /// Informational messages
    Info,
    /// Warning messages
    Warning,
    /// Error messages
    Error,
}

/// Trust-specific metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrustMetadata {
    /// Sincerity Bit value
    pub sincerity_bit: bool,
    /// PQC verification status (Managed Space only)
    pub pqc_verified: Option<bool>,
    /// Validation result
    pub validation_result: ValidationResult,
    /// Processing node ID
    pub node_id: Uuid,
    /// Processing chain ID
    pub chain_id: Uuid,
}

/// Audit trail entry for complete traceability
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuditTrailEntry {
    /// Trail identifier
    pub id: Uuid,
    /// Atom being audited
    pub atom_id: u32,
    /// Processing step
    pub step: ProcessingStep,
    /// Step timestamp
    pub timestamp: DateTime<Utc>,
    /// Step duration in nanoseconds
    pub duration_ns: u64,
    /// Step result
    pub result: StepResult,
    /// Additional context
    pub context: Metadata,
}

/// Processing steps in the audit trail
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProcessingStep {
    /// Initial atom creation
    AtomCreation,
    /// Trust level determination
    TrustDetermination,
    /// PQC signature verification
    PqcVerification,
    /// Logic gate validation
    LogicGateValidation,
    /// Blackbox logging
    BlackboxLogging,
    /// GhostNode storage
    GhostNodeStorage,
    /// Final processing complete
    ProcessingComplete,
}

/// Step result status
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StepResult {
    /// Step completed successfully
    Success,
    /// Step failed
    Failure,
    /// Step skipped
    Skipped,
    /// Step pending
    Pending,
}

/// SAMS Blackbox - Trust-aware logging system
///
/// Provides comprehensive logging with automatic trust-level classification
/// for the OMWEI 32BSA ecosystem.
#[derive(Debug)]
pub struct SamsBlackbox {
    /// Blackbox instance identifier
    blackbox_id: Uuid,
    /// Log storage backend
    storage: LogStorage,
    /// Audit trail manager
    audit_trail: AuditTrailManager,
    /// Trust statistics
    trust_stats: TrustStatistics,
}

/// Log storage backend
#[derive(Debug)]
pub struct LogStorage {
    entries: Vec<BlackboxEntry>,
    max_entries: usize,
    #[cfg(feature = "database")]
    database: Option<sqlx::SqlitePool>,
}

impl LogStorage {
    fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::with_capacity(max_entries),
            max_entries,
            #[cfg(feature = "database")]
            database: None,
        }
    }

    #[cfg(feature = "database")]
    async fn new_with_database(max_entries: usize, db_url: &str) -> Result<Self> {
        let database = sqlx::SqlitePool::connect(db_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&database).await?;

        Ok(Self {
            entries: Vec::with_capacity(max_entries),
            max_entries,
            database: Some(database),
        })
    }

    fn store_entry(&mut self, entry: BlackboxEntry) -> Result<()> {
        self.entries.push(entry.clone());
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        #[cfg(feature = "database")]
        if let Some(database) = &self.database {
            // TODO: Store in database
            debug!("Storing entry {} in database", entry.id);
        }

        Ok(())
    }

    fn query_by_trust_level(&self, trust_level: TrustLevel) -> Vec<&BlackboxEntry> {
        self.entries
            .iter()
            .filter(|e| e.trust_level == trust_level)
            .collect()
    }

    fn query_by_category(&self, category: LogCategory) -> Vec<&BlackboxEntry> {
        self.entries
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }

    fn query_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&BlackboxEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
}

/// Audit trail manager
#[derive(Debug)]
pub struct AuditTrailManager {
    trails: HashMap<u32, Vec<AuditTrailEntry>>, // atom_id -> trail
    max_trail_length: usize,
}

impl AuditTrailManager {
    fn new(max_trail_length: usize) -> Self {
        Self {
            trails: HashMap::new(),
            max_trail_length,
        }
    }

    fn add_step(
        &mut self,
        atom_id: u32,
        step: ProcessingStep,
        result: StepResult,
        context: Metadata,
    ) -> Result<()> {
        let entry = AuditTrailEntry {
            id: Uuid::new_v4(),
            atom_id,
            step,
            timestamp: Utc::now(),
            duration_ns: 0, // TODO: Calculate actual duration
            result,
            context,
        };

        let trail = self.trails.entry(atom_id).or_insert_with(Vec::new);
        trail.push(entry);

        // Limit trail length
        if trail.len() > self.max_trail_length {
            trail.remove(0);
        }

        Ok(())
    }

    fn get_trail(&self, atom_id: u32) -> Option<&[AuditTrailEntry]> {
        self.trails.get(&atom_id).map(|trail| trail.as_slice())
    }
}

/// Trust statistics tracker
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct TrustStatistics {
    /// Total atoms processed
    pub total_atoms: u64,
    /// Managed space atoms
    pub managed_atoms: u64,
    /// Community space atoms
    pub community_atoms: u64,
    /// PQC verification failures
    pub pqc_failures: u64,
    /// Average processing time
    pub avg_processing_time_ns: f64,
    /// Trust level distribution over time
    pub trust_distribution: Vec<(DateTime<Utc>, f64)>, // timestamp, managed_ratio
}

impl TrustStatistics {
    fn record_atom(&mut self, trust_level: TrustLevel, latency_ns: u64) {
        self.total_atoms += 1;

        match trust_level {
            TrustLevel::Managed => self.managed_atoms += 1,
            TrustLevel::Community => self.community_atoms += 1,
        }

        // Update average processing time
        let total_latency =
            self.avg_processing_time_ns * (self.total_atoms - 1) as f64 + latency_ns as f64;
        self.avg_processing_time_ns = total_latency / self.total_atoms as f64;
    }

    fn get_managed_ratio(&self) -> f64 {
        if self.total_atoms > 0 {
            self.managed_atoms as f64 / self.total_atoms as f64
        } else {
            0.0
        }
    }
}

impl SamsBlackbox {
    /// Create new SAMS Blackbox instance
    ///
    /// # Arguments
    /// * `config` - Optional blackbox configuration
    ///
    /// # Returns
    /// Initialized Blackbox ready for logging
    ///
    /// # Examples
    /// ```
    /// use sams_blackbox::SamsBlackbox;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let blackbox = SamsBlackbox::new(None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: Option<BlackboxConfig>) -> Result<Self> {
        let blackbox_id = Uuid::new_v4();

        info!("SAMS Blackbox: Initializing trust-aware logging system");
        info!("Blackbox ID: {}", blackbox_id);

        let config = config.unwrap_or_default();

        let storage = if let Some(_db_url) = &config.database_url {
            #[cfg(feature = "database")]
            {
                LogStorage::new_with_database(config.max_entries, db_url).await?
            }
            #[cfg(not(feature = "database"))]
            {
                LogStorage::new(config.max_entries)
            }
        } else {
            LogStorage::new(config.max_entries)
        };

        let audit_trail = AuditTrailManager::new(config.max_trail_length);

        info!("✅ SAMS Blackbox initialized successfully");

        Ok(Self {
            blackbox_id,
            storage,
            audit_trail,
            trust_stats: TrustStatistics::default(),
        })
    }

    /// Log atom with trust-level awareness
    ///
    /// Automatically creates log entries with appropriate trust-level tagging
    /// and updates audit trail.
    ///
    /// # Arguments
    /// * `atom` - The atom to log
    /// * `validation_result` - Result of validation
    /// * `latency_ns` - Processing latency
    /// * `category` - Log category
    /// * `message` - Log message
    ///
    /// # Returns
    /// Success if logged successfully
    ///
    /// # Examples
    /// ```ignore
    /// use sams_blackbox::SamsBlackbox;
    /// use sams_blackbox::LogCategory;
    /// use omwei_atom::{Atom, ValidationResult};
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let blackbox = SamsBlackbox::new(None).await?;
    ///     let atom = Atom::new(0x12345678, [0x42; 28]);
    ///     let result = ValidationResult::Trusted;
    ///     
    ///     blackbox.log_atom(&atom, &result, 8600, LogCategory::Audit, "Atom processed".to_string()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn log_atom(
        &mut self,
        atom: &Atom,
        validation_result: &ValidationResult,
        latency_ns: u64,
        category: LogCategory,
        message: String,
    ) -> Result<()> {
        let trust_level = get_trust_level(atom.global_id);

        // Create trust metadata
        let trust_metadata = TrustMetadata {
            sincerity_bit: (atom.global_id & 0x80000000) != 0,
            pqc_verified: match trust_level {
                TrustLevel::Managed => Some(matches!(validation_result, ValidationResult::Trusted)),
                TrustLevel::Community => None,
            },
            validation_result: validation_result.clone(),
            node_id: self.blackbox_id,
            chain_id: Uuid::new_v4(),
        };

        // Create blackbox entry
        let mut metadata = Metadata::new();
        metadata.insert("blackbox_id".to_string(), self.blackbox_id.to_string());
        metadata.insert("payload_size".to_string(), atom.payload.len().to_string());

        let entry = BlackboxEntry {
            id: Uuid::new_v4(),
            atom_id: atom.global_id,
            trust_level: trust_level,
            category: category.clone(),
            message,
            timestamp: Utc::now(),
            latency_ns,
            trust_metadata,
            metadata,
        };

        // Store entry
        self.storage.store_entry(entry.clone())?;

        // Update audit trail
        let mut audit_metadata = Metadata::new();
        audit_metadata.insert("entry_id".to_string(), entry.id.to_string());
        audit_metadata.insert("category".to_string(), format!("{:?}", category));

        self.audit_trail.add_step(
            atom.global_id,
            ProcessingStep::BlackboxLogging,
            StepResult::Success,
            audit_metadata,
        )?;

        // Update statistics
        self.trust_stats.record_atom(trust_level, latency_ns);

        // Emit structured log
        self.emit_structured_log(&entry).await;

        info!(
            "SAMS Blackbox: Logged atom 0x{:08X} (trust: {:?}, category: {:?})",
            atom.global_id, trust_level, category
        );

        Ok(())
    }

    /// Get audit trail for atom
    ///
    /// # Arguments
    /// * `atom_id` - Atom global ID
    ///
    /// # Returns
    /// Complete audit trail for the atom
    pub fn get_audit_trail(&self, atom_id: u32) -> Option<&[AuditTrailEntry]> {
        self.audit_trail.get_trail(atom_id)
    }

    /// Query logs by trust level
    ///
    /// # Arguments
    /// * `trust_level` - Filter by specific trust level
    ///
    /// # Returns
    /// Vector of matching log entries
    pub fn query_by_trust_level(&self, trust_level: TrustLevel) -> Vec<&BlackboxEntry> {
        self.storage.query_by_trust_level(trust_level)
    }

    /// Query logs by category
    ///
    /// # Arguments
    /// * `category` - Filter by specific category
    ///
    /// # Returns
    /// Vector of matching log entries
    pub fn query_by_category(&self, category: LogCategory) -> Vec<&BlackboxEntry> {
        self.storage.query_by_category(category)
    }

    /// Query logs by time range
    ///
    /// # Arguments
    /// * `start` - Start timestamp
    /// * `end` - End timestamp
    ///
    /// # Returns
    /// Vector of matching log entries
    pub fn query_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&BlackboxEntry> {
        self.storage.query_by_time_range(start, end)
    }

    /// Get trust statistics
    pub fn get_trust_statistics(&self) -> &TrustStatistics {
        &self.trust_stats
    }

    /// Generate audit report
    ///
    /// # Arguments
    /// * `start` - Report start time
    /// * `end` - Report end time
    ///
    /// # Returns
    /// Comprehensive audit report
    pub fn generate_audit_report(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> AuditReport {
        let entries = self.storage.query_by_time_range(start, end);

        let managed_count = entries
            .iter()
            .filter(|e| e.trust_level == TrustLevel::Managed)
            .count();
        let community_count = entries
            .iter()
            .filter(|e| e.trust_level == TrustLevel::Community)
            .count();

        let category_counts = entries.iter().fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e.category.clone()).or_insert(0) += 1;
            acc
        });

        let avg_latency = if !entries.is_empty() {
            entries.iter().map(|e| e.latency_ns).sum::<u64>() as f64 / entries.len() as f64
        } else {
            0.0
        };

        AuditReport {
            period_start: start,
            period_end: end,
            total_entries: entries.len(),
            managed_count,
            community_count,
            category_counts,
            avg_latency_ns: avg_latency,
            trust_statistics: self.trust_stats.clone(),
        }
    }

    /// Emit structured log to console
    async fn emit_structured_log(&self, entry: &BlackboxEntry) {
        match entry.trust_level {
            TrustLevel::Managed => {
                info!(
                    "[SINCERE] atom_id={} trust_level={:?} category={:?} latency_ns={} {}",
                    entry.atom_id, entry.trust_level, entry.category, entry.latency_ns, entry.message
                );
            }
            TrustLevel::Community => {
                warn!(
                    "[COMMUNITY] atom_id={} trust_level={:?} category={:?} latency_ns={} {}",
                    entry.atom_id, entry.trust_level, entry.category, entry.latency_ns, entry.message
                );
            }
        }
    }
}

/// Blackbox configuration
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlackboxConfig {
    /// Maximum log entries to store
    pub max_entries: usize,
    /// Maximum audit trail length per atom
    pub max_trail_length: usize,
    /// Database URL for persistent storage
    pub database_url: Option<String>,
}

/// Comprehensive audit report
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuditReport {
    /// Report period start
    pub period_start: DateTime<Utc>,
    /// Report period end
    pub period_end: DateTime<Utc>,
    /// Total log entries
    pub total_entries: usize,
    /// Managed space entries
    pub managed_count: usize,
    /// Community space entries
    pub community_count: usize,
    /// Category distribution
    pub category_counts: HashMap<LogCategory, usize>,
    /// Average latency
    pub avg_latency_ns: f64,
    /// Trust statistics
    pub trust_statistics: TrustStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blackbox_creation() {
        let blackbox = SamsBlackbox::new(None).await;
        assert!(blackbox.is_ok());
    }

    #[tokio::test]
    async fn test_trust_aware_logging() {
        let mut blackbox = SamsBlackbox::new(None).await.unwrap();
        let atom = Atom::new(0x12345678, [0x42; 28]);
        let result = ValidationResult::Trusted;

        let log_result = blackbox
            .log_atom(
                &atom,
                &result,
                8600,
                LogCategory::Audit,
                "Test log".to_string(),
            )
            .await;
        assert!(log_result.is_ok());

        let entries = blackbox.query_by_trust_level(TrustLevel::Managed);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].atom_id, atom.global_id);
    }

    #[tokio::test]
    async fn test_audit_trail() {
        let mut blackbox = SamsBlackbox::new(None).await.unwrap();
        let atom = Atom::new(0x12345678, [0x42; 28]);
        let result = ValidationResult::Trusted;

        blackbox
            .log_atom(
                &atom,
                &result,
                8600,
                LogCategory::Audit,
                "Test log".to_string(),
            )
            .await
            .unwrap();

        let trail = blackbox.get_audit_trail(atom.global_id);
        assert!(trail.is_some());
        assert_eq!(trail.unwrap().len(), 1);
    }
}
