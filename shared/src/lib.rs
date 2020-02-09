extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Self { count: 0 }
    }
}