pub struct ConnectedPlayer {
    pub handle : u32,
    pub authid : u16,
    pub rcon : bool,
    pub connected : bool,
}

impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon : false,
            connected : true,
        }
    }
}
