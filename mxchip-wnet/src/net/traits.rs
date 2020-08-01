use super::Error;
use mxchip_wnet_sys as sys;

/// Result type that is returned from the [Write] and [Read] traits
pub type Result<T> = core::result::Result<T, super::Error>;

/// Read data from a readable stream.
pub trait Read {
    /// Read up to `buf.len()` bytes and place it into `buf`. The value returned is how many bytes were written.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

/// Write data to a writable stream
pub trait Write {
    /// Write the given buffer. Returns the amount of bytes written.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    /// Flush the stream.
    fn flush(&mut self) -> Result<()>;
}

/// Implements any type that can be turned into an IP address.
///
/// Currently this is only implemented on [CStr](../struct.CStr.html).
///
/// ```no_run
/// let addr = CStr::from_bytes_with_nul(b"www.google.com\0").unwrap();
/// let num = addr.resolve().unwrap();
/// ```
pub trait ToSocketAddr: sealed::Sealed {
    /// Resolves to a number that can be passed onto the internal system.
    fn resolve(self) -> Result<u32>;
}

mod sealed {
    pub trait Sealed {}
    impl<'a> Sealed for &'a cstr_core::CStr {}
}

impl<'a> ToSocketAddr for &'a cstr_core::CStr {
    fn resolve(self) -> Result<u32> {
        let addr = unsafe { sys::dns_request(self.as_ptr() as *mut _) as i32 };
        if addr == -1 {
            Err(Error::Unknown(addr))
        } else {
            Ok(addr as u32)
        }
    }
}
