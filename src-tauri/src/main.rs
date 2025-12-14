// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use x_sec_core::{Protocol, SecurityModule, Target};
use x_sec_modules::PortScanner;

// Module info structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModuleInfo {
    name: String,
    description: String,
    version: String,
}

// Scan request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanRequest {
    module: String,
    target: String,
    port: Option<u16>,
}

// Scan response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanResponse {
    success: bool,
    module_name: String,
    target: String,
    timestamp: String,
    duration_ms: u64,
    findings: Vec<FindingInfo>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FindingInfo {
    severity: String,
    title: String,
    description: String,
    affected_target: String,
    metadata: std::collections::HashMap<String, String>,
}

// Application state
struct AppState {
    scan_history: Mutex<Vec<ScanResponse>>,
}

#[tauri::command]
fn list_modules() -> Vec<ModuleInfo> {
    let mut modules = Vec::new();
    
    let port_scanner = PortScanner::new();
    modules.push(ModuleInfo {
        name: port_scanner.name().to_string(),
        description: port_scanner.description().to_string(),
        version: port_scanner.version().to_string(),
    });
    
    modules
}

#[tauri::command]
async fn execute_scan(
    request: ScanRequest,
    state: State<'_, AppState>,
) -> Result<ScanResponse, String> {
    let target = Target::new(
        request.target.clone(),
        request.port,
        Protocol::TCP,
    );
    
    let response = match request.module.as_str() {
        "port_scanner" => {
            let scanner = PortScanner::new();
            match scanner.scan(&target).await {
                Ok(result) => {
                    let findings: Vec<FindingInfo> = result
                        .findings
                        .iter()
                        .map(|f| FindingInfo {
                            severity: f.severity.to_string(),
                            title: f.title.clone(),
                            description: f.description.clone(),
                            affected_target: f.affected_target.clone(),
                            metadata: f.metadata.clone(),
                        })
                        .collect();
                    
                    ScanResponse {
                        success: true,
                        module_name: result.module_name,
                        target: result.target.host,
                        timestamp: result.timestamp,
                        duration_ms: result.duration_ms,
                        findings,
                        error: None,
                    }
                }
                Err(e) => ScanResponse {
                    success: false,
                    module_name: scanner.name().to_string(),
                    target: request.target.clone(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    duration_ms: 0,
                    findings: Vec::new(),
                    error: Some(e.to_string()),
                },
            }
        }
        _ => {
            return Err(format!("Unknown module: {}", request.module));
        }
    };
    
    // Store in history
    state.scan_history.lock().unwrap().push(response.clone());
    
    Ok(response)
}

#[tauri::command]
fn get_scan_history(state: State<'_, AppState>) -> Vec<ScanResponse> {
    let history = state.scan_history.lock().unwrap();
    history.clone()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            scan_history: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            list_modules,
            execute_scan,
            get_scan_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
