#[doc(hidden)]
pub mod _hidden {
    pub use colored::*;
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            ::std::println!("cargo:warning=\x1b[2K\r{}", $($arg)*)
        }
    }
}

#[test]
fn test_println_compiles() {
    println!("hello world!");
}
