use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

/// Initialize the database with required tables
pub fn init_database(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS scan_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            module_name TEXT NOT NULL,
            target TEXT NOT NULL,
            protocol TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            status TEXT NOT NULL,
            results TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS findings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            scan_id INTEGER NOT NULL,
            severity TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (scan_id) REFERENCES scan_results(id)
        )",
        [],
    )?;
    
    Ok(conn)
}

/// Save scan result to database
pub fn save_scan_result(
    conn: &Connection,
    module_name: &str,
    target: &str,
    protocol: &str,
    timestamp: &str,
    status: &str,
    results: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO scan_results (module_name, target, protocol, timestamp, status, results)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![module_name, target, protocol, timestamp, status, results],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// Get scan history from database
pub fn get_scan_history(conn: &Connection, limit: usize) -> Result<Vec<serde_json::Value>> {
    let mut stmt = conn.prepare(
        "SELECT id, module_name, target, protocol, timestamp, status, results
         FROM scan_results
         ORDER BY id DESC
         LIMIT ?1"
    )?;
    
    let results = stmt
        .query_map(params![limit], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "module_name": row.get::<_, String>(1)?,
                "target": row.get::<_, String>(2)?,
                "protocol": row.get::<_, String>(3)?,
                "timestamp": row.get::<_, String>(4)?,
                "status": row.get::<_, String>(5)?,
                "results": row.get::<_, String>(6)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(results)
}
