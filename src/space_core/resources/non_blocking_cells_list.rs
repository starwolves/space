use serde::{Deserialize};

#[derive(Deserialize)]
pub struct NonBlockingCellsList {
    pub list: Vec<i64>
}
