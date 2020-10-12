#[derive(Clone)]
pub struct DefaultLocation(std::sync::Arc<parking_lot::RwLock<DefaultLocationEnum>>);

impl DefaultLocation {
    pub fn new_key(key: i32) -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(
            DefaultLocationEnum::Key(key),
        )))
    }
    pub fn new_int(int: i32) -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(
            DefaultLocationEnum::Int(int),
        )))
    }
    pub fn new_int_hex(int_hex: i32) -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(
            DefaultLocationEnum::IntHex(int_hex),
        )))
    }
    pub fn new_float(float: f64) -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(
            DefaultLocationEnum::Float(float),
        )))
    }
    pub fn new_string<S: AsRef<str>>(string: S) -> Self {
        Self(std::sync::Arc::new(parking_lot::RwLock::new(
            DefaultLocationEnum::String(string.as_ref().to_owned()),
        )))
    }

    pub fn set_key(&mut self, key: i32) {
        self.0.write().set_key(key)
    }
    pub fn set_int(&mut self, int: i32) {
        self.0.write().set_int(int)
    }
    pub fn set_int_hex(&mut self, int_hex: i32) {
        self.0.write().set_int_hex(int_hex)
    }
    pub fn set_float(&mut self, float: f64) {
        self.0.write().set_float(float)
    }
    pub fn set_string(&mut self, string: String) {
        self.0.write().set_string(string)
    }

    pub fn as_key(&self) -> Result<i32, DefaultLocationEnumVariantError> {
        self.0.read().as_key()
    }
    pub fn as_int(&self) -> Result<i32, DefaultLocationEnumVariantError> {
        self.0.read().as_int()
    }
    pub fn as_int_hex(&self) -> Result<i32, DefaultLocationEnumVariantError> {
        self.0.read().as_int_hex()
    }
    pub fn as_float(&self) -> Result<f64, DefaultLocationEnumVariantError> {
        self.0.read().as_float()
    }
    pub fn as_string(&self) -> Result<String, DefaultLocationEnumVariantError> {
        self.0.read().as_string()
    }
}

#[derive(Clone)]
pub enum DefaultLocationEnum {
    Key(i32),
    Int(i32),
    IntHex(i32),
    Float(f64),
    String(String),
}

impl DefaultLocationEnum {
    pub fn is_key(&self) -> bool {
        match self {
            Self::Key(_) => true,
            _ => false,
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }
    pub fn is_int_hex(&self) -> bool {
        match self {
            Self::IntHex(_) => true,
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    fn set_key(&mut self, key: i32) {
        match self {
            Self::Key(k) => *k = key,
            _ => {}
        }
    }
    fn set_int(&mut self, int: i32) {
        match self {
            Self::Int(i) => *i = int,
            _ => {}
        }
    }
    fn set_int_hex(&mut self, int_hex: i32) {
        match self {
            Self::IntHex(i) => *i = int_hex,
            _ => {}
        }
    }
    fn set_float(&mut self, float: f64) {
        match self {
            Self::Float(f) => *f = float,
            _ => {}
        }
    }
    fn set_string(&mut self, string: String) {
        match self {
            Self::String(s) => *s = string,
            _ => {}
        }
    }

    fn as_key(&self) -> Result<i32, DefaultLocationEnumVariantError> {
        match self {
            Self::Key(k) => Ok(*k),
            _ => Err("not a key".into()),
        }
    }
    fn as_int(&self) -> Result<i32, DefaultLocationEnumVariantError> {
        match self {
            Self::Int(i) => Ok(*i),
            _ => Err("not an int".into()),
        }
    }
    fn as_int_hex(&self) -> Result<i32, DefaultLocationEnumVariantError> {
        match self {
            Self::IntHex(i) => Ok(*i),
            _ => Err("not a hexadecimal int".into()),
        }
    }
    fn as_float(&self) -> Result<f64, DefaultLocationEnumVariantError> {
        match self {
            Self::Float(f) => Ok(*f),
            _ => Err("not a float".into()),
        }
    }
    fn as_string(&self) -> Result<String, DefaultLocationEnumVariantError> {
        match self {
            Self::String(s) => Ok(s.to_owned()),
            _ => Err("not a string".into()),
        }
    }
}

pub struct DefaultLocationEnumVariantError {
    why: String,
}

impl std::error::Error for DefaultLocationEnumVariantError {}

impl std::fmt::Display for DefaultLocationEnumVariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl std::fmt::Debug for DefaultLocationEnumVariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DefaultLocationEnumVariantError {{ {} }}", self.why)
    }
}

impl<'a> From<&str> for DefaultLocationEnumVariantError {
    fn from(s: &str) -> Self {
        Self { why: s.to_owned() }
    }
}

#[derive(Clone)]
pub struct Default {
    pub name: String,
    pub location: DefaultLocation,
    pub untranslated: i32,
    pub original_translated: i32,
    pub bound: bool,
}

