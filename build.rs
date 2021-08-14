#[cfg(target_os = "windows")]
fn main() {
    windows::build! {
        Windows::Win32::NetworkManagement::IpHelper::*,
        Windows::Win32::System::SystemServices::CHAR,
        Windows::Win32::System::Diagnostics::Debug::*,
    };
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("this crate only supports windows platform");
}
