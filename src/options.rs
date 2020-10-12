pub mod defaults;

pub struct InputOptions {
    pub mouse_acceleration: defaults::DefaultLocation,
    pub mouse_threshold: defaults::DefaultLocation,
    pub vanilla_keyboard_mapping: defaults::DefaultLocation,
    pub novert: defaults::DefaultLocation,
}

impl Default for InputOptions {
    fn default() -> Self {
        Self {
            mouse_acceleration: defaults::DefaultLocation::new_float(2.0),
            mouse_threshold: defaults::DefaultLocation::new_int(10),
            vanilla_keyboard_mapping: defaults::DefaultLocation::new_int(1),
            novert: defaults::DefaultLocation::new_int(0),
        }
    }
}

pub struct VideoOptions {
    pub usemouse: defaults::DefaultLocation,
    pub fullscreen: defaults::DefaultLocation,
    pub video_display: defaults::DefaultLocation,
}

impl Default for VideoOptions {
    fn default() -> Self {
        Self {
            usemouse: defaults::DefaultLocation::new_int(1),
            fullscreen: defaults::DefaultLocation::new_int(1),
            video_display: defaults::DefaultLocation::new_int(0),
        }
    }
}

pub struct JoystickOptions {}

impl Default for JoystickOptions {
    fn default() -> Self {
        Self {}
    }
}

pub struct SoundOptions {}

impl Default for SoundOptions {
    fn default() -> Self {
        Self {}
    }
}

pub struct Controls {
    pub base_controls: BaseControls,
    pub weapon_controls: WeaponControls,
    pub map_controls: MapControls,
    pub menu_controls: MenuControls,
    pub chat_controls: ChatControls,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            base_controls: BaseControls::default(),
            weapon_controls: WeaponControls::default(),
            map_controls: MapControls::default(),
            menu_controls: MenuControls::default(),
            chat_controls: ChatControls::default(),
        }
    }
}

pub struct BaseControls {}

impl Default for BaseControls {
    fn default() -> Self {
        Self {}
    }
}

pub struct WeaponControls {}
impl Default for WeaponControls {
    fn default() -> Self {
        Self {}
    }
}

pub struct MapControls {}

impl Default for MapControls {
    fn default() -> Self {
        Self {}
    }
}

pub struct MenuControls {}

impl Default for MenuControls {
    fn default() -> Self {
        Self {}
    }
}

pub struct ChatControls {}

impl Default for ChatControls {
    fn default() -> Self {
        Self {}
    }
}

impl crate::state::State {
    fn bind_float_variable(&mut self, name: &str, location: defaults::DefaultLocation) {
        let variable = self.get_default_for_name(name);
        assert!(variable.read().is_float());
        variable.write().location = location;
    }
    fn bind_int_variable(&mut self, name: &str, location: defaults::DefaultLocation) {
        let variable = self.get_default_for_name(name);
        assert!(variable.read().is_int());
        variable.write().location = location;
    }

    pub fn bind_input_variables(&mut self) {
        self.bind_float_variable(
            "mouse_acceleration",
            self.input_options.mouse_acceleration.clone(),
        );
        self.bind_int_variable(
            "mouse_threshold",
            self.input_options.mouse_threshold.clone(),
        );
        self.bind_int_variable(
            "vanilla_keyboard_mapping",
            self.input_options.vanilla_keyboard_mapping.clone(),
        );
        self.bind_int_variable("novert", self.input_options.novert.clone());
    }

    pub fn bind_video_variables(&mut self) {}

    pub fn bind_joystick_variables(&mut self) {}

    pub fn bind_sound_variables(&mut self) {}

    pub fn bind_base_controls(&mut self) {}

    pub fn bind_weapon_controls(&mut self) {}

    pub fn bind_map_controls(&mut self) {}

    pub fn bind_menu_controls(&mut self) {}

    pub fn bind_chat_controls(&mut self, max_players: usize) {}
}
