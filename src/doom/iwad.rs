pub trait IwadName {
    fn is_iwad_name(&self) -> bool;
}

enum GameMission {
    Doom,
    Doom2,
    PackTnt,
    PackPlut,
    PackChex,
    PackHacx,
}

enum GameMode {
    Shareware,
    Registered,
    Commercial,
    Retail,
    Indetermined,
}

struct Iwad {
    name: &'static str,
    mission: GameMission,
    mode: GameMode,
    description: &'static str,
}

fn iwads(i: usize) -> Option<Iwad> {
    match i {
        0 => Some(Iwad {
            name: "doom2.wad",
            mission: GameMission::Doom2,
            mode: GameMode::Commercial,
            description: "Doom II",
        }),
        1 => Some(Iwad {
            name: "plutonia.wad",
            mission: GameMission::PackPlut,
            mode: GameMode::Commercial,
            description: "Final Doom: The Plutonia Experiment",
        }),
        2 => Some(Iwad {
            name: "tnt.wad",
            mission: GameMission::PackTnt,
            mode: GameMode::Commercial,
            description: "Final Doom: TNT: Evilution",
        }),
        3 => Some(Iwad {
            name: "doom.wad",
            mission: GameMission::Doom,
            mode: GameMode::Retail,
            description: "Doom",
        }),
        4 => Some(Iwad {
            name: "doom1.wad",
            mission: GameMission::Doom,
            mode: GameMode::Shareware,
            description: "Doom Shareware",
        }),
        5 => Some(Iwad {
            name: "chex.wad",
            mission: GameMission::PackChex,
            mode: GameMode::Retail,
            description: "Chex Quest",
        }),
        6 => Some(Iwad {
            name: "hacx.wad",
            mission: GameMission::PackHacx,
            mode: GameMode::Commercial,
            description: "Hacx",
        }),
        7 => Some(Iwad {
            name: "freedoom2.wad",
            mission: GameMission::Doom2,
            mode: GameMode::Commercial,
            description: "Freedoom: Phase 2",
        }),
        8 => Some(Iwad {
            name: "freedoom1.wad",
            mission: GameMission::Doom,
            mode: GameMode::Commercial,
            description: "Freedoom: Phase 1",
        }),
        9 => Some(Iwad {
            name: "freedm.wad",
            mission: GameMission::Doom2,
            mode: GameMode::Commercial,
            description: "FreeDM",
        }),
        _ => None,
    }
}

impl IwadName for String {
    fn is_iwad_name(&self) -> bool {
        for i in 0.. {
            match iwads(i).map(|iwad| iwad.name == self) {
                Some(true) => return true,
                None => return false,
                _ => {}
            }
        }
        unreachable!()
    }
}

impl crate::state::State {
    pub fn find_iwad(&mut self) -> String {
        if let Some(iwad_parm) = self.check_parm_with_args("--iwad", 1) {
            self.iwad_file = self.args[iwad_parm + 1].clone();
        }
    }
}
