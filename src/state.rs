pub struct State {
    pub args: Vec<String>,

    pub iwad_found: bool,
    pub iwad_file: String,

    pub no_monsters: bool,
    pub respawn_parm: bool,
    pub fast_parm: bool,
    pub dev_parm: bool,

    pub display_fps_dots: bool,

    pub multiplayer_mode: crate::types::MultiplayerMode,

    pub config_dir: String,
    pub default_main_config: String,
    pub default_extra_config: String,

    pub forward_move: [crate::fixed::Fixed; 2],
    pub side_move: [crate::fixed::Fixed; 2],

    pub input_options: crate::options::InputOptions,
    pub video_options: crate::options::VideoOptions,
    pub joystick_options: crate::options::JoystickOptions,
    pub sound_options: crate::options::SoundOptions,
    pub controls: crate::options::Controls,

    pub doom_defaults: crate::options::defaults::DefaultCollection,
    pub extra_defaults: crate::options::defaults::DefaultCollection,
}

impl Default for State {
    fn default() -> Self {
        Self {
            args: vec![],

            iwad_found: false,
            iwad_file: String::new(),

            no_monsters: false,
            respawn_parm: false,
            fast_parm: false,
            dev_parm: false,

            display_fps_dots: false,

            multiplayer_mode: crate::types::MultiplayerMode::CoOp,

            config_dir: String::new(),
            default_main_config: String::new(),
            default_extra_config: String::new(),

            forward_move: [0x19, 0x32],
            side_move: [0x18, 0x28],

            input_options: crate::options::InputOptions::default(),
            video_options: crate::options::VideoOptions::default(),
            joystick_options: crate::options::JoystickOptions::default(),
            sound_options: crate::options::SoundOptions::default(),
            controls: crate::options::Controls::default(),

            doom_defaults: crate::options::defaults::doom_defaults_init(),
            extra_defaults: crate::options::defaults::extra_defaults_init(),
        }
    }
}
