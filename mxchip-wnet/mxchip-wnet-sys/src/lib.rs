//! System bindings to the mxchip WNET library. These are auto generated.
//!
//! Currently this does not compile with `cargo build`. Please use `cargo xbuild` instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub mod ffi {
    pub use core::ffi::c_void;
    pub type c_char = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i64;
    pub type c_ulong = u64;
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
