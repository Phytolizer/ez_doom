use std::io::Read;

use bitflags::bitflags;

use crate::doom::iwad::IwadName;

bitflags! {
    struct FileKind : u8 {
        const UNKNOWN = 0x0;
        const ONE = 0x1;
        const IWAD = 0x2;
        const PWAD = 0x4;
        const DEH = 0x8;
        const MAX = 0x16;
    }
}

impl Default for FileKind {
    fn default() -> Self {
        Self::MAX
    }
}

#[derive(Default, Debug)]
struct Argument {
    string: String,
    kind: FileKind,
    stable: usize,
}

impl crate::state::State {
    pub fn parm_exists(&self, parm: &str) -> bool {
        self.args.iter().any(|arg| arg == parm)
    }

    pub fn check_parm(&self, parm: &str) -> Option<usize> {
        self.args.iter().position(|arg| arg == parm)
    }

    pub fn check_parm_with_args(&self, parm: &str, n_args: usize) -> Option<usize> {
        self.args.iter().position(|arg| arg == parm).and_then(|p| {
            if p + n_args >= self.args.len() {
                None
            } else {
                Some(p)
            }
        })
    }

    pub fn add_loose_files(&mut self) {
        if self.args.len() < 2 {
            return;
        }

        let mut arguments = Vec::<Argument>::with_capacity(self.args.len() + 3);
        arguments.resize_with(self.args.len() + 3, Default::default);
        let mut kinds: FileKind = FileKind::UNKNOWN;

        for i in 1..self.args.len() {
            let arg = self.args[i].clone();
            let arg_c = arg.chars().collect::<Vec<_>>();

            if arg.len() < 3
                || arg_c[0] == '-'
                || arg_c[0] == '@'
                || (!arg_c[0].is_alphabetic() || arg_c[1] != ':' || arg_c[2] != '\\')
                    && (arg_c[0] != '\\' || arg_c[1] != '\\')
            {
                return;
            }

            let kind = self.guess_file_kind(&arg);
            arguments[i].string = arg;
            arguments[i].kind = kind;
            arguments[i].stable = i;
            kinds |= kind;
        }

        let mut argc = self.args.len();

        if kinds.contains(FileKind::IWAD) {
            arguments[argc].string = String::from("-iwad");
            arguments[argc].kind = FileKind::IWAD - FileKind::ONE;
            argc += 1;
        }
        if kinds.contains(FileKind::PWAD) {
            arguments[argc].string = String::from("-merge");
            arguments[argc].kind = FileKind::PWAD - FileKind::ONE;
            argc += 1;
        }
        if kinds.contains(FileKind::DEH) {
            arguments[argc].string = String::from("-deh");
            arguments[argc].kind = FileKind::DEH - FileKind::ONE;
            argc += 1;
        }

        let mut new_args = Vec::<String>::new();
        new_args.resize_with(argc, Default::default);
        arguments[1..].sort_by(|a, b| a.kind.cmp(&b.kind).then(a.stable.cmp(&b.stable)));

        new_args[0] = self.args[0].clone();
        for i in 1..argc {
            new_args[i] = arguments[i].string.clone();
        }

        self.args = new_args;
    }

    fn guess_file_kind(&mut self, arg: &str) -> FileKind {
        let arg = std::path::PathBuf::from(arg);

        let base = arg
            .file_name()
            .map(|b| b.to_string_lossy().to_string())
            .unwrap_or_else(|| String::new());
        let lower = base.to_lowercase();

        if !self.iwad_found && lower.is_iwad_name() {
            self.iwad_found = true;
            FileKind::IWAD
        } else if lower.ends_with(".wad") || lower.ends_with(".lmp") {
            FileKind::PWAD
        } else if lower.ends_with(".deh") || lower.ends_with(".hhe") || lower.ends_with(".seh") {
            FileKind::DEH
        } else {
            FileKind::UNKNOWN
        }
    }

    pub fn find_response_file(&mut self) {
        for i in 1..self.args.len() {
            let arg = &self.args[i];
            if arg.chars().nth(0).unwrap() == '@' {
                let response_filename = arg.chars().skip(1).collect::<String>();
                self.load_response_file(i, response_filename);
            }
        }

        while let Some(i) = self.check_parm_with_args("--response", 1) {
            self.args[i] = String::from("-_");
            self.load_response_file(i + 1, self.args[i + 1].clone());
        }
    }

    fn load_response_file(&mut self, arg_index: usize, response_filename: String) {
        let mut handle = std::fs::File::open(&response_filename).unwrap_or_else(|e| {
            eprintln!(
                "Problem opening response file {}: {}",
                &response_filename, e
            );
            std::process::exit(1);
        });

        println!("Found response file {}", &response_filename);

        let size = std::fs::metadata(&response_filename)
            .map(|m| m.len())
            .unwrap_or_else(|e| {
                eprintln!(
                    "Problem getting size of response file {}: {}",
                    &response_filename, e
                );
                std::process::exit(1);
            });

        let mut file = String::new();
        handle.read_to_string(&mut file).unwrap_or_else(|e| {
            self.error(format!(
                "Failed to read fill contents of '{}': {}",
                response_filename, e
            ));
        });
        let file = file.chars().collect::<Vec<_>>();

        drop(handle);

        let mut new_args = Vec::<String>::new();

        for i in 0..arg_index {
            new_args.push(self.args[i].clone());
        }

        let mut k = 0;
        while k < size {
            while k < size && file[k as usize].is_whitespace() {
                k += 1;
            }

            if k >= size {
                break;
            }

            if file[k as usize] == '"' {
                k += 1;
                let start = k as usize;

                while k < size && file[k as usize] != '"' && file[k as usize] != '\n' {
                    k += 1;
                }

                if k >= size || file[k as usize] == '\n' {
                    self.error(format!(
                        "Quotes unclosed in response file '{}'",
                        response_filename
                    ));
                }

                new_args.push(file[start..k as usize].iter().collect());
                k += 1;
            }
        }

        for i in arg_index + 1..self.args.len() {
            new_args.push(self.args[i].clone());
        }

        self.args = new_args;
    }
}
