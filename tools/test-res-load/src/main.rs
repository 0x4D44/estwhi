use std::env;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::System::LibraryLoader::{
    FindResourceW, LOAD_LIBRARY_FLAGS, LoadLibraryExW, LoadResource, SizeofResource,
};
use windows::Win32::UI::WindowsAndMessaging::RT_BITMAP;
use windows::core::PCWSTR;

const fn make_int_resource(id: u16) -> PCWSTR {
    PCWSTR(id as usize as *const u16)
}

fn main() {
    let exe_path = env::args()
        .nth(1)
        .unwrap_or_else(|| r"c:\apps\estwhi.exe".to_string());
    let path = widen(&exe_path);
    let module =
        unsafe { LoadLibraryExW(PCWSTR(path.as_ptr()), None, LOAD_LIBRARY_FLAGS(0x00000001)) }
            .expect("load estwhi");

    for id in 1..=52 {
        unsafe {
            let handle = FindResourceW(module, make_int_resource(id), RT_BITMAP);
            if handle.0.is_null() {
                println!("ID {} -> not found", id);
                continue;
            }
            let size = SizeofResource(module, handle);
            println!("ID {} -> size {}", id, size);
            let loaded = LoadResource(module, handle);
            println!("    load ok? {}", loaded.is_ok());
        }
    }
}

fn widen(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}
