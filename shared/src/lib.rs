extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Self { count: 0 }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub reason: String,
}
