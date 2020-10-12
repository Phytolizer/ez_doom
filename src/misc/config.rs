use std::io::BufRead;
use std::io::Write;

impl crate::state::State {
    pub fn set_config_dir<S: AsRef<str>>(&mut self, config_dir: S) {
        let config_dir = config_dir.as_ref();
        if config_dir.is_empty() {
            self.config_dir = get_default_config_dir();
        } else {
            self.config_dir = config_dir.to_owned();
        }

        if !self.config_dir.is_empty() {
            println!("Using {} for configuration and saves", self.config_dir);
        }

        std::fs::create_dir_all(&self.config_dir).unwrap_or_else(|e| {
            self.error(format!(
                "Error creating config directory '{}': {}",
                self.config_dir, e,
            ));
        })
    }

    pub fn load_defaults(&mut self) {
        // TODO bind autoload path here

        if let Some(i) = self.check_parm_with_args("--config", 1) {
            self.doom_defaults.file_name = self.args[i + 1].clone();
            println!("   default file: {}", self.doom_defaults.file_name);
        } else {
            self.doom_defaults.file_name =
                self.config_dir.clone() + self.default_main_config.as_str();
        }

        if let Some(i) = self.check_parm_with_args("--extra-config", 1) {
            self.extra_defaults.file_name = self.args[i + 1].clone();
            println!(
                "        extra configuration file: {}",
                self.extra_defaults.file_name
            );
        } else {
            self.extra_defaults.file_name =
                self.config_dir.clone() + self.default_extra_config.as_str();
        }

        self.load_default_collection(&self.doom_defaults);
        self.load_default_collection(&self.extra_defaults);
    }

    pub fn load_default_collection(
        &self,
        collection: &crate::options::defaults::DefaultCollection,
    ) {
        let f = match std::fs::File::open(&collection.file_name) {
            Ok(f) => f,
            Err(_) => return,
        };
        let r = std::io::BufReader::new(f);

        for line in r.lines() {
            if line.is_err() {
                break;
            }
            let line = line.unwrap();
            let (rest, _) = match take_space(&line) {
                Ok(x) => x,
                Err(_) => continue,
            };
            let (rest, def) = match take_word(rest) {
                Ok(x) => x,
                Err(_) => continue,
            };
            let (strparm, _) = take_space(rest).unwrap();

            let def = self.search_collection(collection, def);
            if def.is_none() || !def.as_ref().unwrap().read().bound {
                continue;
            }

            let strparm = strparm.trim();
            let strparm = if strparm.len() >= 2
                && strparm.chars().nth(0).unwrap() == '"'
                && strparm.chars().last().unwrap() == '"'
            {
                &strparm[1..strparm.len() - 1]
            } else {
                &strparm[..]
            };

            set_variable(def.clone().unwrap(), strparm);
        }
    }

    pub fn save_defaults(&mut self) {
        self.save_default_collection(&self.doom_defaults);
        self.save_default_collection(&self.extra_defaults);
    }

    fn save_default_collection(&self, collection: &crate::options::defaults::DefaultCollection) {
        let mut f = match std::fs::File::create(&collection.file_name) {
            Ok(f) => f,
            Err(_) => return,
        };

        let defaults = &collection.defaults;
        let write_fail_err = |e| {
            self.error(format!(
                "Failed to write to config file {}: {}",
                &collection.file_name, e
            ))
        };
        for default in defaults {
            let default = default.read();
            if !default.bound {
                continue;
            }

            f.write_all(default.name.as_bytes())
                .unwrap_or_else(write_fail_err);
            f.write_all(b" ").unwrap_or_else(write_fail_err);

            if default.is_key() {
                let mut v = default.location.as_key().unwrap();
                if v == crate::keys::RSHIFT {
                    v = 54;
                } else if default.untranslated != 0 && v == default.original_translated {
                    v = default.untranslated;
                } else {
                    for s in 0..128i32 {
                        if SCAN_TO_KEY[s as usize] == v {
                            v = s;
                            break;
                        }
                    }
                }
                write!(f, "{}", v).unwrap_or_else(write_fail_err);
            } else if default.is_int() {
                write!(f, "{}", default.location.as_int().unwrap()).unwrap_or_else(write_fail_err);
            } else if default.is_int_hex() {
                write!(f, "{:x}", default.location.as_int_hex().unwrap())
                    .unwrap_or_else(write_fail_err);
            } else if default.is_float() {
                write!(f, "{}", default.location.as_float().unwrap())
                    .unwrap_or_else(write_fail_err);
            } else if default.is_string() {
                write!(f, "\"{}\"", default.location.as_string().unwrap())
                    .unwrap_or_else(write_fail_err);
            }
            writeln!(f).unwrap_or_else(write_fail_err);
        }
    }
}

