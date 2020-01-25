#![warn(clippy::all)]
pub mod environment;
pub mod error;
pub mod function;
pub mod module;
pub mod runtime;
mod ty;
pub use self::ty::{WasmArg, WasmArgs, WasmType};

#[inline]
pub fn print_m3_info() {
    unsafe { ffi::m3_PrintM3Info() };
}

#[inline]
pub fn print_profiler_info() {
    unsafe { ffi::m3_PrintProfilerInfo() };
}
