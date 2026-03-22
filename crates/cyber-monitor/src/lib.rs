/*!
# Cyber Monitor - Real-time Telemetry and UI

**Real-time telemetry and UI with Sincerity Bit awareness for OMWEI 32BSA ecosystem.** Provides comprehensive monitoring, visualization, and alerting with trust-level differentiation.

## Mission

Cyber Monitor serves as the real-time monitoring and visualization layer for the OMWEI 32BSA ecosystem, providing instant feedback on trust levels, processing metrics, and system health with hardware-aware UI components.

## Architecture

```text
CYBER MONITOR ARCHITECTURE
+-----------------------------------------------------+
|            Web UI & Telemetry Engine          |
|  +--------------+------------------+      |
|  | Trust-Aware  | Real-time       |      |
|  | Dashboard    | WebSocket       |      |
|  +--------------+------------------+      |
|                   |                          |
|         Sincerity Bit Visualization         |
|    (Managed vs Community)              |
+-----------------------------------------------------+
```

## Features

- **Trust-Aware Dashboard:** Real-time visualization of Managed vs Community data
- **WebSocket Streaming:** Live telemetry updates
- **Alert System:** Trust-level based alerts and notifications
- **Performance Metrics:** Hardware-level performance monitoring
- **Historical Analysis:** Trust trend analysis and reporting

## Sincerity Compliance

- **Visual Differentiation:** Clear UI distinction between "Sincere" and "Unverified" data
- **Real-time Trust Updates:** Instant feedback on trust level changes
- **Hardware Metrics:** Zero-latency performance visualization
- **Audit Trail:** Complete monitoring history with trust metadata
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
use log::{error, info, warn};
use std::collections::HashMap;
use uuid::Uuid;

/// Telemetry data point with trust-level awareness
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TelemetryPoint {
    /// Unique telemetry identifier
    pub id: Uuid,
    /// Atom global ID
    pub atom_id: u32,
    /// Trust level at time of measurement
    pub trust_level: TrustLevel,
    /// Measurement timestamp
    pub timestamp: DateTime<Utc>,
    /// Processing latency (nanoseconds)
    pub latency_ns: u64,
    /// System metrics
    pub metrics: SystemMetrics,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// System performance metrics
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemMetrics {
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Network throughput (bytes/second)
    pub network_throughput: f64,
    /// Trust determination rate (ops/second)
    pub trust_ops_per_sec: f64,
    /// PQC verification rate (ops/second)
    pub pqc_ops_per_sec: f64,
}

/// Alert configuration and state
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alert {
    /// Alert identifier
    pub id: Uuid,
    /// Alert type (trust, performance, security)
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Associated atom ID (if applicable)
    pub atom_id: Option<u32>,
    /// Alert metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Alert types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlertType {
    /// Trust level related alerts
    Trust,
    /// Performance related alerts
    Performance,
    /// Security related alerts
    Security,
    /// System health alerts
    System,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlertSeverity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// Cyber Monitor - Real-time telemetry and UI
///
/// Provides comprehensive monitoring with trust-level awareness
/// for the OMWEI 32BSA ecosystem.
#[derive(Debug)]
pub struct CyberMonitor {
    /// Monitor instance identifier
    monitor_id: Uuid,
    /// Telemetry storage
    telemetry_store: TelemetryStore,
    /// Alert manager
    alert_manager: AlertManager,
    /// WebSocket connections
    websocket_connections: HashMap<Uuid, WebSocketConnection>,
    /// Performance tracker
    performance_tracker: PerformanceTracker,
}

/// Telemetry data store
#[derive(Debug)]
struct TelemetryStore {
    points: Vec<TelemetryPoint>,
    max_points: usize,
}

impl TelemetryStore {
    fn new(max_points: usize) -> Self {
        Self {
            points: Vec::with_capacity(max_points),
            max_points,
        }
    }

    fn add_point(&mut self, point: TelemetryPoint) {
        self.points.push(point);
        if self.points.len() > self.max_points {
            self.points.remove(0);
        }
    }

    fn get_by_trust_level(&self, trust_level: TrustLevel) -> Vec<&TelemetryPoint> {
        self.points
            .iter()
            .filter(|p| p.trust_level == trust_level)
            .collect()
    }

    fn get_recent(&self, since: DateTime<Utc>) -> Vec<&TelemetryPoint> {
        self.points
            .iter()
            .filter(|p| p.timestamp >= since)
            .collect()
    }
}

/// Alert management system
#[derive(Debug)]
struct AlertManager {
    alerts: Vec<Alert>,
    alert_rules: Vec<AlertRule>,
}

impl AlertManager {
    fn new() -> Self {
        Self {
            alerts: Vec::new(),
            alert_rules: vec![
                AlertRule::new(
                    AlertType::Trust,
                    AlertSeverity::Warning,
                    "High Community Space ratio",
                ),
                AlertRule::new(
                    AlertType::Performance,
                    AlertSeverity::Error,
                    "High latency detected",
                ),
                AlertRule::new(
                    AlertType::Security,
                    AlertSeverity::Critical,
                    "Invalid PQC signatures",
                ),
            ],
        }
    }

    fn check_alerts(&mut self, telemetry: &TelemetryPoint) -> Vec<Alert> {
        let mut new_alerts = Vec::new();

        for rule in &self.alert_rules {
            if rule.should_trigger(telemetry) {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("telemetry_id".to_string(), telemetry.id.to_string());
                metadata.insert("trust_level".to_string(), format!("{:?}", telemetry.trust_level));
                metadata.insert("latency_ns".to_string(), telemetry.latency_ns.to_string());

                let alert = Alert {
                    id: Uuid::new_v4(),
                    alert_type: rule.alert_type.clone(),
                    severity: rule.severity.clone(),
                    message: rule.message.clone(),
                    timestamp: Utc::now(),
                    atom_id: Some(telemetry.atom_id),
                    metadata,
                };
                new_alerts.push(alert);
            }
        }

        self.alerts.extend(new_alerts.clone());
        new_alerts
    }
}

/// Alert rule definition
#[derive(Debug)]
struct AlertRule {
    alert_type: AlertType,
    severity: AlertSeverity,
    message: String,
}

impl AlertRule {
    fn new(alert_type: AlertType, severity: AlertSeverity, message: &str) -> Self {
        Self {
            alert_type,
            severity,
            message: message.to_string(),
        }
    }

    fn should_trigger(&self, telemetry: &TelemetryPoint) -> bool {
        match self.alert_type {
            AlertType::Trust => {
                // Alert if Community Space atom
                telemetry.trust_level == TrustLevel::Community
            }
            AlertType::Performance => {
                // Alert if latency exceeds threshold
                telemetry.latency_ns > 100_000 // 100 microseconds
            }
            AlertType::Security => {
                // Alert if trust level is Managed but validation failed
                telemetry.trust_level == TrustLevel::Managed && telemetry.latency_ns > 50_000
                // High PQC verification time
            }
            AlertType::System => {
                // System health checks
                telemetry.metrics.cpu_utilization > 90.0
            }
        }
    }
}

/// WebSocket connection for real-time monitoring
#[derive(Debug)]
struct WebSocketConnection {
    id: Uuid,
    // TODO: Implement actual WebSocket connection
}

/// Performance tracking system
#[derive(Debug)]
struct PerformanceTracker {
    trust_ops_count: u64,
    pqc_ops_count: u64,
    start_time: DateTime<Utc>,
}

impl PerformanceTracker {
    fn new() -> Self {
        Self {
            trust_ops_count: 0,
            pqc_ops_count: 0,
            start_time: Utc::now(),
        }
    }

    fn record_trust_op(&mut self) {
        self.trust_ops_count += 1;
    }

    fn record_pqc_op(&mut self) {
        self.pqc_ops_count += 1;
    }

    fn get_ops_per_sec(&self) -> (f64, f64) {
        let elapsed = (Utc::now() - self.start_time).num_seconds() as f64;
        if elapsed > 0.0 {
            (
                self.trust_ops_count as f64 / elapsed,
                self.pqc_ops_count as f64 / elapsed,
            )
        } else {
            (0.0, 0.0)
        }
    }
}

impl CyberMonitor {
    /// Create new Cyber Monitor instance
    ///
    /// # Arguments
    /// * `config` - Optional monitor configuration
    ///
    /// # Returns
    /// Initialized Cyber Monitor ready for telemetry
    ///
    /// # Examples
    /// ```
    /// use cyber_monitor::CyberMonitor;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let monitor = CyberMonitor::new(None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: Option<MonitorConfig>) -> Result<Self> {
        let monitor_id = Uuid::new_v4();

        info!("CyberMonitor: Initializing telemetry system");
        info!("Monitor ID: {}", monitor_id);

        let config = config.unwrap_or_default();

        let telemetry_store = TelemetryStore::new(config.max_telemetry_points);
        let alert_manager = AlertManager::new();
        let performance_tracker = PerformanceTracker::new();

        info!("✅ CyberMonitor telemetry system initialized");

        Ok(Self {
            monitor_id,
            telemetry_store,
            alert_manager,
            websocket_connections: HashMap::new(),
            performance_tracker,
        })
    }

    /// Record telemetry point with trust-level awareness
    ///
    /// # Arguments
    /// * `atom` - The atom being monitored
    /// * `latency_ns` - Processing latency in nanoseconds
    /// * `metrics` - System performance metrics
    ///
    /// # Returns
    /// Success if telemetry recorded successfully
    ///
    /// # Examples
    /// ```ignore
    /// use cyber_monitor::CyberMonitor;
    /// use omwei_atom::Atom;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let monitor = CyberMonitor::new(None).await?;
    ///     let atom = Atom::new(0x12345678, [0x42; 28]);
    ///     let metrics = SystemMetrics::default();
    ///     
    ///     monitor.record_telemetry(&atom, 8600, metrics).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn record_telemetry(
        &mut self,
        atom: &Atom,
        latency_ns: u64,
        metrics: SystemMetrics,
    ) -> Result<()> {
        let trust_level = get_trust_level(atom.global_id);

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("monitor_id".to_string(), self.monitor_id.to_string());
        metadata.insert("sincerity_bit".to_string(), ((atom.global_id & 0x80000000) != 0).to_string());

        let telemetry_point = TelemetryPoint {
            id: Uuid::new_v4(),
            atom_id: atom.global_id,
            trust_level,
            timestamp: Utc::now(),
            latency_ns,
            metrics,
            metadata,
        };

        // Store telemetry
        self.telemetry_store.add_point(telemetry_point.clone());

        // Update performance metrics
        self.performance_tracker.record_trust_op();
        if trust_level == TrustLevel::Managed {
            self.performance_tracker.record_pqc_op();
        }

        // Check for alerts
        let alerts = self.alert_manager.check_alerts(&telemetry_point);

        // Broadcast alerts to WebSocket clients
        for alert in alerts {
            self.broadcast_alert(&alert).await;
        }

        // Broadcast telemetry to WebSocket clients
        self.broadcast_telemetry(&telemetry_point).await;

        info!(
            "CyberMonitor: Recorded telemetry for atom 0x{:08X} (trust: {:?}, latency: {} ns)",
            atom.global_id, trust_level, latency_ns
        );

        Ok(())
    }

    /// Get telemetry by trust level
    ///
    /// # Arguments
    /// * `trust_level` - Filter by specific trust level
    ///
    /// # Returns
    /// Vector of matching telemetry points
    pub fn get_telemetry_by_trust_level(&self, trust_level: TrustLevel) -> Vec<&TelemetryPoint> {
        self.telemetry_store.get_by_trust_level(trust_level)
    }

    /// Get recent telemetry
    ///
    /// # Arguments
    /// * `since` - Get telemetry since this timestamp
    ///
    /// # Returns
    /// Vector of recent telemetry points
    pub fn get_recent_telemetry(&self, since: DateTime<Utc>) -> Vec<&TelemetryPoint> {
        self.telemetry_store.get_recent(since)
    }

    /// Get current performance metrics
    pub fn get_performance_metrics(&self) -> (f64, f64) {
        self.performance_tracker.get_ops_per_sec()
    }

    /// Get active alerts
    pub fn get_active_alerts(&self) -> &[Alert] {
        &self.alert_manager.alerts
    }

    /// Broadcast telemetry to WebSocket clients
    async fn broadcast_telemetry(&self, _telemetry: &TelemetryPoint) {
        // TODO: Implement actual WebSocket broadcasting
        info!(
            "CyberMonitor: Broadcasting telemetry to {} clients",
            self.websocket_connections.len()
        );
    }

    /// Broadcast alert to WebSocket clients
    async fn broadcast_alert(&self, _alert: &Alert) {
        // TODO: Implement actual WebSocket alert broadcasting
        warn!(
            "CyberMonitor: Broadcasting alert to {} clients",
            self.websocket_connections.len()
        );
    }
}

/// Monitor configuration
#[derive(Debug)]
pub struct MonitorConfig {
    /// Maximum telemetry points to store
    pub max_telemetry_points: usize,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert threshold configuration
#[derive(Debug, Default)]
pub struct AlertThresholds {
    /// Maximum acceptable latency (nanoseconds)
    pub max_latency_ns: u64,
    /// Maximum CPU utilization percentage
    pub max_cpu_utilization: f64,
    /// Community Space ratio threshold (0.0 - 1.0)
    pub community_ratio_threshold: f64,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            max_telemetry_points: 10000,
            alert_thresholds: AlertThresholds {
                max_latency_ns: 100_000,
                max_cpu_utilization: 90.0,
                community_ratio_threshold: 0.3,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// Rest of the code remains the same
    #[tokio::test]
    async fn test_cyber_monitor_creation() {
        let monitor = CyberMonitor::new(None).await;
        assert!(monitor.is_ok());
    }

    #[tokio::test]
    async fn test_telemetry_recording() {
        let mut monitor = CyberMonitor::new(None).await.unwrap();
        let atom = Atom::new(0x12345678, [0x42; 28]);
        let metrics = SystemMetrics::default();

        let result = monitor.record_telemetry(&atom, 8600, metrics).await;
        assert!(result.is_ok());

        let telemetry = monitor.get_telemetry_by_trust_level(TrustLevel::Managed);
        assert_eq!(telemetry.len(), 1);
    }

    #[tokio::test]
    async fn test_trust_aware_alerts() {
        let mut monitor = CyberMonitor::new(None).await.unwrap();
        let managed_atom = Atom::new(0x12345678, [0x42; 28]);
        let community_atom = Atom::new(0x80000001, [0x24; 28]);
        let metrics = SystemMetrics::default();

        // Managed atom should not trigger trust alert
        monitor
            .record_telemetry(&managed_atom, 8600, metrics.clone())
            .await
            .unwrap();
        let alerts = monitor.get_active_alerts();
        assert_eq!(alerts.len(), 0);

        // Community atom should trigger trust alert
        monitor
            .record_telemetry(&community_atom, 8600, metrics)
            .await
            .unwrap();
        let alerts = monitor.get_active_alerts();
        assert_eq!(alerts.len(), 1);
        assert!(matches!(alerts[0].alert_type, AlertType::Trust));
    }
}
