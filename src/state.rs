pub struct State {
    pub args: Vec<String>,

    pub iwad_found: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            args: vec![],
            iwad_found: false,
        }
    }
}
