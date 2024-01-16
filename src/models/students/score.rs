use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Score {
    pub pointsScored: f64,
    pub isPostscript: bool,
}
