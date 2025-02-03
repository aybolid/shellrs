use std::{
    fs::File,
    io::{self, stderr, stdout, StderrLock, StdoutLock, Write},
    process::Stdio,
};

/// The output type of the shell.
/// Can be either a standard output, a standard error or a file.
pub enum ShellOutput {
    Stdout(StdoutLock<'static>),
    Stderr(StderrLock<'static>),
    #[allow(dead_code)]
    File(File),
}

impl ShellOutput {
    pub fn stdout() -> Self {
        ShellOutput::Stdout(stdout().lock())
    }

    pub fn stderr() -> Self {
        ShellOutput::Stderr(stderr().lock())
    }

    #[allow(dead_code)]
    pub fn file(path: String) -> Self {
        ShellOutput::File(File::create(path).unwrap())
    }

    /// Writes a string to the output.
    pub fn writeln(&mut self, s: &str) {
        writeln!(self, "{}", s).expect("should be able to write");
    }

    /// Converts the `ShellOutput` into a `Stdio`.
    pub fn as_stdio(&mut self) -> io::Result<Stdio> {
        match self {
            ShellOutput::File(ref mut file) => Ok(Stdio::from(file.try_clone()?)),
            ShellOutput::Stdout(_) | ShellOutput::Stderr(_) => Ok(Stdio::inherit()),
        }
    }
}

impl Write for ShellOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            ShellOutput::Stdout(ref mut writer) => writer.write(buf),
            ShellOutput::Stderr(ref mut writer) => writer.write(buf),
            ShellOutput::File(ref mut writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            ShellOutput::Stdout(ref mut writer) => writer.flush(),
            ShellOutput::Stderr(ref mut writer) => writer.flush(),
            ShellOutput::File(ref mut writer) => writer.flush(),
        }
    }
}
