use super::{exam_in::ExamIn, grading_scale::GradingScale};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ExamOut {
    pub id: String,
    pub name: String,
    pub maxPoints: f64,
    pub gradingScale: GradingScale,
}

impl From<ExamIn> for ExamOut {
    fn from(exam: ExamIn) -> Self {
        Self {
            id: "".to_owned(),
            name: exam.name,
            maxPoints: exam.maxPoints,
            gradingScale: exam.gradingScale,
        }
    }
}

impl From<Document> for ExamOut {
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
