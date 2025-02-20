/// print! macro but excluded in release builds.
#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        print!("\x1b[2m{}\x1b[0m", format!($($arg)*));
    };
}

/// println! macro but excluded in release builds.
#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("\x1b[2m{}\x1b[0m", format!($($arg)*));
    };
}

/// eprintln! macro but excluded in release builds.
#[macro_export]
macro_rules! dprintln_err {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!("\x1b[2m\x1b[31m{}\x1b[0m", format!($($arg)*));
    };
}
