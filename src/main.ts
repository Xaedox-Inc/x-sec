import { invoke } from '@tauri-apps/api/core';

// Type definitions
interface ModuleInfo {
  name: string;
  description: string;
  version: string;
}

interface ScanRequest {
  module: string;
  target: string;
  port?: number;
}

interface FindingInfo {
  severity: string;
  title: string;
  description: string;
  affected_target: string;
  metadata: Record<string, string>;
}

interface ScanResponse {
  success: boolean;
  module_name: string;
  target: string;
  timestamp: string;
  duration_ms: number;
  findings: FindingInfo[];
  error?: string;
}

// DOM elements
const modulesList = document.getElementById('modules-list')!;
const moduleSelect = document.getElementById('module-select') as HTMLSelectElement;
const scanForm = document.getElementById('scan-form') as HTMLFormElement;
const targetInput = document.getElementById('target-input') as HTMLInputElement;
const portInput = document.getElementById('port-input') as HTMLInputElement;
const resultsContainer = document.getElementById('results-container')!;
const historyContainer = document.getElementById('history-container')!;
const loadHistoryBtn = document.getElementById('load-history-btn')!;

// Load available modules
async function loadModules() {
  try {
    const modules = await invoke<ModuleInfo[]>('list_modules');
    
    // Display modules list
    modulesList.innerHTML = modules.map(mod => `
      <div class="module-item">
        <div class="module-header">
          <strong>${mod.name}</strong>
          <span class="version">v${mod.version}</span>
        </div>
        <p class="module-description">${mod.description}</p>
      </div>
    `).join('');
    
    // Populate module select
    moduleSelect.innerHTML = '<option value="">Select a module...</option>' +
      modules.map(mod => `<option value="${mod.name}">${mod.name} v${mod.version}</option>`).join('');
    
  } catch (error) {
    modulesList.innerHTML = `<p class="error">Failed to load modules: ${error}</p>`;
  }
}

// Execute scan
scanForm.addEventListener('submit', async (e) => {
  e.preventDefault();
  
  const module = moduleSelect.value;
  const target = targetInput.value.trim();
  const portValue = portInput.value.trim();
  
  if (!module || !target) {
    alert('Please select a module and enter a target');
    return;
  }
  
  const request: ScanRequest = {
    module,
    target,
  };
  
  if (portValue) {
    request.port = parseInt(portValue, 10);
  }
  
  // Show loading state
  resultsContainer.innerHTML = `
    <div class="loading-spinner">
      <p>🔄 Scanning ${target}...</p>
    </div>
  `;
  
  try {
    const result = await invoke<ScanResponse>('execute_scan', { request });
    displayScanResult(result);
  } catch (error) {
    resultsContainer.innerHTML = `
      <div class="error">
        <h3>❌ Scan Failed</h3>
        <p>${error}</p>
      </div>
    `;
  }
});

// Display scan result
function displayScanResult(result: ScanResponse) {
  if (!result.success) {
    resultsContainer.innerHTML = `
      <div class="error">
        <h3>❌ Scan Failed</h3>
        <p>${result.error || 'Unknown error'}</p>
      </div>
    `;
    return;
  }
  
  const severityClass = (severity: string) => `severity-${severity.toLowerCase()}`;
  
  resultsContainer.innerHTML = `
    <div class="scan-result success">
      <div class="result-header">
        <h3>✅ Scan Completed</h3>
        <div class="result-meta">
          <span><strong>Module:</strong> ${result.module_name}</span>
          <span><strong>Target:</strong> ${result.target}</span>
          <span><strong>Duration:</strong> ${result.duration_ms}ms</span>
          <span><strong>Timestamp:</strong> ${new Date(result.timestamp).toLocaleString()}</span>
        </div>
      </div>
      
      <div class="findings">
        <h4>Findings (${result.findings.length})</h4>
        ${result.findings.length === 0 
          ? '<p class="no-findings">No security findings detected.</p>'
          : result.findings.map(finding => `
              <div class="finding ${severityClass(finding.severity)}">
                <div class="finding-header">
                  <span class="severity-badge ${severityClass(finding.severity)}">${finding.severity}</span>
                  <strong>${finding.title}</strong>
                </div>
                <p>${finding.description}</p>
                <div class="finding-metadata">
                  <small><strong>Affected:</strong> ${finding.affected_target}</small>
                  ${Object.entries(finding.metadata).map(([key, value]) => 
                    `<small><strong>${key}:</strong> ${value}</small>`
                  ).join('')}
                </div>
              </div>
            `).join('')
        }
      </div>
    </div>
  `;
}

// Load scan history
loadHistoryBtn.addEventListener('click', async () => {
  try {
    const history = await invoke<ScanResponse[]>('get_scan_history');
    
    if (history.length === 0) {
      historyContainer.innerHTML = '<p class="placeholder">No scan history available.</p>';
      return;
    }
    
    historyContainer.innerHTML = history.map(scan => `
      <div class="history-item">
        <div class="history-header">
          <strong>${scan.module_name}</strong>
          <span>${scan.target}</span>
          <span class="timestamp">${new Date(scan.timestamp).toLocaleString()}</span>
        </div>
        <div class="history-details">
          <span>${scan.success ? '✅ Success' : '❌ Failed'}</span>
          <span>${scan.findings.length} findings</span>
          <span>${scan.duration_ms}ms</span>
        </div>
      </div>
    `).join('');
    
  } catch (error) {
    historyContainer.innerHTML = `<p class="error">Failed to load history: ${error}</p>`;
  }
});

// Initialize
loadModules();
