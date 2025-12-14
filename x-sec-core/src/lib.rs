pub mod db;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during module execution
#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid target: {0}")]
    InvalidTarget(String),
}

/// Network protocol types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    HTTP,
    HTTPS,
    SSH,
    FTP,
    SMTP,
    DNS,
    Other(String),
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
            Protocol::HTTP => write!(f, "HTTP"),
            Protocol::HTTPS => write!(f, "HTTPS"),
            Protocol::SSH => write!(f, "SSH"),
            Protocol::FTP => write!(f, "FTP"),
            Protocol::SMTP => write!(f, "SMTP"),
            Protocol::DNS => write!(f, "DNS"),
            Protocol::Other(s) => write!(f, "{}", s),
        }
    }
}

/// Severity levels for findings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Low => write!(f, "Low"),
            Severity::Medium => write!(f, "Medium"),
            Severity::High => write!(f, "High"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

/// Target specification for scans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub host: String,
    pub port: Option<u16>,
    pub protocol: Protocol,
}

impl Target {
    pub fn new(host: String, port: Option<u16>, protocol: Protocol) -> Self {
        Self { host, port, protocol }
    }
}

/// Individual security finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub affected_target: String,
    pub recommendation: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Finding {
    pub fn new(
        severity: Severity,
        title: String,
        description: String,
        affected_target: String,
    ) -> Self {
        Self {
            severity,
            title,
            description,
            affected_target,
            recommendation: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_recommendation(mut self, recommendation: String) -> Self {
        self.recommendation = Some(recommendation);
        self
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Result of a security scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub module_name: String,
    pub target: Target,
    pub timestamp: String,
    pub duration_ms: u64,
    pub status: ScanStatus,
    pub findings: Vec<Finding>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanStatus {
    Success,
    Failed(String),
    Partial,
}

impl ScanResult {
    pub fn new(module_name: String, target: Target) -> Self {
        Self {
            module_name,
            target,
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: 0,
            status: ScanStatus::Success,
            findings: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }
    
    pub fn with_status(mut self, status: ScanStatus) -> Self {
        self.status = status;
        self
    }
    
    pub fn add_finding(mut self, finding: Finding) -> Self {
        self.findings.push(finding);
        self
    }
}

/// Configuration for security modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub timeout_ms: u64,
    pub concurrent_scans: usize,
    pub retry_count: u32,
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 5000,
            concurrent_scans: 10,
            retry_count: 3,
            custom: HashMap::new(),
        }
    }
}

/// Main trait that all security modules must implement
#[async_trait::async_trait]
pub trait SecurityModule: Send + Sync {
    /// Returns the name of the module
    fn name(&self) -> &str;
    
    /// Returns a description of what the module does
    fn description(&self) -> &str;
    
    /// Returns the version of the module
    fn version(&self) -> &str;
    
    /// Configure the module with custom settings
    fn configure(&mut self, config: ModuleConfig) -> Result<(), ModuleError>;
    
    /// Execute the security scan
    async fn scan(&self, target: &Target) -> Result<ScanResult, ModuleError>;
    
    /// Validate that the target is appropriate for this module
    fn validate_target(&self, target: &Target) -> Result<(), ModuleError>;
}