impl Default {
    pub fn is_key(&self) -> bool {
        self.location.0.read().is_key()
    }
    pub fn is_int(&self) -> bool {
        self.location.0.read().is_int()
    }
    pub fn is_int_hex(&self) -> bool {
        self.location.0.read().is_int_hex()
    }
    pub fn is_float(&self) -> bool {
        self.location.0.read().is_float()
    }
    pub fn is_string(&self) -> bool {
        self.location.0.read().is_string()
    }
}

pub struct DefaultCollection {
    pub defaults: Vec<std::sync::Arc<parking_lot::RwLock<Default>>>,
    pub file_name: String,
}

impl crate::state::State {
    pub fn get_default_for_name(&self, name: &str) -> std::sync::Arc<parking_lot::RwLock<Default>> {
        self.search_collection(&self.doom_defaults, name)
            .or_else(|| self.search_collection(&self.extra_defaults, name))
            .unwrap_or_else(|| self.error(format!("Unknown configuration variable '{}'", name)))
    }

    pub fn search_collection(
        &self,
        collection: &DefaultCollection,
        name: &str,
    ) -> Option<std::sync::Arc<parking_lot::RwLock<Default>>> {
        collection
            .defaults
            .iter()
            .find(|d| d.read().name == name)
            .cloned()
    }
}

fn config_variable(
    name: &str,
    value: DefaultLocation,
) -> std::sync::Arc<parking_lot::RwLock<Default>> {
    std::sync::Arc::new(parking_lot::RwLock::new(Default {
        name: name.to_owned(),
        location: value,
        untranslated: 0,
        original_translated: 0,
        bound: false,
    }))
}

fn config_variable_key(name: &str) -> std::sync::Arc<parking_lot::RwLock<Default>> {
    config_variable(name, DefaultLocation::new_key(0))
}

fn config_variable_int(name: &str) -> std::sync::Arc<parking_lot::RwLock<Default>> {
    config_variable(name, DefaultLocation::new_int(0))
}

fn config_variable_int_hex(name: &str) -> std::sync::Arc<parking_lot::RwLock<Default>> {
    config_variable(name, DefaultLocation::new_int_hex(0))
}

fn config_variable_float(name: &str) -> std::sync::Arc<parking_lot::RwLock<Default>> {
    config_variable(name, DefaultLocation::new_float(0.0))
}

fn config_variable_string(name: &str) -> std::sync::Arc<parking_lot::RwLock<Default>> {
    config_variable(name, DefaultLocation::new_string(String::new()))
}

