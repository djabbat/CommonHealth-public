use axum::{
    extract::{Path, Query, State},
    routing,
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::{
    models::{Counter3Record, Counter3Parameters, Tissue, Counter3RecordCreate, Counter3ParametersCreate, TissueCreate},
    error::AppError,
};

#[derive(Debug, Deserialize)]
struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}

pub fn api_routes() -> Router<PgPool> {
    Router::new()
        .route("/counter3_records", routing::get(list_counter3_records).post(create_counter3_record))
        .route("/counter3_records/:id", routing::get(get_counter3_record).put(update_counter3_record).delete(delete_counter3_record))
        .route("/counter3_parameters", routing::get(list_counter3_parameters).post(create_counter3_parameters))
        .route("/counter3_parameters/:id", routing::get(get_counter3_parameters).put(update_counter3_parameters).delete(delete_counter3_parameters))
        .route("/tissues", routing::get(list_tissues).post(create_tissue))
        .route("/tissues/:id", routing::get(get_tissue).put(update_tissue).delete(delete_tissue))
        .route("/compute_d3", routing::post(compute_d3))
}

// Counter3Record endpoints
async fn list_counter3_records(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Counter3Record>>, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(100);
    let offset = (page - 1) * per_page;
    
    let records = sqlx::query_as::<_, Counter3Record>(
        "SELECT * FROM counter3_records ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    Ok(Json(records))
}

async fn get_counter3_record(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Counter3Record>, AppError> {
    let record = sqlx::query_as::<_, Counter3Record>(
        "SELECT * FROM counter3_records WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Counter3Record not found".to_string()))?;
    
    Ok(Json(record))
}

async fn create_counter3_record(
    State(pool): State<PgPool>,
    Json(payload): Json<Counter3RecordCreate>,
) -> Result<Json<Counter3Record>, AppError> {
    let record = sqlx::query_as::<_, Counter3Record>(
        r#"
        INSERT INTO counter3_records 
        (tissue_id, n_cell_divisions, t_time, d3_value, metadata)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(payload.tissue_id)
    .bind(payload.n_cell_divisions)
    .bind(payload.t_time)
    .bind(payload.d3_value)
    .bind(payload.metadata)
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    info!("Created Counter3Record with id: {}", record.id);
    Ok(Json(record))
}

async fn update_counter3_record(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Counter3RecordCreate>,
) -> Result<Json<Counter3Record>, AppError> {
    let record = sqlx::query_as::<_, Counter3Record>(
        r#"
        UPDATE counter3_records 
        SET tissue_id = $1, n_cell_divisions = $2, t_time = $3, d3_value = $4, metadata = $5, updated_at = CURRENT_TIMESTAMP
        WHERE id = $6
        RETURNING *
        "#
    )
    .bind(payload.tissue_id)
    .bind(payload.n_cell_divisions)
    .bind(payload.t_time)
    .bind(payload.d3_value)
    .bind(payload.metadata)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Counter3Record not found".to_string()))?;
    
    info!("Updated Counter3Record with id: {}", record.id);
    Ok(Json(record))
}

async fn delete_counter3_record(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        "DELETE FROM counter3_records WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Counter3Record not found".to_string()));
    }
    
    info!("Deleted Counter3Record with id: {}", id);
    Ok(Json(()))
}

