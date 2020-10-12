pub mod defs;
pub mod doom;
pub mod english;
pub mod fixed;
pub mod keys;
pub mod meta;
pub mod misc;
pub mod net;
pub mod options;
pub mod state;
pub mod strings;
pub mod system;
pub mod types;

fn main() {
    let mut state: Box<state::State> = Box::default();
    state.args = std::env::args().collect();

    if state.parm_exists("--version") {
        println!("{}", meta::PACKAGE_STRING);
        return;
    }

    state.add_loose_files();
    state.find_response_file();

    sdl2::hint::set("SDL_HINT_NO_SIGNAL_HANDLERS", "1");

    state.doom_main();
}
