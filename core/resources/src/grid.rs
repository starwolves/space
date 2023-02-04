use serde::{Deserialize, Serialize};

/// All six faces of the cell. Represents walls, ceilings and floors.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub enum CellFace {
    #[default]
    FrontWall,
    RightWall,
    BackWall,
    LeftWall,
    Floor,
    Ceiling,
}
