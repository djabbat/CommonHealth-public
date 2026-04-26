// Ontogenesis v4.1 — src/params.rs
// All numeric parameters — single source of truth (mirrors PARAMETERS.md)

/// All numeric parameters for Ontogenesis v4.1
/// Source: PARAMETERS.md | Do not change without updating PARAMETERS.md
#[derive(Debug, Clone)]
pub struct OntogenesisParams {
    // § Transition Detection — General
    /// Age step in months for the algorithm sweep
    pub age_step: u32,           // 1 month
    /// Cluster radius in months for merging nearby transition marks
    pub cluster_radius: u32,     // 6 months
    /// Minimum sample size for CV/Range calculations
    pub min_sample_size: usize,  // 30 individuals
    /// Minimum stable period before detecting a transition (months)
    pub stable_period_min: u32,  // 3 months

    // § Anatomical transitions (longitudinal data)
    /// Threshold: change > N × SD from individual trajectory
    pub anat_threshold_sd: f64,   // 2.0 SD
    /// Stable fraction threshold: change interval < X% of previous stable period
    pub anat_stable_fraction: f64, // 0.05 (5%)
    /// Cross-sectional threshold multiplier: CV(A,t) > CV_mean + N × SD_CV
    pub anat_cross_sd_mult: f64,   // 2.0

    // § Endocrine transitions (longitudinal data)
    /// Threshold: change > N × SD (stricter than anatomical)
    pub endo_threshold_sd: f64,    // 3.0 SD
    /// Stable fraction threshold
    pub endo_stable_fraction: f64, // 0.10 (10%)
    /// Cross-sectional Range threshold multiplier
    pub endo_cross_sd_mult: f64,   // 2.0

    // § Age grid
    /// Minimum age in months
    pub age_min_months: u32,       // 0
    /// Maximum age in months (25 years)
    pub age_max_months: u32,       // 300
}

impl Default for OntogenesisParams {
    fn default() -> Self {
        Self {
            age_step: 1,
            cluster_radius: 6,
            min_sample_size: 30,
            stable_period_min: 3,
            anat_threshold_sd: 2.0,
            anat_stable_fraction: 0.05,
            anat_cross_sd_mult: 2.0,
            endo_threshold_sd: 3.0,
            endo_stable_fraction: 0.10,
            endo_cross_sd_mult: 2.0,
            age_min_months: 0,
            age_max_months: 300,
        }
    }
}
