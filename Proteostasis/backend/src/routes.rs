use axum::{
    extract::{Path, State},
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::info;

use crate::error::ApiError;
use crate::models::{ProteostasisParameter, ProteostasisTimeSeries, DamageComputationRequest};

pub async fn list_parameters(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Listing all proteostasis parameters");
    let parameters = sqlx::query_as!(
        ProteostasisParameter,
        r#"SELECT * FROM proteostasis_parameters ORDER BY created_at DESC"#
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(parameters))
}

pub async fn get_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Getting proteostasis parameter: {}", id);
    let parameter = sqlx::query_as!(
        ProteostasisParameter,
        r#"SELECT * FROM proteostasis_parameters WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await?;
    
    match parameter {
        Some(p) => Ok(Json(p)),
        None => Err(ApiError::NotFound("Parameter not found".to_string())),
    }
}

pub async fn create_parameter(
    State(pool): State<PgPool>,
    Json(mut param): Json<ProteostasisParameter>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Creating new proteostasis parameter");
    
    param.id = Uuid::new_v4();
    param.created_at = Utc::now();
    param.updated_at = Utc::now();
    
    sqlx::query!(
        r#"INSERT INTO proteostasis_parameters 
        (id, tissue_type, d50, alpha5, n5_critical, beta5, tau5, 
         gamma51, gamma52, gamma53, gamma54, gamma55, weight,
         created_at, updated_at) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#,
        param.id,
        param.tissue_type,
        param.d50,
        param.alpha5,
        param.n5_critical,
        param.beta5,
        param.tau5,
        param.gamma51,
        param.gamma52,
        param.gamma53,
        param.gamma54,
        param.gamma55,
        param.weight,
        param.created_at,
        param.updated_at
    )
    .execute(&pool)
    .await?;
    
    Ok(Json(param))
}

pub async fn update_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(mut param): Json<ProteostasisParameter>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Updating proteostasis parameter: {}", id);
    
    param.updated_at = Utc::now();
    
    let rows_affected = sqlx::query!(
        r#"UPDATE proteostasis_parameters SET
        tissue_type = $2, d50 = $3, alpha5 = $4, n5_critical = $5,
        beta5 = $6, tau5 = $7, gamma51 = $8, gamma52 = $9, gamma53 = $10,
        gamma54 = $11, gamma55 = $12, weight = $13, updated_at = $14
        WHERE id = $1"#,
        id,
        param.tissue_type,
        param.d50,
        param.alpha5,
        param.n5_critical,
        param.beta5,
        param.tau5,
        param.gamma51,
        param.gamma52,
        param.gamma53,
        param.gamma54,
        param.gamma55,
        param.weight,
        param.updated_at
    )
    .execute(&pool)
    .await?
    .rows_affected();
    
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Parameter not found".to_string()));
    }
    
    param.id = id;
    Ok(Json(param))
}

pub async fn delete_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Deleting proteostasis parameter: {}", id);
    
    let rows_affected = sqlx::query!(
        "DELETE FROM proteostasis_parameters WHERE id = $1",
        id
    )
    .execute(&pool)
    .await?
    .rows_affected();
    
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Parameter not found".to_string()));
    }
    
    Ok(Json(serde_json::json!({"message": "Parameter deleted"})))
}

pub async fn list_time_series(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Listing all proteostasis time series");
    let series = sqlx::query_as!(
        ProteostasisTimeSeries,
        r#"SELECT * FROM proteostasis_time_series ORDER BY timestamp DESC"#
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(series))
}

pub async fn get_time_series(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Getting proteostasis time series: {}", id);
    let series = sqlx::query_as!(
        ProteostasisTimeSeries,
        r#"SELECT * FROM proteostasis_time_series WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await?;
    
    match series {
        Some(s) => Ok(Json(s)),
        None => Err(ApiError::NotFound("Time series not found".to_string())),
    }
}

pub async fn create_time_series(
    State(pool): State<PgPool>,
    Json(mut ts): Json<ProteostasisTimeSeries>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Creating new proteostasis time series");
    
    ts.id = Uuid::new_v4();
    ts.created_at = Utc::now();
    ts.updated_at = Utc::now();
    
    sqlx::query!(
        r#"INSERT INTO proteostasis_time_series
        (id, subject_id, timestamp, cell_divisions, chronological_time,
         d5_value, parameter_set_id, metadata, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        ts.id,
        ts.subject_id,
        ts.timestamp,
        ts.cell_divisions,
        ts.chronological_time,
        ts.d5_value,
        ts.parameter_set_id,
        ts.metadata,
        ts.created_at,
        ts.updated_at
    )
    .execute(&pool)
    .await?;
    
    Ok(Json(ts))
}

pub async fn update_time_series(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(mut ts): Json<ProteostasisTimeSeries>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Updating proteostasis time series: {}", id);
    
    ts.updated_at = Utc::now();
    
    let rows_affected = sqlx::query!(
        r#"UPDATE proteostasis_time_series SET
        subject_id = $2, timestamp = $3, cell_divisions = $4,
        chronological_time = $5, d5_value = $6, parameter_set_id = $7,
        metadata = $8, updated_at = $9
        WHERE id = $1"#,
        id,
        ts.subject_id,
        ts.timestamp,
        ts.cell_divisions,
        ts.chronological_time,
        ts.d5_value,
        ts.parameter_set_id,
        ts.metadata,
        ts.updated_at
    )
    .execute(&pool)
    .await?
    .rows_affected();
    
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Time series not found".to_string()));
    }
    
    ts.id = id;
    Ok(Json(ts))
}

pub async fn delete_time_series(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Deleting proteostasis time series: {}", id);
    
    let rows_affected = sqlx::query!(
        "DELETE FROM proteostasis_time_series WHERE id = $1",
        id
    )
    .execute(&pool)
    .await?
    .rows_affected();
    
    if rows_affected == 0 {
        return Err(ApiError::NotFound("Time series not found".to_string()));
    }
    
    Ok(Json(serde_json::json!({"message": "Time series deleted"})))
}

pub async fn compute_damage(
    State(pool): State<PgPool>,
    Json(req): Json<DamageComputationRequest>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Computing D5 damage for request");
    
    let param = if let Some(param_id) = req.parameter_set_id {
        sqlx::query_as!(
            ProteostasisParameter,
            "SELECT * FROM proteostasis_parameters WHERE id = $1",
            param_id
        )
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Parameter set not found".to_string()))?
    } else {
        ProteostasisParameter::default()
    };
    
    let d5 = compute_d5_value(&param, req.cell_divisions, req.chronological_time);
    
    Ok(Json(serde_json::json!({
        "d5_value": d5,
        "parameters_used": param,
        "cell_divisions": req.cell_divisions,
        "chronological_time": req.chronological_time
    })))
}

fn compute_d5_value(param: &ProteostasisParameter, n: f64, t: f64) -> f64 {
    let n_term = if param.n5_critical > 0.0 {
        param.alpha5 * (n / param.n5_critical)
    } else {
        0.0
    };
    
    let t_term = if param.tau5 > 0.0 {
        param.beta5 * (t / param.tau5)
    } else {
        0.0
    };
    
    param.d50 + n_term + t_term
}