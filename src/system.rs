impl crate::state::State {
    pub fn error<S: AsRef<str>>(&self, why: S) -> ! {
        eprintln!("{}", why.as_ref());
        std::process::exit(-1);
    }
}
