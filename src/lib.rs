//! A simple set of macros that allow regular printing as well as properly formatted info,
//! warnings, errors, and notes during build scripts.
//!
//! Traditionally, there are only two ways to print to the console during a build script:
//! * `println!("cargo:warning=...")` to print a somewhat annoyingly formatted warning message
//! * `panic!(..)` to halt the build process and print an error message
//!
//! Regular `println!` statements are not shown during the build process, so this crate hijacks
//! the `cargo:warning=...` variant using ANSI escape sequences to produce a working `println!`
//! macro as well as `info!`, `warn!`, `error!`, and `note!` macros that following the
//! indentation and coloring of standard cargo diagnostic messages.
//!
//! You can also define your own custom print messages using the `custom_println!` macro, which
//! is also the basis for the other macros.

#[macro_export]
macro_rules! println {
    () => {
        ::std::println!("cargo:warning=\x1b[2K\r");
    };
    ($($arg:tt)*) => {
        ::std::println!("cargo:warning=\x1b[2K\r{}", ::std::format!($($arg)*));
    }
}

#[macro_export]
macro_rules! custom_println {
    ($prefix:literal, cyan, $($arg:tt)*) => {
        $crate::println!("   \x1b[1m\x1b[36m{}:\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
    ($prefix:literal, green, $($arg:tt)*) => {
        $crate::println!("   \x1b[1m\x1b[32m{}:\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
    ($prefix:literal, yellow, $($arg:tt)*) => {
        $crate::println!("   \x1b[1m\x1b[33m{}:\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
    ($prefix:literal, red, $($arg:tt)*) => {
        $crate::println!("   \x1b[1m\x1b[31m{}:\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::custom_println!("info", green, $($arg)+);
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::custom_println!("warning", yellow, $($arg)+);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::custom_println!("error", red, $($arg)+);
    }
}

#[macro_export]
macro_rules! note {
    ($($arg:tt)+) => {
        $crate::custom_println!("note", cyan, $($arg)+);
    }
}

#[test]
fn test_println_compiles() {
    println!();
    println!("hello world!");
    println!("hello {}", 33);
    println!("hello {}, {}, {}", 1, 2, 3);
}

#[test]
fn test_info_compiles() {
    info!("hello world!");
    info!("hello {}", 33);
    info!("hello {}, {}, {}", 1, 2, 3);
}

#[test]
fn test_warn_compiles() {
    warn!("hello world!");
    warn!("hello {}", 33);
    warn!("hello {}, {}, {}", 1, 2, 3);
}

#[test]
fn test_error_compiles() {
    error!("hello world!");
    error!("hello {}", 33);
    error!("hello {}, {}, {}", 1, 2, 3);
}

#[test]
fn test_note_compiles() {
    note!("hello world!");
    note!("hello {}", 33);
    note!("hello {}, {}, {}", 1, 2, 3);
}
