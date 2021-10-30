use libc;
use termios::*;

pub struct Terminal {
    original_termios: Option<Termios>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            original_termios: None,
        }
    }

    pub fn setup(&mut self) {
        self.original_termios = Termios::from_fd(libc::STDIN_FILENO).ok();

        let mut termios = self.original_termios.unwrap().clone();
        termios.c_lflag &= !(ECHO | ICANON);
        tcsetattr(libc::STDIN_FILENO, TCSANOW, &termios).unwrap();
    }

    pub fn cleanup(&self) {
        println!("clean");
        tcsetattr(libc::STDIN_FILENO, TCSANOW, &self.original_termios.unwrap()).unwrap();
    }
}
