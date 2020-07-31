#![no_std]

pub mod net;
pub use cstr_core::{CStr, FromBytesWithNulError};

use mxchip_wnet_sys as sys;
use net::TcpStream;

pub use sys::_ApList_str as ApEntry;
pub use sys::_adv_ap_info as ApInfo;
pub use sys::_net_para as NetParameter;
pub use sys::_network_InitTypeDef_st as NetworkInfo;

pub type EasylinkUserDataCallback = &'static dyn Fn(&str);
pub type RptConfigmodeCallback = &'static dyn Fn(&NetworkInfo);
pub type ConnectedApInfoCallback = &'static dyn Fn(&ApInfo, &str);
pub type SocketHandler = &'static dyn Fn(TcpStream);
pub type StatusCallback = &'static dyn Fn(Result<(), StatusError>);
pub type ApListCallback = &'static dyn Fn(&[ApEntry]);
pub type NetCallback = &'static dyn Fn(&NetParameter);

pub struct MxChip {}

impl MxChip {
    /// Init the MXChip
    pub fn init() -> Result<Self, StatusError> {
        let result = unsafe { sys::mxchipInit() };
        StatusError::check_error(result)?;
        Ok(Self {})
    }

    /// Get the version of the MXChip
    pub fn version() -> &'static str {
        unsafe {
            let ptr = sys::system_lib_version() as *const u8;
            let len = {
                let mut cur = ptr;
                while *cur != b'\0' {
                    cur = cur.offset(1);
                }
                (cur as usize) - (ptr as usize)
            };

            let slice = core::slice::from_raw_parts(ptr, len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// Set the global status callback. Returns a previous handler if this has been set.
    pub fn status_callback(cb: StatusCallback) -> Option<StatusCallback> {
        unsafe { core::mem::replace(&mut WIFI_STATUS_HANDLER, Some(cb)) }
    }

    /// Turn this chip into a wifi client and connect to the given SSID and password.
    ///
    /// Make sure to register [status_callback] and call [update] frequently.
    ///
    /// [status_callback]: #method.status_callback
    /// [update]: #method.update
    pub fn station_mode(&mut self, ssid: &str, password: &str) {
        let mut init_params: sys::_network_InitTypeDef_st = unsafe { core::mem::zeroed() };
        init_params.wifi_mode = sys::WiFi_Interface_Station as i8;
        strcpy(&mut init_params.wifi_ssid, ssid);
        strcpy(&mut init_params.wifi_key, password);
        init_params.dhcpMode = sys::DHCPOperationMode_DHCP_Client as i8;
        init_params.wifi_retry_interval = 1000;

        unsafe {
            sys::StartNetwork(&mut init_params);
        }
    }

    /// Turn this chip into a wifi station with the given SSID and password.
    ///
    /// Clients can connect to this station and will be given an IP in the range of 192.168.0.10
    /// through 192.168.0.117.
    ///
    /// Make sure to register [status_callback] and call [update] frequently.
    ///
    /// [status_callback]: #method.status_callback
    /// [update]: #method.update
    pub fn soft_ap_mode(&mut self, ssid: &str, password: &str) {
        let mut init_params: sys::_network_InitTypeDef_st = unsafe { core::mem::zeroed() };
        init_params.wifi_mode = sys::WiFi_Interface_Station as i8;
        strcpy(&mut init_params.wifi_ssid, ssid);
        strcpy(&mut init_params.wifi_key, password);
        strcpy(&mut init_params.local_ip_addr, "192.168.0.1");
        strcpy(&mut init_params.net_mask, "255.255.255.0");
        strcpy(&mut init_params.address_pool_start, "192.168.0.10");
        strcpy(&mut init_params.address_pool_end, "192.168.0.177");
        init_params.dhcpMode = sys::DHCPOperationMode_DHCP_Server as i8;

        unsafe {
            sys::StartNetwork(&mut init_params);
        }
    }

    /// Update the internal mxchip state. This should be called frequently.
    pub fn update(&mut self) {
        unsafe { sys::mxchipTick() };
    }
}

fn strcpy(slice: &mut [i8], str: &str) {
    // Copy up to `str.bytes().len()` or `slice.len() - 1` bytes
    // This will ensure that the slice still has a NUL byte at the end
    let max_len = str.bytes().len().max(slice.len() - 1);
    for (i, b) in str.bytes().enumerate().take(max_len) {
        slice[i] = b as i8;
    }
}

#[derive(Debug)]
pub enum StatusError {
    Failed,
    UapUp,
    WifiUp,
    UapDown,
    WifiDown,
    SysIllegal,
    InitFailed8782,
    WifiJoinFailed,
    Unknown(i32),
}

impl StatusError {
    fn check_error(code: i32) -> Result<(), StatusError> {
        match code {
            sys::MxchipStatus_MXCHIP_SUCCESS => Ok(()),
            sys::MxchipStatus_MXCHIP_FAILED => Err(StatusError::Failed),
            sys::MxchipStatus_MXCHIP_UAP_UP => Err(StatusError::UapUp),
            sys::MxchipStatus_MXCHIP_WIFI_UP => Err(StatusError::WifiUp),
            sys::MxchipStatus_MXCHIP_UAP_DOWN => Err(StatusError::UapDown),
            sys::MxchipStatus_MXCHIP_WIFI_DOWN => Err(StatusError::WifiDown),
            sys::MxchipStatus_MXCHIP_SYS_ILLEGAL => Err(StatusError::SysIllegal),
            sys::MxchipStatus_MXCHIP_8782_INIT_FAILED => Err(StatusError::InitFailed8782),
            sys::MxchipStatus_MXCHIP_WIFI_JOIN_FAILED => Err(StatusError::WifiJoinFailed),
            _ => Err(StatusError::Unknown(code)),
        }
    }
}

/// These are all callbacks that are needed for EMW to work.
/// Documentation is basically non-existant at this point, so this is a best-effort implementation
mod emw_callbacks {
    #![warn(missing_docs)]
    use super::*;

    /// Write the APP_VERSION to the given `app_version` parameter
    #[no_mangle]
    pub unsafe extern "C" fn system_version(
        app_version: *mut sys::ffi::c_char,
        len: sys::ffi::c_int,
    ) {
        let mut slice = core::slice::from_raw_parts_mut(app_version, len as usize);
        strcpy(&mut slice, APP_VERSION);
    }

    /// Return user info from "OpenEasylink2_withdata"
    #[no_mangle]
    pub unsafe extern "C" fn easylink_user_data_result(
        datalen: sys::ffi::c_int,
        data: *mut sys::ffi::c_char,
    ) {
        if let Some(handler) = &EASYLINK_USER_DATA_CALLBACK {
            let slice = core::slice::from_raw_parts(data as *const u8, datalen as usize);
            let str = core::str::from_utf8_unchecked(slice);
            handler(str);
        }
    }

    ///  Return SSID and Password
    #[no_mangle]
    pub unsafe extern "C" fn RptConfigmodeRslt(nwkpara: *mut sys::network_InitTypeDef_st) {
        if let Some(handler) = RPT_CONFIGMODE_CALLBACK {
            handler(&*nwkpara);
        }
    }

    /// Return current status once Wi-Fi's status is changed
    #[no_mangle]
    pub unsafe extern "C" fn WifiStatusHandler(status: sys::ffi::c_int) {
        if let Some(handler) = &WIFI_STATUS_HANDLER {
            handler(StatusError::check_error(status));
        }
    }

    /// Return connected AP info
    #[no_mangle]
    pub unsafe extern "C" fn connected_ap_info(
        ap_info: *mut sys::apinfo_adv_t,
        key: *mut sys::ffi::c_char,
        key_len: sys::ffi::c_int,
    ) {
        if let Some(handler) = &CONNECTED_AP_INFO_HANDLER {
            let slice = core::slice::from_raw_parts(key as *const u8, key_len as usize);
            let key = core::str::from_utf8_unchecked(slice);

            let ap_info = &*ap_info;
            handler(ap_info, key);
        }
    }

    /// Notify a succesful socket connection in unblock mode
    #[no_mangle]
    pub unsafe extern "C" fn socket_connected(sockfd: sys::ffi::c_int) {
        if let Some(handler) = &SOCKET_CONNECTED_HANDLER {
            let socket = TcpStream::from_fd(sockfd);
            handler(socket);
        }
    }

    /// AP list callback
    #[no_mangle]
    pub unsafe extern "C" fn ApListCallback(list: *mut sys::ScanResult) {
        if let Some(handler) = &AP_LIST_CALLBACK {
            let list = &*list;
            let list: &[ApEntry] = core::slice::from_raw_parts(list.ApList, list.ApNum as usize);

            handler(list);
        }
    }

    /// Net callback, return the DHCP result once DHCP is success
    #[no_mangle]
    pub unsafe extern "C" fn NetCallback(pnet: *mut sys::net_para_st) {
        if let Some(handler) = &NET_CALLBACK {
            let pnet = &*pnet;
            handler(pnet);
        }
    }
}

// Global state
static mut EASYLINK_USER_DATA_CALLBACK: Option<EasylinkUserDataCallback> = None;
static mut RPT_CONFIGMODE_CALLBACK: Option<RptConfigmodeCallback> = None;
static mut WIFI_STATUS_HANDLER: Option<StatusCallback> = None;
static mut CONNECTED_AP_INFO_HANDLER: Option<ConnectedApInfoCallback> = None;
static mut AP_LIST_CALLBACK: Option<ApListCallback> = None;
static mut SOCKET_CONNECTED_HANDLER: Option<SocketHandler> = None;
static mut NET_CALLBACK: Option<NetCallback> = None;
static mut APP_VERSION: &'static str =
    concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
