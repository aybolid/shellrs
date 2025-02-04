use std::{
    fs::File,
    io::{self, Read, Write},
    os::fd::AsRawFd,
};

use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};

pub struct InputHandler {
    tty: File,

    cursor_pos: usize,
}

impl InputHandler {
    pub fn new() -> Self {
        let tty = File::open("/dev/tty").unwrap();
        Self { tty, cursor_pos: 0 }
    }

    pub fn input_loop(&mut self, buffer: &mut String, prompt: &str) {
        let fd = self.tty.as_raw_fd();

        // save the original terminal settings.
        let original_termios = Self::get_termios(fd).unwrap();

        // set terminal to raw mode (disable canonical mode and echo).
        let raw_termios = Self::disable_canonical_echo(original_termios);
        Self::set_termios(fd, &raw_termios).unwrap();

        let redraw_line = |buffer: &str, cursor_pos: usize| {
            // \r returns to the beginning of the line; \x1b[K clears the line from the cursor onward.
            print!("\r{}{}\x1b[K", prompt, buffer);

            let cursor_col = prompt.len() + cursor_pos + 1;

            // move the cursor to the correct position.
            print!("\r\x1b[{}G", cursor_col);

            io::stdout().flush().unwrap();
        };

        loop {
            let mut byte = [0u8; 1];
            if self.tty.read(&mut byte).unwrap() == 0 {
                break;
            }
            let b = byte[0];

            match b {
                b'\n' | b'\r' => {
                    println!();
                    break;
                }
                0x1B => {
                    // possibly an escape sequence.
                    let mut seq = [0u8; 2];
                    if self.tty.read(&mut seq).unwrap() < 2 {
                        continue;
                    }
                    if seq[0] == b'[' {
                        match seq[1] {
                            b'D' => {
                                // left arrow: move cursor left.
                                if self.cursor_pos > 0 {
                                    self.cursor_pos -= 1;
                                }
                            }
                            b'C' => {
                                // right arrow: move cursor right.
                                if self.cursor_pos < buffer.len() {
                                    self.cursor_pos += 1;
                                }
                            }
                            b'A' => {
                                // up arrow: move cursor up.
                                buffer.clear();
                                buffer.push_str("command from history todo!!!");
                                self.cursor_pos = buffer.len();
                            }
                            _ => {}
                        }
                    }
                }
                127 | 8 => {
                    // handle backspace.
                    if self.cursor_pos > 0 {
                        buffer.remove(self.cursor_pos - 1);
                        self.cursor_pos -= 1;
                    }
                }
                0x04 => break, // Ctrl-D (EOF).
                _ if !b.is_ascii_control() => {
                    // insert printable character.
                    let ch = b as char;
                    buffer.insert(self.cursor_pos, ch);
                    self.cursor_pos += 1;
                }
                _ => {}
            }

            redraw_line(buffer, self.cursor_pos);
        }

        // restore the original terminal settings.
        Self::set_termios(fd, &original_termios).expect("failed to restore terminal settings");
        self.cursor_pos = 0;
    }

    /// Helper function to get terminal attributes.
    fn get_termios(fd: i32) -> io::Result<termios> {
        unsafe {
            let mut term = std::mem::zeroed::<termios>();
            if tcgetattr(fd, &mut term) != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(term)
            }
        }
    }

    /// Helper function to set terminal attributes.
    fn set_termios(fd: i32, term: &termios) -> io::Result<()> {
        unsafe {
            if tcsetattr(fd, TCSANOW, term) != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }

    /// Returns a modified termios with canonical mode and echo disabled.
    fn disable_canonical_echo(mut term: termios) -> termios {
        term.c_lflag &= !(ICANON | ECHO);
        term
    }
}
