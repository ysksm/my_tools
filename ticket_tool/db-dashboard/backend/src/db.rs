use crate::{error::Result, models::*};
use duckdb::Connection;
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Initialize dashboard table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dashboards (
                id VARCHAR PRIMARY KEY,
                name VARCHAR NOT NULL,
                config JSON NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get_schema(&self) -> Result<SchemaResponse> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT table_name 
             FROM information_schema.tables 
             WHERE table_schema = 'main' 
             AND table_type = 'BASE TABLE'
             AND table_name != 'dashboards'"
        )?;
        
        let tables = stmt.query_map([], |row| {
            let table_name: String = row.get(0)?;
            Ok(table_name)
        })?;
        
        let mut table_infos = Vec::new();
        
        for table in tables {
            let table_name = table?;
            
            let mut col_stmt = conn.prepare(
                "SELECT column_name, data_type, is_nullable 
                 FROM information_schema.columns 
                 WHERE table_name = ? 
                 ORDER BY ordinal_position"
            )?;
            
            let columns = col_stmt.query_map([&table_name], |row| {
                Ok(ColumnInfo {
                    name: row.get(0)?,
                    data_type: row.get(1)?,
                    is_nullable: row.get::<_, String>(2)? == "YES",
                })
            })?;
            
            let column_infos: Result<Vec<_>> = columns.collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into);
            
            table_infos.push(TableInfo {
                name: table_name,
                columns: column_infos?,
            });
        }
        
        Ok(SchemaResponse { tables: table_infos })
    }

    pub fn execute_query(&self, sql: &str) -> Result<QueryResponse> {
        let conn = self.conn.lock().unwrap();
        
        // Basic SQL injection prevention - check for dangerous patterns
        let sql_lower = sql.to_lowercase();
        if sql_lower.contains("drop") || sql_lower.contains("delete") || 
           sql_lower.contains("update") || sql_lower.contains("insert") ||
           sql_lower.contains("create") || sql_lower.contains("alter") {
            return Err(crate::error::AppError::BadRequest(
                "Only SELECT queries are allowed".to_string()
            ));
        }
        
        let mut stmt = conn.prepare(sql)?;
        let column_count = stmt.column_count();
        
        let mut columns = Vec::new();
        for i in 0..column_count {
            columns.push(stmt.column_name(i)?.to_string());
        }
        
        let rows_result: std::result::Result<Vec<Vec<serde_json::Value>>, duckdb::Error> = 
            stmt.query_map([], |row| {
                let mut values = Vec::new();
                for i in 0..column_count {
                    let value: serde_json::Value = match row.get_ref(i)? {
                        duckdb::types::ValueRef::Null => serde_json::Value::Null,
                        duckdb::types::ValueRef::Boolean(b) => serde_json::Value::Bool(b),
                        duckdb::types::ValueRef::TinyInt(i) => serde_json::Value::Number(i.into()),
                        duckdb::types::ValueRef::SmallInt(i) => serde_json::Value::Number(i.into()),
                        duckdb::types::ValueRef::Int(i) => serde_json::Value::Number(i.into()),
                        duckdb::types::ValueRef::BigInt(i) => serde_json::Value::Number(i.into()),
                        duckdb::types::ValueRef::Float(f) => serde_json::json!(f),
                        duckdb::types::ValueRef::Double(f) => serde_json::json!(f),
                        duckdb::types::ValueRef::Text(s) => serde_json::Value::String(String::from_utf8_lossy(s).to_string()),
                        _ => serde_json::Value::String(row.get::<_, String>(i)?),
                    };
                    values.push(value);
                }
                Ok(values)
            })?
            .collect();
        
        let rows = rows_result?;
        let row_count = rows.len();
        
        Ok(QueryResponse {
            columns,
            rows,
            row_count,
        })
    }
    
    pub fn export_table_to_parquet(&self, table_name: &str, output_path: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Validate table name to prevent SQL injection
        let valid_tables = self.get_schema()?.tables;
        if !valid_tables.iter().any(|t| t.name == table_name) {
            return Err(crate::error::AppError::NotFound(
                format!("Table '{}' not found", table_name)
            ));
        }
        
        let query = format!(
            "COPY (SELECT * FROM {}) TO '{}' (FORMAT PARQUET)",
            table_name, output_path
        );
        
        conn.execute(&query, [])?;
        Ok(())
    }
}