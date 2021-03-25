use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BlackcellsData {
    pub blackcell_id : i64,
    pub blackcell_blocking_id : i64
}
