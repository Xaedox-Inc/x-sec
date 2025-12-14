use x_sec_core::{
    Finding, ModuleConfig, ModuleError, ScanResult, ScanStatus, SecurityModule,
    Severity, Target,
};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Port scanner module for detecting open TCP ports
#[derive(Debug, Clone)]
pub struct PortScanner {
    config: ModuleConfig,
    common_ports: Vec<u16>,
}

impl Default for PortScanner {
    fn default() -> Self {
        Self {
            config: ModuleConfig::default(),
            // Common ports: FTP, SSH, Telnet, SMTP, HTTP, HTTPS, MySQL, PostgreSQL, HTTP-Alt
            common_ports: vec![21, 22, 23, 25, 80, 443, 3306, 5432, 8080],
        }
    }
}

impl PortScanner {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.common_ports = ports;
        self
    }
    
    async fn check_port(&self, host: &str, port: u16) -> bool {
        let addr = format!("{}:{}", host, port);
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);
        
        match timeout(timeout_duration, TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => true,
            _ => false,
        }
    }
    
    fn port_to_service(port: u16) -> &'static str {
        match port {
            21 => "FTP",
            22 => "SSH",
            23 => "Telnet",
            25 => "SMTP",
            80 => "HTTP",
            443 => "HTTPS",
            3306 => "MySQL",
            5432 => "PostgreSQL",
            8080 => "HTTP-Alt",
            _ => "Unknown",
        }
    }
}

#[async_trait::async_trait]
impl SecurityModule for PortScanner {
    fn name(&self) -> &str {
        "port_scanner"
    }
    
    fn description(&self) -> &str {
        "Scans for open TCP ports on target hosts"
    }
    
    fn version(&self) -> &str {
        "0.1.0"
    }
    
    fn configure(&mut self, config: ModuleConfig) -> Result<(), ModuleError> {
        self.config = config;
        Ok(())
    }
    
    async fn scan(&self, target: &Target) -> Result<ScanResult, ModuleError> {
        self.validate_target(target)?;
        
        let start_time = std::time::Instant::now();
        let mut result = ScanResult::new(self.name().to_string(), target.clone());
        
        let ports_to_scan = if let Some(port) = target.port {
            vec![port]
        } else {
            self.common_ports.clone()
        };
        
        let host = &target.host;
        let mut open_ports = Vec::new();
        
        // Scan ports concurrently
        let mut tasks = Vec::new();
        for port in ports_to_scan {
            let host = host.clone();
            let scanner = self.clone();
            tasks.push(tokio::spawn(async move {
                (port, scanner.check_port(&host, port).await)
            }));
        }
        
        for task in tasks {
            match task.await {
                Ok((port, is_open)) => {
                    if is_open {
                        open_ports.push(port);
                    }
                }
                Err(e) => {
                    return Err(ModuleError::ExecutionError(format!(
                        "Task execution failed: {}",
                        e
                    )));
                }
            }
        }
        
        // Create findings for open ports
        for port in open_ports {
            let service = Self::port_to_service(port);
            let finding = Finding::new(
                Severity::Info,
                format!("Open Port: {}", port),
                format!("Port {} ({}) is open and accepting connections", port, service),
                format!("{}:{}", host, port),
            )
            .with_metadata("port".to_string(), port.to_string())
            .with_metadata("service".to_string(), service.to_string());
            
            result = result.add_finding(finding);
        }
        
        let duration_ms = start_time.elapsed().as_millis() as u64;
        result = result
            .with_duration(duration_ms)
            .with_status(ScanStatus::Success);
        
        Ok(result)
    }
    
    fn validate_target(&self, target: &Target) -> Result<(), ModuleError> {
        if target.host.is_empty() {
            return Err(ModuleError::InvalidTarget(
                "Host cannot be empty".to_string(),
            ));
        }
        
        // Basic validation - could be enhanced with proper IP/hostname validation
        if !target.host.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
            return Err(ModuleError::InvalidTarget(
                "Invalid hostname or IP address".to_string(),
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_scanner_creation() {
        let scanner = PortScanner::new();
        assert_eq!(scanner.name(), "port_scanner");
        assert_eq!(scanner.version(), "0.1.0");
    }

    #[tokio::test]
    async fn test_validate_target() {
        let scanner = PortScanner::new();
        
        let valid_target = Target::new("127.0.0.1".to_string(), None, Protocol::TCP);
        assert!(scanner.validate_target(&valid_target).is_ok());
        
        let invalid_target = Target::new("".to_string(), None, Protocol::TCP);
        assert!(scanner.validate_target(&invalid_target).is_err());
    }
}
