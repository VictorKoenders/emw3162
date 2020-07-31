use super::{Error, ToSocketAddr};
use crate::MxChip;
use mxchip_wnet_sys as sys;

pub struct TcpStream {
    sockfd: i32,
}

impl TcpStream {
    pub(crate) fn from_fd(sockfd: i32) -> Self {
        Self { sockfd }
    }

    pub fn tx_buf_size(&self) -> usize {
        unsafe { sys::tx_buf_size(self.sockfd) as usize }
    }

    pub fn connect(_chip: &MxChip, addr: impl ToSocketAddr, port: u16) -> super::Result<Self> {
        let sockfd = unsafe {
            sys::socket(
                super::Domain::INet as i32,
                super::SocketType::Stream as i32,
                super::Protocol::Tcp as i32,
            )
        };
        if sockfd == -1 {
            return Err(Error::Unknown(sockfd));
        }

        let sockaddr = addr.resolve()?;

        let ret = unsafe {
            sys::connect(
                sockfd,
                &sys::sockaddr_t {
                    s_ip: sockaddr,
                    s_port: port,
                    s_spares: [0, 0, 0, 0, 0, 0],
                    s_type: 0,
                },
                core::mem::size_of::<sys::sockaddr_t>() as _,
            )
        };

        if ret != 0 {
            return Err(Error::Unknown(ret));
        }

        Ok(TcpStream::from_fd(sockfd))
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        unsafe { sys::close(self.sockfd) };
    }
}

impl super::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> super::Result<usize> {
        let n = unsafe { sys::recv(self.sockfd, buf.as_mut_ptr() as *mut _, buf.len(), 0) as i32 };

        if n < 0 {
            Err(Error::Unknown(n))
        } else {
            Ok(n as usize)
        }
    }
}

impl super::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> super::Result<usize> {
        let n = unsafe { sys::send(self.sockfd, buf.as_ptr() as *mut _, buf.len(), 0) as i32 };

        if n < 0 {
            Err(Error::Unknown(n))
        } else {
            Ok(n as usize)
        }
    }

    fn flush(&mut self) -> super::Result<()> {
        Ok(())
    }
}
