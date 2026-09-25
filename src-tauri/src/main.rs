//! Biquad Studio — Windows executable entry point.
//!
//! This file configures the Windows subsystem to hide the console window
//! and delegates to `lib::run()`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    biquad_studio::run();
}
