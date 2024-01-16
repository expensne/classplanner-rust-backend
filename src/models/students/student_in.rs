use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::score::Score;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct StudentIn {
    pub firstName: String,
    pub lastName: String,
    pub scores: HashMap<String, Score>,
}
