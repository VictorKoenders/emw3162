//! Networking library. Designed to be as close to the `std::net` library as possible.

use mxchip_wnet_sys as sys;

mod error;
mod tcp;
mod traits;

pub use self::error::*;
pub use self::tcp::*;
pub use self::traits::*;

#[repr(u32)]
pub(self) enum Domain {
    INet = sys::AF_INET,
}

#[repr(u32)]
pub(self) enum SocketType {
    Stream = sys::SOCK_STREAM,
    // DGrm = sys::SOCK_DGRM,
}

#[repr(u32)]
pub(self) enum Protocol {
    Tcp = sys::IPPROTO_TCP,
    // Udp = sys::IPPROTO_UDP,
}
