// SPDX-License-Identifier: GPL-2.0-only

use std::fmt::Display;

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {
        $crate::error::fail(format!($($arg)*))
    };
}

pub fn fail(message: impl Display) -> ! {
    eprintln!("\x1B[31m{message}\x1B[0m");
    std::process::exit(1);
}

pub fn tryy<T, E: Display>(wgat: &'static str, result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => fail!("{wgat}: {error}"),
    }
}