// Counter3Parameters endpoints (similar pattern, shortened for brevity)
async fn list_counter3_parameters(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Counter3Parameters>>, AppError> {
    let params = sqlx::query_as::<_, Counter3Parameters>(
        "SELECT * FROM counter3_parameters ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    Ok(Json(params))
}

async fn get_counter3_parameters(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Counter3Parameters>, AppError> {
    let params = sqlx::query_as::<_, Counter3Parameters>(
        "SELECT * FROM counter3_parameters WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Counter3Parameters not found".to_string()))?;
    
    Ok(Json(params))
}

async fn create_counter3_parameters(
    State(pool): State<PgPool>,
    Json(payload): Json<Counter3ParametersCreate>,
) -> Result<Json<Counter3Parameters>, AppError> {
    let params = sqlx::query_as::<_, Counter3Parameters>(
        r#"
        INSERT INTO counter3_parameters 
        (tissue_id, d3_0, alpha3, n3_star, beta3, tau3, gamma3, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(payload.tissue_id)
    .bind(payload.d3_0)
    .bind(payload.alpha3)
    .bind(payload.n3_star)
    .bind(payload.beta3)
    .bind(payload.tau3)
    .bind(payload.gamma3)
    .bind(payload.metadata)
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    info!("Created Counter3Parameters with id: {}", params.id);
    Ok(Json(params))
}

async fn update_counter3_parameters(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Counter3ParametersCreate>,
) -> Result<Json<Counter3Parameters>, AppError> {
    let params = sqlx::query_as::<_, Counter3Parameters>(
        r#"
        UPDATE counter3_parameters 
        SET tissue_id = $1, d3_0 = $2, alpha3 = $3, n3_star = $4, beta3 = $5, tau3 = $6, gamma3 = $7, metadata = $8, updated_at = CURRENT_TIMESTAMP
        WHERE id = $9
        RETURNING *
        "#
    )
    .bind(payload.tissue_id)
    .bind(payload.d3_0)
    .bind(payload.alpha3)
    .bind(payload.n3_star)
    .bind(payload.beta3)
    .bind(payload.tau3)
    .bind(payload.gamma3)
    .bind(payload.metadata)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Counter3Parameters not found".to_string()))?;
    
    info!("Updated Counter3Parameters with id: {}", params.id);
    Ok(Json(params))
}

async fn delete_counter3_parameters(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        "DELETE FROM counter3_parameters WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Counter3Parameters not found".to_string()));
    }
    
    info!("Deleted Counter3Parameters with id: {}", id);
    Ok(Json(()))
}

// Tissue endpoints
async fn list_tissues(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Tissue>>, AppError> {
    let tissues = sqlx::query_as::<_, Tissue>(
        "SELECT * FROM tissues ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    Ok(Json(tissues))
}

async fn get_tissue(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Tissue>, AppError> {
    let tissue = sqlx::query_as::<_, Tissue>(
        "SELECT * FROM tissues WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Tissue not found".to_string()))?;
    
    Ok(Json(tissue))
}

async fn create_tissue(
    State(pool): State<PgPool>,
    Json(payload): Json<TissueCreate>,
) -> Result<Json<Tissue>, AppError> {
    let tissue = sqlx::query_as::<_, Tissue>(
        r#"
        INSERT INTO tissues (name, description, mitotic_index, metabolic_rate, weight_w3)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.mitotic_index)
    .bind(payload.metabolic_rate)
    .bind(payload.weight_w3)
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    info!("Created tissue with id: {}", tissue.id);
    Ok(Json(tissue))
}

async fn update_tissue(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TissueCreate>,
) -> Result<Json<Tissue>, AppError> {
    let tissue = sqlx::query_as::<_, Tissue>(
        r#"
        UPDATE tissues 
        SET name = $1, description = $2, mitotic_index = $3, metabolic_rate = $4, weight_w3 = $5, updated_at = CURRENT_TIMESTAMP
        WHERE id = $6
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.mitotic_index)
    .bind(payload.metabolic_rate)
    .bind(payload.weight_w3)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Tissue not found".to_string()))?;
    
    info!("Updated tissue with id: {}", tissue.id);
    Ok(Json(tissue))
}

async fn delete_tissue(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        "DELETE FROM tissues WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tissue not found".to_string()));
    }
    
    info!("Deleted tissue with id: {}", id);
    Ok(Json(()))
}

// Compute D3 endpoint
#[derive(Debug, Deserialize)]
struct ComputeD3Request {
    tissue_id: Uuid,
    n: f64,
    t: f64,
    parameters_id: Option<Uuid>,
}

#[derive(Debug, serde::Serialize)]
struct ComputeD3Response {
    d3_value: f64,
    parameters_used: Counter3Parameters,
}

async fn compute_d3(
    State(pool): State<PgPool>,
    Json(request): Json<ComputeD3Request>,
) -> Result<Json<ComputeD3Response>, AppError> {
    // Get parameters for the tissue
    let params = if let Some(id) = request.parameters_id {
        sqlx::query_as::<_, Counter3Parameters>(
            "SELECT * FROM counter3_parameters WHERE id = $1 AND tissue_id = $2"
        )
        .bind(id)
        .bind(request.tissue_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Counter3Parameters not found".to_string()))?
    } else {
        sqlx::query_as::<_, Counter3Parameters>(
            "SELECT * FROM counter3_parameters WHERE tissue_id = $1 ORDER BY created_at DESC LIMIT 1"
        )
        .bind(request.tissue_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("No Counter3Parameters found for tissue".to_string()))?
    };
    
    // Compute D3 according to the kinetic equation
    // D3(n,t) = D3_0 + α3 * (n/n3*) + β3 * (t/τ3) + γ3 * I(other_counters)
    // According to CORRECTIONS_2026-04-22: γ3 = 0 by default (null hypothesis)
    // I(other_counters) is assumed to be 0 for scaffold
    let d3_value = params.d3_0 
        + params.alpha3 * (request.n / params.n3_star)
        + params.beta3 * (request.t / params.tau3)
        + params.gamma3 * 0.0;  // Explicitly 0 per canonical rules
    
    info!("Computed D3: {} for tissue_id: {}, n: {}, t: {}", d3_value, request.tissue_id, request.n, request.t);
    
    Ok(Json(ComputeD3Response {
        d3_value,
        parameters_used: params,
    }))
}