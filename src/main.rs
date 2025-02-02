mod app;
mod commands;
mod debug_print;

fn main() {
    let mut shell = app::Shell::new();
    shell.run_repl();
}
