mod app;
mod commands;

fn main() {
    let mut shell = app::Shell::new();
    shell.run_repl();
}
