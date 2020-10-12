pub fn print_banner(message: &str) {
    println!("{: ^70}", message);
}

pub fn print_divider() {
    println!("{}", std::iter::repeat("=").take(75).collect::<String>());
}

pub fn print_startup_banner(game_description: &str) {
    print_divider();
    print_banner(game_description);
    print_divider();

    println!(
        " {} is free software, covered by the GNU General Public",
        crate::meta::PACKAGE_NAME
    );
    println!(" License.  There is NO warranty; not even for MERCHANTIBILITY or FITNESS");
    println!(" FOR A PARTICULAR PURPOSE. You are welcome to change and distribute");
    println!(" copies under certain conditions. See the source for more information.");

    print_divider();
}
