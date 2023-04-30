#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

#[cfg(feature = "alloc")]
extern crate alloc;

use std::mem::MaybeUninit;
use windows::Win32::System::Console::{ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, STD_HANDLE, STD_OUTPUT_HANDLE};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    /// Returns foreground color string for ANSI color sequence.
    pub const fn to_fg_str(self) -> &'static str {
        match self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
        }
    }
}

#[cfg(all(windows, feature = "windows_virtual_terminal_sequence"))]
pub fn enable_virtual_terminal() {
    use windows::Win32::System::Console::{GetStdHandle, GetConsoleMode, SetConsoleMode};

    let stdout_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE).expect("stdout is not associated with this process") };
    let mut mode = MaybeUninit::uninit();
    let get_console_mode = unsafe { GetConsoleMode(stdout_handle, mode.as_mut_ptr()) };
    if !get_console_mode.as_bool() {
        return
    }
    // SAFETY: GetConsoleMode succeeded, so `dest` must be overwritten.
    let mode = unsafe { mode.assume_init() };
    let is_vt_enabled = (mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING) == ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    if is_vt_enabled {
        return
    }

    unsafe { SetConsoleMode(stdout_handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) };
}
