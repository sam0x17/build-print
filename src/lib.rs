#[doc(hidden)]
pub mod __hidden {
    pub use colored::*;
}

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
macro_rules! info {
    ($($arg:tt)+) => {
        {
            use $crate::__hidden::*;
            $crate::println!("{} {}", "info:".cyan().bold(), ::std::format!($($arg)+));
        }
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
