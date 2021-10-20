use crate::space_core::resources::{gridmap_main::CellData};

const NAMES_MAIN : [&str;14] = [
    "an aluminum floor",
    "an aluminum wall",
    "INVISIBLECELL",
    "an aluminum floor",
    "an aluminum wall",
    "an aluminum floor",
    "an aluminum floor",
    "an aluminum floor",
    "an aluminum security counter",
    "an aluminum wall",
    "an aluminum wall",
    "an aluminum wall",
    "INVISIBLECELL2",
    "a decorated security table"
];

pub fn get_cell_name(
    ship_cell : &CellData,
) -> String {

    NAMES_MAIN[ship_cell.item as usize].to_string()

}
