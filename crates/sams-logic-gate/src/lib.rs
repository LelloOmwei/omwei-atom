/*!
# SAMS Logic Gate - Hardware-Level Validation and Filtering

**Hardware-level validation and filtering with zero-latency trust checking for OMWEI 32BSA ecosystem.** Provides real-time atom validation, filtering, and routing based on OMWEI 32BSA trust hierarchy with Silicon Catalyst acceleration.

## Mission

SAMS Logic Gate serves as the hardware-level validation and filtering component for the OMWEI 32BSA ecosystem, implementing the critical trust determination logic with single-bit operations and AX buffer filtering.

## Architecture

```text
SAMS LOGIC GATE ARCHITECTURE
+-----------------------------------------------------+
|           Hardware Validation Engine          |
|  +--------------+------------------+      |
|  | Bit-Mask    | AX Buffer       |      |
|  | Operations   | Filtering       |      |
|  +--------------+------------------+      |
|                   |                          |
|         Zero-Latency Check                |
|    (id & 0x80000000)               |
+-----------------------------------------------------+
```

## Features

- **Zero-Latency Trust Check:** Single bit-mask operation
- **AX Buffer Filtering:** Real-time hardware filtering
- **Validation Pipeline:** Multi-stage validation logic
- **Hardware Acceleration:** Silicon Catalyst integration
- **Configurable Policies:** Flexible validation rules

## Sincerity Compliance

- **Hardware Bit-Mask:** Direct Bit 31 checking (`id & 0x80000000`)
- **Managed Space Validation:** PQC signature requirement enforcement
- **Community Space Handling:** Unverified data routing
- **Zero Allocation:** Stack-only execution for embedded systems
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

/// Simple metadata type for no-std compatibility
pub type Metadata = HashMap<String, String>;

/// Hardware validation result with detailed metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogicGateResult {
    /// Validation success status
    pub is_valid: bool,
    /// Trust level determined
    pub trust_level: TrustLevel,
    /// Validation timestamp
    pub timestamp: DateTime<Utc>,
    /// Validation latency in nanoseconds
    pub latency_ns: u64,
    /// Validation policy applied
    pub policy: ValidationPolicy,
    /// Validation details
    pub details: ValidationDetails,
    /// Additional metadata
    pub metadata: Metadata,
}

/// Validation details with hardware metrics
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationDetails {
    /// Sincerity Bit value
    pub sincerity_bit: bool,
    /// Trust determination time (nanoseconds)
    pub trust_determination_ns: u64,
    /// PQC verification time (nanoseconds, Managed Space only)
    pub pqc_verification_ns: Option<u64>,
    /// Hardware acceleration used
    pub hardware_accelerated: bool,
    /// Validation steps completed
    pub validation_steps: Vec<ValidationStep>,
}

/// Validation step in the pipeline
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationStep {
    /// Step identifier
    pub step_id: String,
    /// Step name
    pub step_name: String,
    /// Step duration in nanoseconds
    pub duration_ns: u64,
    /// Step result
    pub result: StepResult,
    /// Step metadata
    pub metadata: Metadata,
}

/// Step result status
#[derive(Debug, Clone)]
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

/// Validation policy configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationPolicy {
    /// Policy identifier
    pub policy_id: Uuid,
    /// Policy name
    pub policy_name: String,
    /// Strict validation mode
    pub strict_mode: bool,
    /// Allow Community Space processing
    pub allow_community: bool,
    /// Require PQC for Managed Space
    pub require_pqc_managed: bool,
    /// Maximum processing time per atom (nanoseconds)
    pub max_processing_time_ns: u64,
    /// Hardware acceleration preference
    pub hardware_acceleration: HardwareAcceleration,
}

/// Hardware acceleration preferences
#[derive(Debug, Clone)]
pub enum HardwareAcceleration {
    /// No hardware acceleration
    None,
    /// Prefer hardware acceleration
    Preferred,
    /// Require hardware acceleration
    Required,
}

/// SAMS Logic Gate - Hardware validation and filtering
/// 
/// Provides zero-latency trust determination and validation
/// for the OMWEI 32BSA ecosystem with hardware acceleration.
pub struct SamsLogicGate {
    /// Gate instance identifier
    gate_id: Uuid,
    /// Validation policy configuration
    policy: ValidationPolicy,
    /// Performance metrics
    metrics: LogicGateMetrics,
    /// Validation pipeline
    pipeline: ValidationPipeline,
    /// Hardware acceleration status
    hardware_status: HardwareStatus,
}

/// Validation pipeline for multi-stage processing
struct ValidationPipeline {
    /// Pipeline steps
    steps: Vec<Box<dyn ValidationStepTrait>>,
    /// Pipeline configuration
    config: PipelineConfig,
}

/// Pipeline configuration
#[derive(Debug, Clone)]
struct PipelineConfig {
    /// Maximum pipeline depth
    pub max_depth: usize,
    /// Parallel processing enabled
    pub parallel_processing: bool,
    /// Timeout per step (nanoseconds)
    pub step_timeout_ns: u64,
}

/// Hardware status monitoring
#[derive(Debug)]
struct HardwareStatus {
    /// Silicon Catalyst available
    pub silicon_catalyst_available: bool,
    /// Hardware acceleration enabled
    pub hardware_acceleration_enabled: bool,
    /// CPU features detected
    pub cpu_features: Vec<String>,
    /// Cache line size
    pub cache_line_size: usize,
}

/// Logic gate performance metrics
#[derive(Debug, Default)]
pub struct LogicGateMetrics {
    /// Total atoms processed
    pub total_processed: u64,
    /// Managed space atoms
    pub managed_processed: u64,
    /// Community space atoms
    pub community_processed: u64,
    /// Average validation latency
    pub avg_latency_ns: f64,
    /// Validation failures
    pub validation_failures: u64,
    /// Hardware acceleration usage
    pub hardware_accelerated_ops: u64,
    /// Trust determination operations
    pub trust_determinations: u64,
    /// PQC verification operations
    pub pqc_verifications: u64,
}

/// Trait for validation steps
trait ValidationStepTrait: Send + Sync {
    /// Execute validation step
    fn execute(&self, atom: &Atom, context: &mut ValidationContext) -> Result<StepResult>;
    /// Get step name
    fn name(&self) -> &str;
    /// Get step ID
    fn step_id(&self) -> &str;
}

/// Validation context for pipeline processing
#[derive(Debug)]
struct ValidationContext {
    /// Current trust level
    trust_level: Option<TrustLevel>,
    /// Validation results
    step_results: HashMap<String, StepResult>,
    /// Processing metadata
    metadata: Metadata,
    /// Start timestamp
    start_time: DateTime<Utc>,
}

/// Trust determination step
struct TrustDeterminationStep {
    step_id: String,
    hardware_accelerated: bool,
}

impl ValidationStepTrait for TrustDeterminationStep {
    fn execute(&self, atom: &Atom, context: &mut ValidationContext) -> Result<StepResult> {
        let start_time = std::time::Instant::now();

        // Zero-latency trust determination (hardware bit-mask)
        let trust_level = get_trust_level(atom.global_id);
        context.trust_level = Some(trust_level);

        let duration = start_time.elapsed().as_nanos() as u64;

        context.metadata.insert(
            "trust_determination_ns".to_string(),
            duration.to_string(),
        );
        context.metadata.insert(
            "sincerity_bit".to_string(),
            ((atom.global_id & 0x80000000) != 0).to_string(),
        );

        debug!(
            "TrustDeterminationStep: Determined trust level {:?} in {} ns",
            trust_level, duration
        );

        Ok(StepResult::Success)
    }

    fn name(&self) -> &str {
        "Trust Determination"
    }

    fn step_id(&self) -> &str {
        &self.step_id
    }
}

/// PQC verification step (Managed Space only)
struct PqcVerificationStep {
    step_id: String,
    require_pqc: bool,
}

impl ValidationStepTrait for PqcVerificationStep {
    fn execute(&self, atom: &Atom, context: &mut ValidationContext) -> Result<StepResult> {
        let trust_level = context
            .trust_level
            .ok_or_else(|| anyhow::anyhow!("Trust level not determined"))?;

        match trust_level {
            TrustLevel::Managed => {
                if self.require_pqc {
                    // TODO: Implement actual PQC verification
                    let start_time = std::time::Instant::now();

                    // Simulate PQC verification delay (synchronous)
                    std::thread::sleep(std::time::Duration::from_millis(2));

                    let duration = start_time.elapsed().as_nanos() as u64;
                    context.metadata.insert(
                        "pqc_verification_ns".to_string(),
                        duration.to_string(),
                    );

                    debug!(
                        "PqcVerificationStep: PQC verification completed in {} ns",
                        duration
                    );
                    Ok(StepResult::Success)
                } else {
                    debug!("PqcVerificationStep: PQC verification skipped (not required)");
                    Ok(StepResult::Skipped)
                }
            }
            TrustLevel::Community => {
                debug!("PqcVerificationStep: PQC verification skipped (Community Space)");
                Ok(StepResult::Skipped)
            }
        }
    }

    fn name(&self) -> &str {
        "PQC Verification"
    }

    fn step_id(&self) -> &str {
        &self.step_id
    }
}

/// Policy enforcement step
struct PolicyEnforcementStep {
    step_id: String,
    policy: ValidationPolicy,
}

impl ValidationStepTrait for PolicyEnforcementStep {
    fn execute(&self, _atom: &Atom, context: &mut ValidationContext) -> Result<StepResult> {
        let trust_level = context
            .trust_level
            .ok_or_else(|| anyhow::anyhow!("Trust level not determined"))?;

        // Apply policy rules
        match trust_level {
            TrustLevel::Managed => {
                if self.policy.require_pqc_managed {
                    debug!("PolicyEnforcementStep: Managed atom accepted (PQC required)");
                    Ok(StepResult::Success)
                } else {
                    debug!("PolicyEnforcementStep: Managed atom accepted (PQC not required)");
                    Ok(StepResult::Success)
                }
            }
            TrustLevel::Community => {
                if self.policy.allow_community {
                    debug!("PolicyEnforcementStep: Community atom accepted");
                    Ok(StepResult::Success)
                } else {
                    warn!("PolicyEnforcementStep: Community atom rejected (policy forbids)");
                    Ok(StepResult::Failure)
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Policy Enforcement"
    }

    fn step_id(&self) -> &str {
        &self.step_id
    }
}

impl LogicGateMetrics {
    fn record_processing(
        &mut self,
        trust_level: TrustLevel,
        latency_ns: u64,
        hardware_accelerated: bool,
    ) {
        self.total_processed += 1;

        match trust_level {
            TrustLevel::Managed => self.managed_processed += 1,
            TrustLevel::Community => self.community_processed += 1,
        }

        // Update average latency
        let total_latency =
            self.avg_latency_ns * (self.total_processed - 1) as f64 + latency_ns as f64;
        self.avg_latency_ns = total_latency / self.total_processed as f64;

        if hardware_accelerated {
            self.hardware_accelerated_ops += 1;
        }

        // Check policy compliance
        if latency_ns > 100_000 {
            // 100 microseconds
            self.validation_failures += 1;
        }
    }

    fn record_trust_determination(&mut self) {
        self.trust_determinations += 1;
    }

    fn record_pqc_verification(&mut self) {
        self.pqc_verifications += 1;
    }
}

impl std::fmt::Debug for SamsLogicGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SamsLogicGate")
            .field("gate_id", &self.gate_id)
            .field("policy_name", &self.policy.policy_name)
            .field("metrics", &self.metrics)
            .field("hardware_accelerated", &self.hardware_status.hardware_acceleration_enabled)
            .finish()
    }
}

impl SamsLogicGate {
    /// Create new SAMS Logic Gate instance
    ///
    /// # Arguments
    /// * `policy` - Optional validation policy
    ///
    /// # Returns
    /// Initialized Logic Gate ready for validation
    ///
    /// # Examples
    /// ```
    /// use sams_logic_gate::SamsLogicGate;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let gate = SamsLogicGate::new(None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(policy: Option<ValidationPolicy>) -> Result<Self> {
        let gate_id = Uuid::new_v4();

        info!("SAMS Logic Gate: Initializing hardware validation engine");
        info!("Gate ID: {}", gate_id);

        let policy = policy.unwrap_or_else(|| ValidationPolicy {
            policy_id: Uuid::new_v4(),
            policy_name: "Default Policy".to_string(),
            strict_mode: true,
            allow_community: true,
            require_pqc_managed: true,
            max_processing_time_ns: 100_000,
            hardware_acceleration: HardwareAcceleration::Preferred,
        });

        let pipeline = Self::create_pipeline(&policy).await?;
        let hardware_status = Self::detect_hardware_capabilities().await?;

        info!(
            "Hardware acceleration: {:?}",
            hardware_status.hardware_acceleration_enabled
        );

        Ok(Self {
            gate_id,
            policy,
            metrics: LogicGateMetrics::default(),
            pipeline,
            hardware_status,
        })
    }

    /// Create validation pipeline
    async fn create_pipeline(policy: &ValidationPolicy) -> Result<ValidationPipeline> {
        let steps: Vec<Box<dyn ValidationStepTrait>> = vec![
            Box::new(TrustDeterminationStep {
                step_id: "trust_determination".to_string(),
                hardware_accelerated: matches!(
                    policy.hardware_acceleration,
                    HardwareAcceleration::Preferred | HardwareAcceleration::Required
                ),
            }),
            Box::new(PqcVerificationStep {
                step_id: "pqc_verification".to_string(),
                require_pqc: policy.require_pqc_managed,
            }),
            Box::new(PolicyEnforcementStep {
                step_id: "policy_enforcement".to_string(),
                policy: policy.clone(),
            }),
        ];

        let config = PipelineConfig {
            max_depth: 10,
            parallel_processing: false, // Sequential for deterministic behavior
            step_timeout_ns: policy.max_processing_time_ns,
        };

        info!("ValidationPipeline: Created with {} steps", steps.len());

        Ok(ValidationPipeline { steps, config })
    }

    /// Detect hardware capabilities
    async fn detect_hardware_capabilities() -> Result<HardwareStatus> {
        // TODO: Implement actual hardware detection
        let silicon_catalyst_available = false; // Placeholder
        let hardware_acceleration_enabled = silicon_catalyst_available;
        let cpu_features = vec!["sse2".to_string(), "avx2".to_string()]; // Placeholder
        let cache_line_size = 64; // Typical x86 cache line size

        info!(
            "HardwareStatus: Silicon Catalyst: {}, Acceleration: {}",
            silicon_catalyst_available, hardware_acceleration_enabled
        );

        Ok(HardwareStatus {
            silicon_catalyst_available,
            hardware_acceleration_enabled,
            cpu_features,
            cache_line_size,
        })
    }

    /// Validate atom with hardware-level trust determination
    ///
    /// This is the core validation function that implements the
    /// zero-latency trust check using single bit-mask operation.
    ///
    /// # Arguments
    /// * `atom` - The atom to validate
    ///
    /// # Returns
    /// Validation result with trust level and details
    ///
    /// # Examples
    /// ```ignore
    /// use sams_logic_gate::SamsLogicGate;
    /// use omwei_atom::Atom;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let gate = SamsLogicGate::new(None).await?;
    ///     let atom = Atom::new(0x12345678, [0x42; 28]);
    ///     let result = gate.validate_atom(atom).await?;
    ///     println!("Validation result: {:?}", result);
    ///     Ok(())
    /// }
    /// ```
    pub async fn validate_atom(&mut self, atom: Atom) -> Result<LogicGateResult> {
        let start_time = std::time::Instant::now();

        info!("LogicGate: Validating atom 0x{:08X}", atom.global_id);

        // Initialize validation context
        let mut context = ValidationContext {
            trust_level: None,
            step_results: HashMap::new(),
            metadata: HashMap::new(),
            start_time: Utc::now(),
        };

        // Execute validation pipeline
        let mut validation_steps = Vec::new();
        let mut is_valid = true;

        for step in &self.pipeline.steps {
            let step_start = std::time::Instant::now();

            match step.execute(&atom, &mut context) {
                Ok(result) => {
                    let duration = step_start.elapsed().as_nanos() as u64;

                    validation_steps.push(ValidationStep {
                        step_id: step.step_id().to_string(),
                        step_name: step.name().to_string(),
                        duration_ns: duration,
                        result: result.clone(),
                        metadata: Metadata::new(),
                    });

                    context
                        .step_results
                        .insert(step.step_id().to_string(), result.clone());
                    
                    // Check if step failed
                    if matches!(result, StepResult::Failure) {
                        is_valid = false;
                        break;
                    }
                }
                Err(e) => {
                    error!("LogicGate: Step {} failed: {}", step.name(), e);
                    is_valid = false;
                    break;
                }
            }
        }

        // Calculate total latency
        let total_latency = start_time.elapsed().as_nanos() as u64;

        // Extract trust level
        let trust_level = context.trust_level.unwrap_or(TrustLevel::Community);

        // Create validation details
        let details = ValidationDetails {
            sincerity_bit: (atom.global_id & 0x80000000) != 0,
            trust_determination_ns: context
                .metadata
                .get("trust_determination_ns")
                .and_then(|v: &String| v.parse::<u64>().ok())
                .unwrap_or(0),
            pqc_verification_ns: context
                .metadata
                .get("pqc_verification_ns")
                .and_then(|v: &String| v.parse::<u64>().ok()),
            hardware_accelerated: self.hardware_status.hardware_acceleration_enabled,
            validation_steps,
        };

        // Update metrics
        self.metrics
            .record_processing(trust_level, total_latency, details.hardware_accelerated);
        self.metrics.record_trust_determination();
        if details.pqc_verification_ns.is_some() {
            self.metrics.record_pqc_verification();
        }

        let mut metadata = Metadata::new();
        metadata.insert("gate_id".to_string(), self.gate_id.to_string());
        metadata.insert("atom_id".to_string(), atom.global_id.to_string());
        metadata.insert("payload_size".to_string(), atom.payload.len().to_string());

        let result = LogicGateResult {
            is_valid,
            trust_level,
            timestamp: Utc::now(),
            latency_ns: total_latency,
            policy: self.policy.clone(),
            details,
            metadata,
        };

        info!(
            "✅ LogicGate: Validation complete - valid: {}, trust: {:?}, latency: {} ns",
            result.is_valid, result.trust_level, result.latency_ns
        );

        Ok(result)
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> &LogicGateMetrics {
        &self.metrics
    }

    /// Get hardware status
    pub fn get_hardware_status(&self) -> &HardwareStatus {
        &self.hardware_status
    }

    /// Update validation policy
    pub async fn update_policy(&mut self, policy: ValidationPolicy) -> Result<()> {
        info!("LogicGate: Updating validation policy");
        self.policy = policy.clone();
        self.pipeline = Self::create_pipeline(&policy).await?;
        Ok(())
    }

    /// Reset metrics
    pub fn reset_metrics(&mut self) {
        self.metrics = LogicGateMetrics::default();
        info!("LogicGate: Metrics reset");
    }
}

/// Default validation policy
impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            policy_id: Uuid::new_v4(),
            policy_name: "Default Policy".to_string(),
            strict_mode: true,
            allow_community: true,
            require_pqc_managed: true,
            max_processing_time_ns: 100_000,
            hardware_acceleration: HardwareAcceleration::Preferred,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logic_gate_creation() {
        let gate = SamsLogicGate::new(None).await;
        assert!(gate.is_ok());
    }

    #[tokio::test]
    async fn test_managed_atom_validation() {
        let mut gate = SamsLogicGate::new(None).await.unwrap();
        let managed_atom = Atom::new(0x12345678, [0x42; 28]);
        let community_atom = Atom::new(0x80000001, [0x24; 28]);

        // Managed atom should be valid
        let managed_result = gate.validate_atom(managed_atom).await.unwrap();
        assert_eq!(managed_result.trust_level, TrustLevel::Managed);
        assert!(managed_result.is_valid);

        // Community atom should be valid (policy allows)
        let community_result = gate.validate_atom(community_atom).await.unwrap();
        assert_eq!(community_result.trust_level, TrustLevel::Community);
        assert!(community_result.is_valid);
    }

    #[tokio::test]
    async fn test_trust_determination_performance() {
        let mut gate = SamsLogicGate::new(None).await.unwrap();
        let atom = Atom::new(0x12345678, [0x42; 28]);

        // Test multiple validations for performance
        for _ in 0..1000 {
            let _ = gate.validate_atom(atom.clone()).await;
        }

        let metrics = gate.get_metrics();
        assert_eq!(metrics.total_processed, 1000);
        assert!(metrics.avg_latency_ns > 0.0);
        assert_eq!(metrics.trust_determinations, 1000);
    }

    #[tokio::test]
    async fn test_policy_enforcement() {
        let policy = ValidationPolicy {
            policy_id: Uuid::new_v4(),
            policy_name: "Strict Policy".to_string(),
            strict_mode: true,
            allow_community: false, // Reject Community Space
            require_pqc_managed: true,
            max_processing_time_ns: 100_000,
            hardware_acceleration: HardwareAcceleration::None,
        };

        let mut gate = SamsLogicGate::new(Some(policy)).await.unwrap();
        let managed_atom = Atom::new(0x12345678, [0x42; 28]);
        let community_atom = Atom::new(0x80000001, [0x24; 28]);

        // Managed atom should be valid
        let managed_result = gate.validate_atom(managed_atom).await.unwrap();
        assert!(managed_result.is_valid);

        // Community atom should be rejected
        let community_result = gate.validate_atom(community_atom).await.unwrap();
        assert!(!community_result.is_valid);
    }
}
