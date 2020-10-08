pub mod doom;
pub mod meta;
pub mod misc;
pub mod state;
pub mod system;

fn main() {
    let mut state: Box<state::State> = Box::default();
    state.args = std::env::args().collect();

    if state.parm_exists("--version") {
        println!("{}", meta::PACKAGE_STRING);
        return;
    }

    state.add_loose_files();
    state.find_response_file();
    dbg!(&state.args);
}
