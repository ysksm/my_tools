use crate::{db::Database, error::Result, models::*};
use actix_web::{web, HttpResponse};
use std::sync::Arc;
use tempfile::NamedTempFile;

pub async fn get_schema(db: web::Data<Arc<Database>>) -> Result<HttpResponse> {
    let schema = db.get_schema()?;
    Ok(HttpResponse::Ok().json(schema))
}

pub async fn get_table_data(
    db: web::Data<Arc<Database>>,
    query: web::Query<DataRequest>,
) -> Result<HttpResponse> {
    let temp_file = NamedTempFile::new()?;
    let output_path = temp_file.path().to_str()
        .ok_or_else(|| crate::error::AppError::InternalServerError)?;
    
    db.export_table_to_parquet(&query.table, output_path)?;
    
    let file_content = std::fs::read(output_path)?;
    
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header(("Content-Disposition", 
            format!("attachment; filename=\"{}.parquet\"", query.table)))
        .body(file_content))
}

pub async fn execute_query(
    db: web::Data<Arc<Database>>,
    body: web::Json<QueryRequest>,
) -> Result<HttpResponse> {
    let result = db.execute_query(&body.sql)?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    })))
}