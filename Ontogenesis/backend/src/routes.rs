use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{Path, State, Json},
};
use serde::Deserialize;
use uuid::Uuid;
use tracing::{info, warn};

use crate::{
    db::DbPool,
    models::*,
    error::AppError,
};

pub fn create_routes() -> Router<DbPool> {
    Router::new()
        // Phase endpoints
        .route("/phases", get(list_phases))
        .route("/phases", post(create_phase))
        .route("/phases/:id", get(get_phase))
        .route("/phases/:id", put(update_phase))
        .route("/phases/:id", delete(delete_phase))
        
        // Domain endpoints
        .route("/domains", get(list_domains))
        .route("/domains", post(create_domain))
        .route("/domains/:id", get(get_domain))
        .route("/domains/:id", put(update_domain))
        .route("/domains/:id", delete(delete_domain))
        
        // Parameter endpoints
        .route("/parameters", get(list_parameters))
        .route("/parameters", post(create_parameter))
        .route("/parameters/:id", get(get_parameter))
        .route("/parameters/:id", put(update_parameter))
        .route("/parameters/:id", delete(delete_parameter))
        
        // Individual endpoints
        .route("/individuals", get(list_individuals))
        .route("/individuals", post(create_individual))
        .route("/individuals/:id", get(get_individual))
        .route("/individuals/:id", put(update_individual))
        .route("/individuals/:id", delete(delete_individual))
        
        // Transition endpoints
        .route("/transitions", get(list_transitions))
        .route("/transitions", post(create_transition))
        .route("/transitions/:id", get(get_transition))
        .route("/transitions/:id", put(update_transition))
        .route("/transitions/:id", delete(delete_transition))
        
        // Metamorphosis endpoints
        .route("/metamorphoses", get(list_metamorphoses))
        .route("/metamorphoses", post(create_metamorphosis))
        .route("/metamorphoses/:id", get(get_metamorphosis))
        .route("/metamorphoses/:id", put(update_metamorphosis))
        .route("/metamorphoses/:id", delete(delete_metamorphosis))
        
        // LCS computation endpoint
        .route("/compute-lcs", post(compute_lcs))
        
        // Metamorphosis detection endpoint
        .route("/detect-metamorphoses", post(detect_metamorphoses))
}

// Phase handlers
async fn list_phases(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Phase>>, AppError> {
    info!("Listing all phases");
    let phases = sqlx::query_as::<_, Phase>("SELECT * FROM phases ORDER BY age_start")
        .fetch_all(&pool)
        .await?;
    Ok(Json(phases))
}

async fn create_phase(
    State(pool): State<DbPool>,
    Json(phase): Json<CreatePhase>,
) -> Result<Json<Phase>, AppError> {
    info!("Creating new phase: {:?}", phase.name);
    let created = sqlx::query_as::<_, Phase>(
        "INSERT INTO phases (name, description, age_start, age_end, neurobio_characteristics) 
         VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&phase.name)
    .bind(&phase.description)
    .bind(phase.age_start)
    .bind(phase.age_end)
    .bind(&phase.neurobio_characteristics)
    .fetch_one(&pool)
    .await?;
    Ok(Json(created))
}

async fn get_phase(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Phase>, AppError> {
    info!("Getting phase with id: {}", id);
    let phase = sqlx::query_as::<_, Phase>("SELECT * FROM phases WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Phase with id {} not found", id)))?;
    Ok(Json(phase))
}

async fn update_phase(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdatePhase>,
) -> Result<Json<Phase>, AppError> {
    info!("Updating phase with id: {}", id);
    let updated = sqlx::query_as::<_, Phase>(
        "UPDATE phases SET 
         name = COALESCE($1, name),
         description = COALESCE($2, description),
         age_start = COALESCE($3, age_start),
         age_end = COALESCE($4, age_end),
         neurobio_characteristics = COALESCE($5, neurobio_characteristics),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $6 RETURNING *"
    )
    .bind(update.name)
    .bind(update.description)
    .bind(update.age_start)
    .bind(update.age_end)
    .bind(update.neurobio_characteristics)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Phase with id {} not found", id)))?;
    Ok(Json(updated))
}

async fn delete_phase(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    info!("Deleting phase with id: {}", id);
    let result = sqlx::query("DELETE FROM phases WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Phase with id {} not found", id)));
    }
    
    Ok(Json(()))
}

