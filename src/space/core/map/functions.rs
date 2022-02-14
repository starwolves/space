#[derive(Clone, PartialEq)]
pub enum OverlayTile {
    Green,
    Yellow,
    Orange,
    Red,
}

pub fn get_overlay_tile_priority(tile : &OverlayTile) -> u8 {

    match tile {
        OverlayTile::Green => {0},
        OverlayTile::Yellow => {1},
        OverlayTile::Orange => {2},
        OverlayTile::Red => {3},
    }

}

pub fn get_overlay_tile_item(tile : &OverlayTile) -> i16 {

    match tile {
        OverlayTile::Green => {
            0
        },
        OverlayTile::Yellow => {
            3
        },
        OverlayTile::Orange => {
            1
        },
        OverlayTile::Red => {
            2
        },
    }

}
