use super::grading_scale::GradingScale;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ExamIn {
    pub name: String,
    pub maxPoints: f64,
    pub gradingScale: GradingScale,
}