// Domain handlers
async fn list_domains(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Domain>>, AppError> {
    let domains = sqlx::query_as::<_, Domain>("SELECT * FROM domains ORDER BY name")
        .fetch_all(&pool)
        .await?;
    Ok(Json(domains))
}

async fn create_domain(
    State(pool): State<DbPool>,
    Json(domain): Json<CreateDomain>,
) -> Result<Json<Domain>, AppError> {
    let created = sqlx::query_as::<_, Domain>(
        "INSERT INTO domains (name, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&domain.name)
    .bind(&domain.description)
    .fetch_one(&pool)
    .await?;
    Ok(Json(created))
}

async fn get_domain(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Domain>, AppError> {
    let domain = sqlx::query_as::<_, Domain>("SELECT * FROM domains WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Domain with id {} not found", id)))?;
    Ok(Json(domain))
}

async fn update_domain(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateDomain>,
) -> Result<Json<Domain>, AppError> {
    let updated = sqlx::query_as::<_, Domain>(
        "UPDATE domains SET 
         name = COALESCE($1, name),
         description = COALESCE($2, description),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $3 RETURNING *"
    )
    .bind(update.name)
    .bind(update.description)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Domain with id {} not found", id)))?;
    Ok(Json(updated))
}

async fn delete_domain(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query("DELETE FROM domains WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Domain with id {} not found", id)));
    }
    
    Ok(Json(()))
}

// Parameter handlers
async fn list_parameters(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Parameter>>, AppError> {
    let params = sqlx::query_as::<_, Parameter>(
        "SELECT p.*, d.name as domain_name FROM parameters p 
         JOIN domains d ON p.domain_id = d.id 
         ORDER BY p.domain_id, p.name"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(params))
}

async fn create_parameter(
    State(pool): State<DbPool>,
    Json(param): Json<CreateParameter>,
) -> Result<Json<Parameter>, AppError> {
    let created = sqlx::query_as::<_, Parameter>(
        "INSERT INTO parameters (
         domain_id, name, description, default_value, unit, 
         source, status, min_value, max_value, gamma, is_scaffold
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
        RETURNING *"
    )
    .bind(param.domain_id)
    .bind(&param.name)
    .bind(&param.description)
    .bind(param.default_value)
    .bind(&param.unit)
    .bind(&param.source)
    .bind(&param.status)
    .bind(param.min_value)
    .bind(param.max_value)
    .bind(param.gamma.unwrap_or(decimal::Decimal::ZERO)) // γ_i = 0 by default per CORRECTIONS
    .bind(param.is_scaffold.unwrap_or(false))
    .fetch_one(&pool)
    .await?;
    Ok(Json(created))
}

async fn get_parameter(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Parameter>, AppError> {
    let param = sqlx::query_as::<_, Parameter>(
        "SELECT p.*, d.name as domain_name FROM parameters p 
         JOIN domains d ON p.domain_id = d.id 
         WHERE p.id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Parameter with id {} not found", id)))?;
    Ok(Json(param))
}

async fn update_parameter(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateParameter>,
) -> Result<Json<Parameter>, AppError> {
    let updated = sqlx::query_as::<_, Parameter>(
        "UPDATE parameters SET 
         name = COALESCE($1, name),
         description = COALESCE($2, description),
         default_value = COALESCE($3, default_value),
         unit = COALESCE($4, unit),
         source = COALESCE($5, source),
         status = COALESCE($6, status),
         min_value = COALESCE($7, min_value),
         max_value = COALESCE($8, max_value),
         gamma = COALESCE($9, gamma),
         is_scaffold = COALESCE($10, is_scaffold),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $11 RETURNING *"
    )
    .bind(update.name)
    .bind(update.description)
    .bind(update.default_value)
    .bind(update.unit)
    .bind(update.source)
    .bind(update.status)
    .bind(update.min_value)
    .bind(update.max_value)
    .bind(update.gamma)
    .bind(update.is_scaffold)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Parameter with id {} not found", id)))?;
    Ok(Json(updated))
}

async fn delete_parameter(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query("DELETE FROM parameters WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Parameter with id {} not found", id)));
    }
    
    Ok(Json(()))
}

// Individual handlers
async fn list_individuals(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Individual>>, AppError> {
    let individuals = sqlx::query_as::<_, Individual>(
        "SELECT i.*, 
         COUNT(DISTINCT t.id) as transition_count,
         COUNT(DISTINCT m.id) as metamorphosis_count
         FROM individuals i
         LEFT JOIN transitions t ON i.id = t.individual_id
         LEFT JOIN metamorphoses m ON i.id = m.individual_id
         GROUP BY i.id
         ORDER BY i.created_at DESC"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(individuals))
}

async fn create_individual(
    State(pool): State<DbPool>,
    Json(individual): Json<CreateIndividual>,
) -> Result<Json<Individual>, AppError> {
    let created = sqlx::query_as::<_, Individual>(
        "INSERT INTO individuals (
         cohort_id, birth_date, sex, simulated, initial_age, max_age
        ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(individual.cohort_id)
    .bind(individual.birth_date)
    .bind(individual.sex)
    .bind(individual.simulated.unwrap_or(false))
    .bind(individual.initial_age.unwrap_or(0.0))
    .bind(individual.max_age.unwrap_or(120.0))
    .fetch_one(&pool)
    .await?;
    Ok(Json(created))
}

async fn get_individual(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Individual>, AppError> {
    let individual = sqlx::query_as::<_, Individual>(
        "SELECT i.*, 
         COUNT(DISTINCT t.id) as transition_count,
         COUNT(DISTINCT m.id) as metamorphosis_count
         FROM individuals i
         LEFT JOIN transitions t ON i.id = t.individual_id
         LEFT JOIN metamorphoses m ON i.id = m.individual_id
         WHERE i.id = $1
         GROUP BY i.id"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Individual with id {} not found", id)))?;
    Ok(Json(individual))
}

async fn update_individual(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateIndividual>,
) -> Result<Json<Individual>, AppError> {
    let updated = sqlx::query_as::<_, Individual>(
        "UPDATE individuals SET 
         cohort_id = COALESCE($1, cohort_id),
         birth_date = COALESCE($2, birth_date),
         sex = COALESCE($3, sex),
         simulated = COALESCE($4, simulated),
         initial_age = COALESCE($5, initial_age),
         max_age = COALESCE($6, max_age),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $7 RETURNING *"
    )
    .bind(update.cohort_id)
    .bind(update.birth_date)
    .bind(update.sex)
    .bind(update.simulated)
    .bind(update.initial_age)
    .bind(update.max_age)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Individual with id {} not found", id)))?;
    Ok(Json(updated))
}

async fn delete_individual(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query("DELETE FROM individuals WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Individual with id {} not found", id)));
    }
    
    Ok(Json(()))
}

