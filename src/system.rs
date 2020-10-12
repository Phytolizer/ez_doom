impl crate::state::State {
    pub fn error<S: AsRef<str>>(&self, why: S) -> ! {
        eprintln!("{}", why.as_ref());
        std::process::exit(-1);
    }

    pub fn at_exit(&mut self, _callback: impl Fn(&mut crate::state::State), _run_on_error: bool) {
        // TODO
    }
}

pub fn console_stdout() -> bool {
    atty::is(atty::Stream::Stdout)
}