pub fn doom_defaults_init() -> DefaultCollection {
    DefaultCollection {
        defaults: vec![
            config_variable_int("mouse_sensitivity"),
            config_variable_int("sfx_volume"),
            config_variable_int("music_volume"),
            config_variable_int("show_messages"),
            config_variable_key("key_right"),
            config_variable_key("key_left"),
            config_variable_key("key_up"),
            config_variable_key("key_down"),
            config_variable_key("key_strafeleft"),
            config_variable_key("key_straferight"),
            config_variable_key("key_fire"),
            config_variable_key("key_use"),
            config_variable_key("key_strafe"),
            config_variable_key("key_speed"),
            config_variable_int("use_mouse"),
            config_variable_int("mouseb_fire"),
            config_variable_int("mouseb_strafe"),
            config_variable_int("mouseb_forward"),
            config_variable_int("mouseb_jump"),
            config_variable_int("use_joystick"),
            config_variable_int("joyb_fire"),
            config_variable_int("joyb_strafe"),
            config_variable_int("joyb_use"),
            config_variable_int("joyb_speed"),
            config_variable_int("joyb_jump"),
            config_variable_int("screenblocks"),
            config_variable_int("detaillevel"),
            config_variable_int("snd_channels"),
            config_variable_int("snd_musicdevice"),
            config_variable_int("snd_sfxdevice"),
            config_variable_int("usegamma"),
            config_variable_string("chatmacro0"),
            config_variable_string("chatmacro1"),
            config_variable_string("chatmacro2"),
            config_variable_string("chatmacro3"),
            config_variable_string("chatmacro4"),
            config_variable_string("chatmacro5"),
            config_variable_string("chatmacro6"),
            config_variable_string("chatmacro7"),
            config_variable_string("chatmacro8"),
            config_variable_string("chatmacro9"),
        ],
        file_name: String::new(),
    }
}
pub fn extra_defaults_init() -> DefaultCollection {
    DefaultCollection {
        defaults: vec![
            config_variable_string("video_driver"),
            config_variable_string("window_position"),
            config_variable_int("fullscreen"),
            config_variable_int("video_display"),
            config_variable_int("aspect_ratio_correct"),
            config_variable_int("integer_scaling"),
            config_variable_int("vga_porch_flash"),
            config_variable_int("window_width"),
            config_variable_int("window_height"),
            config_variable_int("fullscreen_width"),
            config_variable_int("fullscreen_height"),
            config_variable_int("force_software_renderer"),
            config_variable_int("max_scaling_buffer_pixels"),
            config_variable_int("startup_delay"),
            config_variable_int("show_endoom"),
            config_variable_int("show_diskicon"),
            config_variable_int("png_screenshots"),
            config_variable_int("snd_samplerate"),
            config_variable_int("snd_cachesize"),
            config_variable_int("snd_maxslicetime_ms"),
            config_variable_int("snd_pitchshift"),
            config_variable_string("snd_musiccmd"),
            config_variable_string("snd_dmxoption"),
            config_variable_int_hex("opl_io_port"),
            config_variable_int("use_libsamplerate"),
            config_variable_float("libsamplerate_scale"),
            config_variable_string("autoload_path"),
            config_variable_string("music_pack_path"),
            config_variable_string("timidity_cfg_path"),
            config_variable_string("gus_patch_path"),
            config_variable_int("gus_ram_kb"),
            config_variable_int("vanilla_savegame_limit"),
            config_variable_int("vanilla_demo_limit"),
            config_variable_int("vanilla_keyboard_mapping"),
            config_variable_string("player_name"),
            config_variable_int("grabmouse"),
            config_variable_int("novert"),
            config_variable_float("mouse_acceleration"),
            config_variable_int("mouse_threshold"),
            config_variable_int("mouseb_strafeleft"),
            config_variable_int("mouseb_straferight"),
            config_variable_int("mouseb_use"),
            config_variable_int("mouseb_backward"),
            config_variable_int("mouseb_prevweapon"),
            config_variable_int("mouseb_nextweapon"),
            config_variable_int("dclick_use"),
            config_variable_string("joystick_guid"),
            config_variable_int("joystick_index"),
            config_variable_int("joystick_x_axis"),
            config_variable_int("joystick_x_invert"),
            config_variable_int("joystick_y_axis"),
            config_variable_int("joystick_y_invert"),
            config_variable_int("joystick_strafe_axis"),
            config_variable_int("joystick_strafe_invert"),
            config_variable_int("joystick_look_axis"),
            config_variable_int("joystick_look_invert"),
            config_variable_int("joystick_physical_button0"),
            config_variable_int("joystick_physical_button1"),
            config_variable_int("joystick_physical_button2"),
            config_variable_int("joystick_physical_button3"),
            config_variable_int("joystick_physical_button4"),
            config_variable_int("joystick_physical_button5"),
            config_variable_int("joystick_physical_button6"),
            config_variable_int("joystick_physical_button7"),
            config_variable_int("joystick_physical_button8"),
            config_variable_int("joystick_physical_button9"),
            config_variable_int("joystick_physical_button10"),
            config_variable_int("joyb_strafeleft"),
            config_variable_int("joyb_straferight"),
            config_variable_int("joyb_menu_activate"),
            config_variable_int("joyb_toggle_automap"),
            config_variable_int("joyb_prevweapon"),
            config_variable_int("joyb_nextweapon"),
            config_variable_key("key_pause"),
            config_variable_key("key_menu_activate"),
            config_variable_key("key_menu_up"),
            config_variable_key("key_menu_down"),
            config_variable_key("key_menu_left"),
            config_variable_key("key_menu_right"),
            config_variable_key("key_menu_back"),
            config_variable_key("key_menu_forward"),
            config_variable_key("key_menu_confirm"),
            config_variable_key("key_menu_abort"),
            config_variable_key("key_menu_help"),
            config_variable_key("key_menu_save"),
            config_variable_key("key_menu_load"),
            config_variable_key("key_menu_volume"),
            config_variable_key("key_menu_detail"),
            config_variable_key("key_menu_qsave"),
            config_variable_key("key_menu_endgame"),
            config_variable_key("key_menu_messages"),
            config_variable_key("key_menu_qload"),
            config_variable_key("key_menu_quit"),
            config_variable_key("key_menu_gamma"),
            config_variable_key("key_spy"),
            config_variable_key("key_menu_incscreen"),
            config_variable_key("key_menu_decscreen"),
            config_variable_key("key_menu_screenshot"),
            config_variable_key("key_map_toggle"),
            config_variable_key("key_map_north"),
            config_variable_key("key_map_south"),
            config_variable_key("key_map_east"),
            config_variable_key("key_map_west"),
            config_variable_key("key_map_zoomin"),
            config_variable_key("key_map_zoomout"),
            config_variable_key("key_map_maxzoom"),
            config_variable_key("key_map_follow"),
            config_variable_key("key_map_grid"),
            config_variable_key("key_map_mark"),
            config_variable_key("key_map_clearmark"),
            config_variable_key("key_weapon1"),
            config_variable_key("key_weapon2"),
            config_variable_key("key_weapon3"),
            config_variable_key("key_weapon4"),
            config_variable_key("key_weapon5"),
            config_variable_key("key_weapon6"),
            config_variable_key("key_weapon7"),
            config_variable_key("key_weapon8"),
            config_variable_key("key_prevweapon"),
            config_variable_key("key_nextweapon"),
            config_variable_key("key_message_refresh"),
            config_variable_key("key_demo_quit"),
            config_variable_key("key_multi_msg"),
            config_variable_key("key_multi_msgplayer1"),
            config_variable_key("key_multi_msgplayer2"),
            config_variable_key("key_multi_msgplayer3"),
            config_variable_key("key_multi_msgplayer4"),
        ],
        file_name: String::new(),
    }
}