// Transition handlers
async fn list_transitions(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Transition>>, AppError> {
    let transitions = sqlx::query_as::<_, Transition>(
        "SELECT t.*, p.name as parameter_name, d.name as domain_name
         FROM transitions t
         JOIN parameters p ON t.parameter_id = p.id
         JOIN domains d ON p.domain_id = d.id
         ORDER BY t.individual_id, t.age"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(transitions))
}

async fn create_transition(
    State(pool): State<DbPool>,
    Json(transition): Json<CreateTransition>,
) -> Result<Json<Transition>, AppError> {
    let created = sqlx::query_as::<_, Transition>(
        "INSERT INTO transitions (
         individual_id, parameter_id, age, value, change_score,
         previous_value, domain_coupling, is_significant, fdr_q_value
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(transition.individual_id)
    .bind(transition.parameter_id)
    .bind(transition.age)
    .bind(transition.value)
    .bind(transition.change_score)
    .bind(transition.previous_value)
    .bind(transition.domain_coupling)
    .bind(transition.is_significant.unwrap_or(false))
    .bind(transition.fdr_q_value)
    .fetch_one(&pool)
    .await?;
    Ok(Json(created))
}

async fn get_transition(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Transition>, AppError> {
    let transition = sqlx::query_as::<_, Transition>(
        "SELECT t.*, p.name as parameter_name, d.name as domain_name
         FROM transitions t
         JOIN parameters p ON t.parameter_id = p.id
         JOIN domains d ON p.domain_id = d.id
         WHERE t.id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Transition with id {} not found", id)))?;
    Ok(Json(transition))
}

async fn update_transition(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateTransition>,
) -> Result<Json<Transition>, AppError> {
    let updated = sqlx::query_as::<_, Transition>(
        "UPDATE transitions SET 
         age = COALESCE($1, age),
         value = COALESCE($2, value),
         change_score = COALESCE($3, change_score),
         previous_value = COALESCE($4, previous_value),
         domain_coupling = COALESCE($5, domain_coupling),
         is_significant = COALESCE($6, is_significant),
         fdr_q_value = COALESCE($7, fdr_q_value),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $8 RETURNING *"
    )
    .bind(update.age)
    .bind(update.value)
    .bind(update.change_score)
    .bind(update.previous_value)
    .bind(update.domain_coupling)
    .bind(update.is_significant)
    .bind(update.fdr_q_value)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Transition with id {} not found", id)))?;
    Ok(Json(updated))
}

async fn delete_transition(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query("DELETE FROM transitions WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Transition with id {} not found", id)));
    }
    
    Ok(Json(()))
}

// Metamorphosis handlers
async fn list_metamorphoses(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Metamorphosis>>, AppError> {
    let metamorphoses = sqlx::query_as::<_, Metamorphosis>(
        "SELECT m.*, 
         COUNT(DISTINCT mt.transition_id) as transition_count,
         COUNT(DISTINCT d.id) as domain_count
         FROM metamorphoses m
         LEFT JOIN metamorphosis_transitions mt ON m.id = mt.metamorphosis_id
         LEFT JOIN transitions t ON mt.transition_id = t.id
         LEFT JOIN parameters p ON t.parameter_id = p.id
         LEFT JOIN domains d ON p.domain_id = d.id
         GROUP BY m.id
         ORDER BY m.individual_id, m.age"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(metamorphoses))
}

async fn create_metamorphosis(
    State(pool): State<DbPool>,
    Json(metamorphosis): Json<CreateMetamorphosis>,
) -> Result<Json<Metamorphosis>, AppError> {
    let created = sqlx::query_as::<_, Metamorphosis>(
        "INSERT INTO metamorphoses (
         individual_id, age, cluster_radius, domain_count, 
         is_valid, fdr_corrected
        ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(metamorphosis.individual_id)
    .bind(metamorphosis.age)
    .bind(metamorphosis.cluster_radius.unwrap_or(6.0)) // 6 months default
    .bind(metamorphosis.domain_count)
    .bind(metamorphosis.is_valid.unwrap_or(true))
    .bind(metamorphosis.fdr_corrected.unwrap_or(true))
    .fetch_one(&pool)
    .await?;
    
    // Insert transition associations if provided
    if let Some(transition_ids) = metamorphosis.transition_ids {
        for transition_id in transition_ids {
            sqlx::query(
                "INSERT INTO metamorphosis_transitions (metamorphosis_id, transition_id) 
                 VALUES ($1, $2)"
            )
            .bind(created.id)
            .bind(transition_id)
            .execute(&pool)
            .await?;
        }
    }
    
    Ok(Json(created))
}

async fn get_metamorphosis(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Metamorphosis>, AppError> {
    let metamorphosis = sqlx::query_as::<_, Metamorphosis>(
        "SELECT m.*, 
         COUNT(DISTINCT mt.transition_id) as transition_count,
         COUNT(DISTINCT d.id) as domain_count
         FROM metamorphoses m
         LEFT JOIN metamorphosis_transitions mt ON m.id = mt.metamorphosis_id
         LEFT JOIN transitions t ON mt.transition_id = t.id
         LEFT JOIN parameters p ON t.parameter_id = p.id
         LEFT JOIN domains d ON p.domain_id = d.id
         WHERE m.id = $1
         GROUP BY m.id"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Metamorphosis with id {} not found", id)))?;
    Ok(Json(metamorphosis))
}

async fn update_metamorphosis(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateMetamorphosis>,
) -> Result<Json<Metamorphosis>, AppError> {
    let updated = sqlx::query_as::<_, Metamorphosis>(
        "UPDATE metamorphoses SET 
         age = COALESCE($1, age),
         cluster_radius = COALESCE($2, cluster_radius),
         domain_count = COALESCE($3, domain_count),
         is_valid = COALESCE($4, is_valid),
         fdr_corrected = COALESCE($5, fdr_corrected),
         updated_at = CURRENT_TIMESTAMP
         WHERE id = $6 RETURNING *"
    )
    .bind(update.age)
    .bind(update.cluster_radius)
    .bind(update.domain_count)
    .bind(update.is_valid)
    .bind(update.fdr_corrected)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Metamorphosis with id {} not found", id)))?;
    Ok(Json(updated))
}

async fn delete_metamorphosis(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query("DELETE FROM metamorphoses WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Metamorphosis with id {} not found", id)));
    }
    
    Ok(Json(()))
}

// LCS computation endpoint
#[derive(Debug, Deserialize)]
struct LcsRequest {
    individual_id: Uuid,
    parameter_ids: Vec<Uuid>,
    start_age: f64,
    end_age: f64,
    coupling_gamma: Option<decimal::Decimal>,
}

async fn compute_lcs(
    State(pool): State<DbPool>,
    Json(req): Json<LcsRequest>,
) -> Result<Json<Vec<Transition>>, AppError> {
    info!("Computing LCS for individual {}, parameters: {:?}", 
          req.individual_id, req.parameter_ids);
    
    // This would implement the LCS (Latent Change Score) computation
    // For now, return empty array as placeholder
    warn!("LCS computation not yet implemented - returning placeholder");
    Ok(Json(vec![]))
}

// Metamorphosis detection endpoint
#[derive(Debug, Deserialize)]
struct MetamorphosisDetectionRequest {
    individual_id: Uuid,
    window_months: Option<f64>, // Default 6 months
    min_domains: Option<i32>, // Default 2
    fdr_threshold: Option<decimal::Decimal>, // Default 0.05
}

async fn detect_metamorphoses(
    State(pool): State<DbPool>,
    Json(req): Json<MetamorphosisDetectionRequest>,
) -> Result<Json<Vec<Metamorphosis>>, AppError> {
    info!("Detecting metamorphoses for individual {}", req.individual_id);
    
    // This would implement the metamorphosis detection algorithm
    // For now, return empty array as placeholder
    warn!("Metamorphosis detection not yet implemented - returning placeholder");
    Ok(Json(vec![]))
}