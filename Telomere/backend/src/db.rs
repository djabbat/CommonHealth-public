use sqlx::{postgres::PgPoolOptions, PgPool, FromRow, Row, postgres::PgRow};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    models::{
        TelomereMeasurement, TelomereParameters,
        MeasurementQuery, Pagination,
    },
    error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    // TelomereMeasurement operations
    pub async fn get_measurements(
        &self,
        query: &MeasurementQuery,
        pagination: &Pagination,
    ) -> AppResult<Vec<TelomereMeasurement>> {
        let mut sql = String::from("SELECT * FROM telomere_measurements WHERE 1=1");
        let mut conditions = Vec::new();
        let mut args: Vec<&(dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync)> = Vec::new();
        let mut arg_idx = 1;

        if let Some(subject_id) = query.subject_id {
            conditions.push(format!("subject_id = ${}", arg_idx));
            args.push(&subject_id);
            arg_idx += 1;
        }

        if let Some(sample_id) = &query.sample_id {
            conditions.push(format!("sample_id = ${}", arg_idx));
            args.push(&sample_id);
            arg_idx += 1;
        }

        if let Some(start_date) = query.start_date {
            conditions.push(format!("measured_at >= ${}", arg_idx));
            args.push(&start_date);
            arg_idx += 1;
        }

        if let Some(end_date) = query.end_date {
            conditions.push(format!("measured_at <= ${}", arg_idx));
            args.push(&end_date);
            arg_idx += 1;
        }

        if let Some(min_length) = query.min_length {
            conditions.push(format!("telomere_length_bp >= ${}", arg_idx));
            args.push(&min_length);
            arg_idx += 1;
        }

        if let Some(max_length) = query.max_length {
            conditions.push(format!("telomere_length_bp <= ${}", arg_idx));
            args.push(&max_length);
            arg_idx += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        if let Some(order_by) = &pagination.order_by {
            sql.push_str(&format!(" ORDER BY {}", order_by));
        } else {
            sql.push_str(" ORDER BY measured_at DESC");
        }

        if let Some(limit) = pagination.limit {
            sql.push_str(&format!(" LIMIT ${}", arg_idx));
            args.push(&limit);
            arg_idx += 1;
        }

        if let Some(offset) = pagination.offset {
            sql.push_str(&format!(" OFFSET ${}", arg_idx));
            args.push(&offset);
        }

        let query = sqlx::query_as::<_, TelomereMeasurement>(&sql);
        
        let measurements = query
            .bind_all(args)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(measurements)
    }

    pub async fn get_measurement(&self, id: Uuid) -> AppResult<TelomereMeasurement> {
        let measurement = sqlx::query_as::<_, TelomereMeasurement>(
            "SELECT * FROM telomere_measurements WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Measurement with id {} not found", id)))?;

        Ok(measurement)
    }

    pub async fn create_measurement(&self, mut measurement: TelomereMeasurement) -> AppResult<TelomereMeasurement> {
        measurement.id = Uuid::new_v4();
        measurement.created_at = Utc::now();
        measurement.updated_at = Utc::now();

        let result = sqlx::query_as::<_, TelomereMeasurement>(
            r#"
            INSERT INTO telomere_measurements (
                id, subject_id, sample_id, measured_at, telomere_length_bp,
                telomere_deficit_bp, population_doublings, time_elapsed_years,
                oxidative_stress_marker, shelterin_expression, telomerase_activity,
                measurement_method, metadata, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(measurement.id)
        .bind(measurement.subject_id)
        .bind(measurement.sample_id)
        .bind(measurement.measured_at)
        .bind(measurement.telomere_length_bp)
        .bind(measurement.telomere_deficit_bp)
        .bind(measurement.population_doublings)
        .bind(measurement.time_elapsed_years)
        .bind(measurement.oxidative_stress_marker)
        .bind(measurement.shelterin_expression)
        .bind(measurement.telomerase_activity)
        .bind(measurement.measurement_method)
        .bind(measurement.metadata)
        .bind(measurement.created_at)
        .bind(measurement.updated_at)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    pub async fn update_measurement(&self, measurement: TelomereMeasurement) -> AppResult<TelomereMeasurement> {
        let updated_at = Utc::now();

        let result = sqlx::query_as::<_, TelomereMeasurement>(
            r#"
            UPDATE telomere_measurements SET
                subject_id = $2,
                sample_id = $3,
                measured_at = $4,
                telomere_length_bp = $5,
                telomere_deficit_bp = $6,
                population_doublings = $7,
                time_elapsed_years = $8,
                oxidative_stress_marker = $9,
                shelterin_expression = $10,
                telomerase_activity = $11,
                measurement_method = $12,
                metadata = $13,
                updated_at = $14
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(measurement.id)
        .bind(measurement.subject_id)
        .bind(measurement.sample_id)
        .bind(measurement.measured_at)
        .bind(measurement.telomere_length_bp)
        .bind(measurement.telomere_deficit_bp)
        .bind(measurement.population_doublings)
        .bind(measurement.time_elapsed_years)
        .bind(measurement.oxidative_stress_marker)
        .bind(measurement.shelterin_expression)
        .bind(measurement.telomerase_activity)
        .bind(measurement.measurement_method)
        .bind(measurement.metadata)
        .bind(updated_at)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Measurement with id {} not found", measurement.id)))?;

        Ok(result)
    }

    pub async fn delete_measurement(&self, id: Uuid) -> AppResult<()> {
        let rows_affected = sqlx::query("DELETE FROM telomere_measurements WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Measurement with id {} not found", id)));
        }

        Ok(())
    }

    // TelomereParameters operations
    pub async fn get_parameters(&self, pagination: &Pagination) -> AppResult<Vec<TelomereParameters>> {
        let mut sql = String::from("SELECT * FROM telomere_parameters");
        
        if let Some(order_by) = &pagination.order_by {
            sql.push_str(&format!(" ORDER BY {}", order_by));
        } else {
            sql.push_str(" ORDER BY created_at DESC");
        }

        if let Some(limit) = pagination.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = pagination.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let parameters = sqlx::query_as::<_, TelomereParameters>(&sql)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(parameters)
    }

    pub async fn get_parameters_by_id(&self, id: Uuid) -> AppResult<TelomereParameters> {
        let parameters = sqlx::query_as::<_, TelomereParameters>(
            "SELECT * FROM telomere_parameters WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Parameters with id {} not found", id)))?;

        Ok(parameters)
    }

    pub async fn get_parameters_by_subject(&self, subject_id: Uuid) -> AppResult<TelomereParameters> {
        // First try to get subject-specific parameters
        let parameters = sqlx::query_as::<_, TelomereParameters>(
            "SELECT * FROM telomere_parameters WHERE subject_id = $1 ORDER BY created_at DESC LIMIT 1"
        )
        .bind(subject_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(params) = parameters {
            return Ok(params);
        }

        // Fall back to default parameters
        let default_params = sqlx::query_as::<_, TelomereParameters>(
            "SELECT * FROM telomere_parameters WHERE is_default = true ORDER BY created_at DESC LIMIT 1"
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        default_params.ok_or_else(|| {
            AppError::NotFound("No parameters found (default or subject-specific)".to_string())
        })
    }

    pub async fn create_parameters(&self, mut parameters: TelomereParameters) -> AppResult<TelomereParameters> {
        parameters.id = Uuid::new_v4();
        parameters.created_at = Utc::now();
        parameters.updated_at = Utc::now();

        // Ensure only one default set exists
        if parameters.is_default {
            sqlx::query("UPDATE telomere_parameters SET is_default = false WHERE is_default = true")
                .execute(&*self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        let result = sqlx::query_as::<_, TelomereParameters>(
            r#"
            INSERT INTO telomere_parameters (
                id, subject_id, d2_baseline, alpha2, beta2, n2_star, tau2,
                gamma_21, gamma_23, gamma_24, gamma_25, is_default, notes,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(parameters.id)
        .bind(parameters.subject_id)
        .bind(parameters.d2_baseline)
        .bind(parameters.alpha2)
        .bind(parameters.beta2)
        .bind(parameters.n2_star)
        .bind(parameters.tau2)
        .bind(parameters.gamma_21)
        .bind(parameters.gamma_23)
        .bind(parameters.gamma_24)
        .bind(parameters.gamma_25)
        .bind(parameters.is_default)
        .bind(parameters.notes)
        .bind(parameters.created_at)
        .bind(parameters.updated_at)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    pub async fn update_parameters(&self, parameters: TelomereParameters) -> AppResult<TelomereParameters> {
        let updated_at = Utc::now();

        // If setting as default, clear other defaults
        if parameters.is_default {
            sqlx::query("UPDATE telomere_parameters SET is_default = false WHERE is_default = true AND id != $1")
                .bind(parameters.id)
                .execute(&*self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        let result = sqlx::query_as::<_, TelomereParameters>(
            r#"
            UPDATE telomere_parameters SET
                subject_id = $2,
                d2_baseline = $3,
                alpha2 = $4,
                beta2 = $5,
                n2_star = $6,
                tau2 = $7,
                gamma_21 = $8,
                gamma_23 = $9,
                gamma_24 = $10,
                gamma_25 = $11,
                is_default = $12,
                notes = $13,
                updated_at = $14
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(parameters.id)
        .bind(parameters.subject_id)
        .bind(parameters.d2_baseline)
        .bind(parameters.alpha2)
        .bind(parameters.beta2)
        .bind(parameters.n2_star)
        .bind(parameters.tau2)
        .bind(parameters.gamma_21)
        .bind(parameters.gamma_23)
        .bind(parameters.gamma_24)
        .bind(parameters.gamma_25)
        .bind(parameters.is_default)
        .bind(parameters.notes)
        .bind(updated_at)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Parameters with id {} not found", parameters.id)))?;

        Ok(result)
    }

    pub async fn delete_parameters(&self, id: Uuid) -> AppResult<()> {
        let rows_affected = sqlx::query("DELETE FROM telomere_parameters WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Parameters with id {} not found", id)));
        }

        Ok(())
    }
}