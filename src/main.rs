mod ui;

use std::io;
use std::panic;

use crate::ui::terminal::Terminal;

fn run() {
    let mut buffer = String::new();

    let _ = io::stdin().read_line(&mut buffer);
}

fn main() {
    let mut terminal = Terminal::new();
    terminal.setup();

    let _ = panic::catch_unwind(run);

    terminal.cleanup();
}
