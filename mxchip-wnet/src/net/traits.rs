use super::Error;
use mxchip_wnet_sys as sys;

pub type Result<T> = core::result::Result<T, super::Error>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

pub trait ToSocketAddr {
    fn resolve(self) -> Result<u32>;
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