fn set_variable(
    default: std::sync::Arc<parking_lot::RwLock<crate::options::defaults::Default>>,
    strparm: &str,
) {
    let mut default = default.write();
    if default.is_string() {
        default.location.set_string(strparm.to_owned());
    } else if default.is_int() {
        default.location.set_int(strparm.parse().unwrap_or(0));
    } else if default.is_int_hex() {
        default
            .location
            .set_int_hex(i32::from_str_radix(strparm.trim_start_matches("0x"), 16).unwrap_or(0))
    } else if default.is_key() {
        let mut intparm = strparm.parse().unwrap_or(0);
        default.untranslated = intparm;
        if intparm >= 0 && intparm <= 128 {
            intparm = SCAN_TO_KEY[intparm as usize];
        } else {
            intparm = 0;
        }

        default.original_translated = intparm;
        default.location.set_key(intparm);
    } else if default.is_float() {
        let fparm = strparm.parse().unwrap_or(0.0);
        default.location.set_float(fparm);
    }
}

fn take_space(i: &str) -> nom::IResult<&str, &str> {
    nom::take_while!(i, |c: char| c.is_whitespace())
}

fn take_word(i: &str) -> nom::IResult<&str, &str> {
    nom::take_till1!(i, |c: char| c.is_whitespace())
}

fn get_default_config_dir() -> String {
    sdl2::filesystem::pref_path("", crate::meta::PACKAGE_TARNAME).unwrap_or(String::new())
}

const SCAN_TO_KEY: [i32; 128] = [
    0,
    27,
    b'1' as i32,
    b'2' as i32,
    b'3' as i32,
    b'4' as i32,
    b'5' as i32,
    b'6' as i32,
    b'7' as i32,
    b'8' as i32,
    b'9' as i32,
    b'0' as i32,
    b'-' as i32,
    b'=' as i32,
    crate::keys::BACKSPACE,
    9,
    b'q' as i32,
    b'w' as i32,
    b'e' as i32,
    b'r' as i32,
    b't' as i32,
    b'y' as i32,
    b'u' as i32,
    b'i' as i32,
    b'o' as i32,
    b'p' as i32,
    b'[' as i32,
    b']' as i32,
    13,
    crate::keys::RCTRL,
    b'a' as i32,
    b's' as i32,
    b'd' as i32,
    b'f' as i32,
    b'g' as i32,
    b'h' as i32,
    b'j' as i32,
    b'k' as i32,
    b'l' as i32,
    b';' as i32,
    b'\'' as i32,
    b'`' as i32,
    crate::keys::RSHIFT,
    b'\\' as i32,
    b'z' as i32,
    b'x' as i32,
    b'c' as i32,
    b'v' as i32,
    b'b' as i32,
    b'n' as i32,
    b'm' as i32,
    b',' as i32,
    b'.' as i32,
    b'/' as i32,
    crate::keys::RSHIFT,
    crate::keys::KP_MULTIPLY,
    crate::keys::RALT,
    b' ' as i32,
    crate::keys::CAPSLOCK,
    crate::keys::F1,
    crate::keys::F2,
    crate::keys::F3,
    crate::keys::F4,
    crate::keys::F5,
    crate::keys::F6,
    crate::keys::F7,
    crate::keys::F8,
    crate::keys::F9,
    crate::keys::F10,
    crate::keys::PAUSE,
    crate::keys::SCRLCK,
    crate::keys::HOME,
    crate::keys::UPARROW,
    crate::keys::PGUP,
    crate::keys::MINUS,
    crate::keys::LEFTARROW,
    crate::keys::KP_5,
    crate::keys::RIGHTARROW,
    crate::keys::KP_PLUS,
    crate::keys::END,
    crate::keys::DOWNARROW,
    crate::keys::PGDN,
    crate::keys::INS,
    crate::keys::DEL,
    0,
    0,
    0,
    crate::keys::F11,
    crate::keys::F12,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    crate::keys::PRTSCR,
    0,
];
