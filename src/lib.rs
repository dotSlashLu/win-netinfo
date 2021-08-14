mod bindings {
    windows::include_bindings!();
}

use libc;
use log::debug;
use std::convert::TryInto;
use std::fmt;
use std::mem::size_of;

use bindings::Windows::Win32::NetworkManagement::IpHelper;
use bindings::Windows::Win32::System::Diagnostics::Debug;
use bindings::Windows::Win32::System::SystemServices::CHAR;

#[derive(Default, Debug)]
pub struct NetworkParams {
    pub host_name: String,
    pub dns_servers: Vec<String>,
    pub routing: bool, // specifies whether routing is enabled on the local computer
    pub proxy: bool,   // specifies whether the local computer is acting as an ARP proxy
    pub dns: bool,     // specifies whether DNS is enabled on the local computer
}

#[derive(Debug)]
pub struct Error {
    pub reason: String,
    pub code: u32,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for Error {}

pub fn network_params() -> Result<NetworkParams, Error> {
    let mut params = NetworkParams::default();
    let mut info: &mut IpHelper::FIXED_INFO_W2KSP1 = &mut IpHelper::FIXED_INFO_W2KSP1::default();
    let mut buflen: u32 = size_of::<IpHelper::FIXED_INFO_W2KSP1>().try_into().unwrap();

    let mut ret = Debug::WIN32_ERROR(unsafe { IpHelper::GetNetworkParams(info, &mut buflen) });
    let mut info_ptr: *mut libc::c_void = std::ptr::null::<libc::c_void>() as *mut libc::c_void;

    if ret == Debug::ERROR_BUFFER_OVERFLOW {
        debug!("BufferOverflow");
        info_ptr = unsafe {
            let ptr = libc::malloc(buflen as libc::size_t);
            info = &mut *(ptr as *mut IpHelper::FIXED_INFO_W2KSP1);
            ptr
        };
        ret = Debug::WIN32_ERROR(unsafe { IpHelper::GetNetworkParams(info, &mut buflen) });
        if ret != Debug::NO_ERROR {
            return Err(Error {
                reason: "Windows API error".to_owned(),
                code: ret.0,
            });
        }
    }

    params.host_name = chars_to_string(&info.HostName);
    params.routing = if info.EnableRouting == 1 { true } else { false };
    params.proxy = if info.EnableProxy == 1 { true } else { false };
    params.dns = if info.EnableDns == 1 { true } else { false };

    debug!("{:#?}", &info.DnsServerList.IpAddress.String);
    let ip = chars_to_string(&info.DnsServerList.IpAddress.String);
    params.dns_servers.push(ip);

    let mut addr = info.DnsServerList.Next;
    while !addr.is_null() {
        unsafe {
            params
                .dns_servers
                .push(chars_to_string(&((*addr).IpAddress.String)));
            addr = (*addr).Next;
        }
    }

    unsafe {
        if info_ptr.as_ref().is_some() {
            libc::free(info_ptr);
        }
    }

    Ok(params)
}

pub fn adapters_info() {
    // https://docs.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getadaptersinfo
    unimplemented!()
}

fn chars_to_string(chars: &[CHAR]) -> String {
    let mut arr: Vec<u8> = vec![];
    for c in chars.iter() {
        if c.is_null() {
            break;
        }
        arr.push(c.0);
    }
    let s = String::from_utf8_lossy(&arr[..]);
    let str = s.to_string();
    str
}

#[cfg(test)]
mod tests {
    #[test]
    fn network_params() {
        println!("{:?}", super::network_params());
    }
}
