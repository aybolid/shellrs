mod app;
mod commands;

fn main() {
    let mut repl = app::REPL::new();
    repl.run();
}
