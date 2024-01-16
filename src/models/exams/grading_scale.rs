use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GradingScale {
    pub A: f64,
    pub B: f64,
    pub C: f64,
    pub D: f64,
    pub E: f64,
    pub F: f64,
}
