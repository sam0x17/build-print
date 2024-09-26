#[doc(hidden)]
pub mod _hidden {
    pub use colored::*;
}

#[macro_export]
macro_rules! println {
    () => {
        ::std::println!("cargo:warning=\x1b[2K\r");
    };
    ($($arg:tt)*) => {
        ::std::println!("cargo:warning=\x1b[2K\r{}", ::std::format!($($arg)*));
    };
}

#[test]
fn test_println_compiles() {
    println!();
    println!("hello world!");
    println!("hello {}", 33);
    println!("hello {}, {}, {}", 1, 2, 3);
}
