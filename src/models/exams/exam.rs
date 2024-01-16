use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Percent {
    pub value: f64,
}

impl From<f64> for Percent {
    fn from(value: f64) -> Self {
        if value < 0. || 100. < value {
            panic!("Unable to converet");
        }
        Self { value }
    }
}

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

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Exam {
    pub name: String,
    pub maxPoints: f64,
    pub gradingScale: GradingScale,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ExamResponse {
    pub id: String,
    pub name: String,
    pub maxPoints: f64,
    pub gradingScale: GradingScale,
}

impl From<Exam> for ExamResponse {
    fn from(exam: Exam) -> Self {
        Self {
            id: "".to_owned(),
            name: exam.name,
            maxPoints: exam.maxPoints,
            gradingScale: exam.gradingScale,
        }
    }
}

impl From<Document> for ExamResponse {
    fn from(doc: Document) -> Self {
        let id = doc.get_object_id("_id").unwrap().to_hex();
        let name = doc.get_str("name").unwrap().to_string();
        let max_points = doc.get_f64("maxPoints").unwrap();
        let grading_scale = doc.get_document("gradingScale").unwrap();

        Self {
            id,
            name,
            maxPoints: max_points,
            gradingScale: GradingScale {
                A: grading_scale.get_f64("A").unwrap(),
                B: grading_scale.get_f64("B").unwrap(),
                C: grading_scale.get_f64("C").unwrap(),
                D: grading_scale.get_f64("D").unwrap(),
                E: grading_scale.get_f64("E").unwrap(),
                F: grading_scale.get_f64("F").unwrap(),
            },
        }
    }
}
