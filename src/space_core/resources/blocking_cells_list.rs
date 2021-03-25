use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BlockingCellsList {
    pub list: Vec<i64>
}
