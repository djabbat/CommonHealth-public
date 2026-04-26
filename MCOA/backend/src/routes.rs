use axum::{
    routing::{get, post, put, delete},
    Router, extract::{Path, State}, Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    error::AppResult,
    models::{
        Counter, CreateCounter, UpdateCounter,
        Tissue, CreateTissue, UpdateTissue,
        Subject, CreateSubject, UpdateSubject,
        DamageMeasurement, CreateDamageMeasurement, UpdateDamageMeasurement,
        TissueLoad, CreateTissueLoad, ComputeTissueLoad,
        CouplingMatrixEntry, CreateCouplingMatrixEntry, UpdateCouplingMatrixEntry,
    },
    db::DbPool,
};

// Counter routes
pub fn counter_routes() -> Router<DbPool> {
    Router::new()
        .route("/counters", get(list_counters).post(create_counter))
        .route("/counters/:id", get(get_counter).put(update_counter).delete(delete_counter))
}

async fn list_counters(State(pool): State<DbPool>) -> AppResult<Json<Vec<Counter>>> {
    let counters = Counter::list(&pool).await?;
    Ok(Json(counters))
}

async fn get_counter(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<Json<Counter>> {
    let counter = Counter::find_by_id(&pool, id).await?;
    Ok(Json(counter))
}

async fn create_counter(State(pool): State<DbPool>, Json(payload): Json<CreateCounter>) -> AppResult<Json<Counter>> {
    let counter = Counter::create(&pool, payload).await?;
    Ok(Json(counter))
}

async fn update_counter(State(pool): State<DbPool>, Path(id): Path<Uuid>, Json(payload): Json<UpdateCounter>) -> AppResult<Json<Counter>> {
    let counter = Counter::update(&pool, id, payload).await?;
    Ok(Json(counter))
}

async fn delete_counter(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<()> {
    Counter::delete(&pool, id).await?;
    Ok(())
}

// Tissue routes
pub fn tissue_routes() -> Router<DbPool> {
    Router::new()
        .route("/tissues", get(list_tissues).post(create_tissue))
        .route("/tissues/:id", get(get_tissue).put(update_tissue).delete(delete_tissue))
}

async fn list_tissues(State(pool): State<DbPool>) -> AppResult<Json<Vec<Tissue>>> {
    let tissues = Tissue::list(&pool).await?;
    Ok(Json(tissues))
}

async fn get_tissue(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<Json<Tissue>> {
    let tissue = Tissue::find_by_id(&pool, id).await?;
    Ok(Json(tissue))
}

async fn create_tissue(State(pool): State<DbPool>, Json(payload): Json<CreateTissue>) -> AppResult<Json<Tissue>> {
    let tissue = Tissue::create(&pool, payload).await?;
    Ok(Json(tissue))
}

async fn update_tissue(State(pool): State<DbPool>, Path(id): Path<Uuid>, Json(payload): Json<UpdateTissue>) -> AppResult<Json<Tissue>> {
    let tissue = Tissue::update(&pool, id, payload).await?;
    Ok(Json(tissue))
}

async fn delete_tissue(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<()> {
    Tissue::delete(&pool, id).await?;
    Ok(())
}

// Subject routes
pub fn subject_routes() -> Router<DbPool> {
    Router::new()
        .route("/subjects", get(list_subjects).post(create_subject))
        .route("/subjects/:id", get(get_subject).put(update_subject).delete(delete_subject))
}

async fn list_subjects(State(pool): State<DbPool>) -> AppResult<Json<Vec<Subject>>> {
    let subjects = Subject::list(&pool).await?;
    Ok(Json(subjects))
}

async fn get_subject(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<Json<Subject>> {
    let subject = Subject::find_by_id(&pool, id).await?;
    Ok(Json(subject))
}

async fn create_subject(State(pool): State<DbPool>, Json(payload): Json<CreateSubject>) -> AppResult<Json<Subject>> {
    let subject = Subject::create(&pool, payload).await?;
    Ok(Json(subject))
}

async fn update_subject(State(pool): State<DbPool>, Path(id): Path<Uuid>, Json(payload): Json<UpdateSubject>) -> AppResult<Json<Subject>> {
    let subject = Subject::update(&pool, id, payload).await?;
    Ok(Json(subject))
}

async fn delete_subject(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<()> {
    Subject::delete(&pool, id).await?;
    Ok(())
}

// Damage Measurement routes
pub fn damage_measurement_routes() -> Router<DbPool> {
    Router::new()
        .route("/damage_measurements", get(list_damage_measurements).post(create_damage_measurement))
        .route("/damage_measurements/:id", get(get_damage_measurement).put(update_damage_measurement).delete(delete_damage_measurement))
        .route("/damage_measurements/subject/:subject_id", get(get_damage_measurements_by_subject))
}

async fn list_damage_measurements(State(pool): State<DbPool>) -> AppResult<Json<Vec<DamageMeasurement>>> {
    let measurements = DamageMeasurement::list(&pool).await?;
    Ok(Json(measurements))
}

async fn get_damage_measurement(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<Json<DamageMeasurement>> {
    let measurement = DamageMeasurement::find_by_id(&pool, id).await?;
    Ok(Json(measurement))
}

async fn create_damage_measurement(State(pool): State<DbPool>, Json(payload): Json<CreateDamageMeasurement>) -> AppResult<Json<DamageMeasurement>> {
    let measurement = DamageMeasurement::create(&pool, payload).await?;
    Ok(Json(measurement))
}

async fn update_damage_measurement(State(pool): State<DbPool>, Path(id): Path<Uuid>, Json(payload): Json<UpdateDamageMeasurement>) -> AppResult<Json<DamageMeasurement>> {
    let measurement = DamageMeasurement::update(&pool, id, payload).await?;
    Ok(Json(measurement))
}

async fn delete_damage_measurement(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<()> {
    DamageMeasurement::delete(&pool, id).await?;
    Ok(())
}

async fn get_damage_measurements_by_subject(State(pool): State<DbPool>, Path(subject_id): Path<Uuid>) -> AppResult<Json<Vec<DamageMeasurement>>> {
    let measurements = DamageMeasurement::find_by_subject(&pool, subject_id).await?;
    Ok(Json(measurements))
}

// Tissue Load routes
pub fn tissue_load_routes() -> Router<DbPool> {
    Router::new()
        .route("/tissue_loads", get(list_tissue_loads).post(create_tissue_load))
        .route("/tissue_loads/:id", get(get_tissue_load).delete(delete_tissue_load))
        .route("/tissue_loads/compute", post(compute_tissue_load))
}

async fn list_tissue_loads(State(pool): State<DbPool>) -> AppResult<Json<Vec<TissueLoad>>> {
    let loads = TissueLoad::list(&pool).await?;
    Ok(Json(loads))
}

async fn get_tissue_load(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<Json<TissueLoad>> {
    let load = TissueLoad::find_by_id(&pool, id).await?;
    Ok(Json(load))
}

async fn create_tissue_load(State(pool): State<DbPool>, Json(payload): Json<CreateTissueLoad>) -> AppResult<Json<TissueLoad>> {
    let load = TissueLoad::create(&pool, payload).await?;
    Ok(Json(load))
}

async fn delete_tissue_load(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<()> {
    TissueLoad::delete(&pool, id).await?;
    Ok(())
}

async fn compute_tissue_load(State(pool): State<DbPool>, Json(payload): Json<ComputeTissueLoad>) -> AppResult<Json<TissueLoad>> {
    let load = TissueLoad::compute(&pool, payload).await?;
    Ok(Json(load))
}

// Coupling Matrix routes
pub fn coupling_matrix_routes() -> Router<DbPool> {
    Router::new()
        .route("/coupling_matrix", get(list_coupling_matrix).post(create_coupling_matrix_entry))
        .route("/coupling_matrix/:id", get(get_coupling_matrix_entry).put(update_coupling_matrix_entry).delete(delete_coupling_matrix_entry))
}

async fn list_coupling_matrix(State(pool): State<DbPool>) -> AppResult<Json<Vec<CouplingMatrixEntry>>> {
    let entries = CouplingMatrixEntry::list(&pool).await?;
    Ok(Json(entries))
}

async fn get_coupling_matrix_entry(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<Json<CouplingMatrixEntry>> {
    let entry = CouplingMatrixEntry::find_by_id(&pool, id).await?;
    Ok(Json(entry))
}

async fn create_coupling_matrix_entry(State(pool): State<DbPool>, Json(payload): Json<CreateCouplingMatrixEntry>) -> AppResult<Json<CouplingMatrixEntry>> {
    let entry = CouplingMatrixEntry::create(&pool, payload).await?;
    Ok(Json(entry))
}

async fn update_coupling_matrix_entry(State(pool): State<DbPool>, Path(id): Path<Uuid>, Json(payload): Json<UpdateCouplingMatrixEntry>) -> AppResult<Json<CouplingMatrixEntry>> {
    let entry = CouplingMatrixEntry::update(&pool, id, payload).await?;
    Ok(Json(entry))
}

async fn delete_coupling_matrix_entry(State(pool): State<DbPool>, Path(id): Path<Uuid>) -> AppResult<()> {
    CouplingMatrixEntry::delete(&pool, id).await?;
    Ok(())
}