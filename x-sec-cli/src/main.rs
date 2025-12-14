use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber;
use x_sec_core::{Protocol, SecurityModule, Target};
use x_sec_modules::PortScanner;

#[derive(Parser)]
#[command(name = "x-sec")]
#[command(about = "X-Sec - Modular Security Penetration Testing Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available security modules
    ListModules,
    
    /// Execute a security scan
    Scan {
        /// Module to use for scanning
        #[arg(short, long)]
        module: String,
        
        /// Target host (IP address or hostname)
        #[arg(short, long)]
        target: String,
        
        /// Target port (optional, scans common ports if not specified)
        #[arg(short, long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();
    
    match cli.command {
        Commands::ListModules => {
            list_modules();
        }
        Commands::Scan { module, target, port } => {
            execute_scan(&module, &target, port).await?;
        }
    }
    
    Ok(())
}

fn list_modules() {
    println!("\n📋 Available Security Modules:\n");
    
    let port_scanner = PortScanner::new();
    println!("  • {} v{}", port_scanner.name(), port_scanner.version());
    println!("    {}\n", port_scanner.description());
}

async fn execute_scan(module_name: &str, target_host: &str, port: Option<u16>) -> Result<()> {
    info!("Starting scan with module: {}", module_name);
    info!("Target: {}", target_host);
    
    let target = Target::new(target_host.to_string(), port, Protocol::TCP);
    
    match module_name {
        "port_scanner" => {
            let scanner = PortScanner::new();
            println!("\n🔍 Running {} on target: {}", scanner.name(), target_host);
            
            match scanner.scan(&target).await {
                Ok(result) => {
                    println!("\n✅ Scan completed successfully!");
                    println!("   Duration: {}ms", result.duration_ms);
                    println!("   Findings: {}\n", result.findings.len());
                    
                    if result.findings.is_empty() {
                        println!("   No open ports found.");
                    } else {
                        println!("   Open Ports:");
                        for finding in &result.findings {
                            let default_port = "?".to_string();
                            let default_service = "Unknown".to_string();
                            let port = finding.metadata.get("port").unwrap_or(&default_port);
                            let service = finding.metadata.get("service").unwrap_or(&default_service);
                            println!("     • Port {}: {} - {}", port, service, finding.description);
                        }
                    }
                    println!();
                }
                Err(e) => {
                    eprintln!("❌ Scan failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("❌ Unknown module: {}", module_name);
            eprintln!("   Use 'x-sec list-modules' to see available modules");
            std::process::exit(1);
        }
    }
    
    Ok(())
}
