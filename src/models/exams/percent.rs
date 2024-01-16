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
