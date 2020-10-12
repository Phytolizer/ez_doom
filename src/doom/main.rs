impl crate::state::State {
    pub fn doom_main(&mut self) {
        self.at_exit(Self::endoom, false);

        crate::strings::print_banner(crate::meta::PACKAGE_STRING);

        if self.parm_exists("--dedicated") {
            println!("Dedicated server mode.");
            self.dedicated_server();
            // Never returns
        }

        if self.parm_exists("--search") {
            self.master_query();
            // Never returns
        }

        if let Some(p) = self.check_parm_with_args("--query", 1) {
            self.query_address(&self.args[p + 1].clone());
            // Never returns
        }

        if self.parm_exists("--local-search") {
            self.lan_query();
            // Never returns
        }

        self.no_monsters = self.parm_exists("--no-monsters");
        self.respawn_parm = self.parm_exists("--respawn");
        self.fast_parm = self.parm_exists("--fast");
        self.dev_parm = self.parm_exists("--dev-parm");
        self.display_fps_dots = self.dev_parm;

        if self.parm_exists("--deathmatch") {
            self.multiplayer_mode = crate::types::MultiplayerMode::Deathmatch;
        }
        if self.parm_exists("--alt-death") {
            self.multiplayer_mode = crate::types::MultiplayerMode::AltDeathmatch;
        }

        if self.dev_parm {
            println!("{}", crate::english::DEVSTR);
        }

        if cfg!(windows) && self.parm_exists("--cdrom") {
            println!("{}", crate::english::CDROM);
            self.set_config_dir("c:\\doomdata\\");
        } else {
            self.set_config_dir("");
        }

        if let Some(p) = self.check_parm("--turbo") {
            let scale = num::clamp(
                self.args
                    .iter()
                    .nth(p + 1)
                    .map(|s| s.parse().unwrap_or(0))
                    .unwrap_or(200),
                10,
                400,
            );
            println!("turbo scale: {}", scale);
            self.forward_move[0] = self.forward_move[0] * scale / 100;
            self.forward_move[1] = self.forward_move[1] * scale / 100;
            self.side_move[0] = self.side_move[0] * scale / 100;
            self.side_move[1] = self.side_move[1] * scale / 100;
        }

        self.default_main_config = String::from("default.cfg");
        self.default_extra_config = String::from("ez_doom.cfg");

        self.bind_variables();
        self.load_defaults();

        self.at_exit(Self::save_defaults, false);

        self.iwad_file = self.find_iwad();
    }

    fn endoom(&mut self) {
        // TODO
    }

    fn bind_variables(&mut self) {
        self.bind_input_variables();
        self.bind_video_variables();
        self.bind_joystick_variables();
        self.bind_sound_variables();

        self.bind_base_controls();
        self.bind_weapon_controls();
        self.bind_map_controls();
        self.bind_menu_controls();
        self.bind_chat_controls(crate::defs::MAX_PLAYERS);
    }
}
