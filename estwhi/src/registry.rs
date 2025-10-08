use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::*;

const ROOT: HKEY = HKEY_CURRENT_USER;
const SUBKEY: &str = "Software\\Estwhi";

pub fn set_u32(name: &str, value: u32) -> windows::core::Result<()> {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(SUBKEY);
        let status = RegCreateKeyW(ROOT, PCWSTR(sub.as_ptr()), &mut hkey);
        if status != ERROR_SUCCESS {
            return Err(windows::core::Error::from_win32());
        }
        let n = wides(name);
        let bytes = &value.to_le_bytes();
        let status = RegSetValueExW(hkey, PCWSTR(n.as_ptr()), 0, REG_DWORD, Some(bytes));
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

pub fn get_u32(name: &str, default: u32) -> u32 {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(SUBKEY);
        if RegOpenKeyExW(ROOT, PCWSTR(sub.as_ptr()), 0, KEY_QUERY_VALUE, &mut hkey) != ERROR_SUCCESS
        {
            return default;
        }
        let n = wides(name);
        let mut typ: REG_VALUE_TYPE = REG_DWORD;
        let mut buf: u32 = 0;
        let mut len = std::mem::size_of::<u32>() as u32;
        let status = RegGetValueW(
            hkey,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_DWORD,
            Some(&mut typ),
            Some(&mut buf as *mut _ as *mut _),
            Some(&mut len),
        );
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            buf
        } else {
            default
        }
    }
}

// set_string intentionally omitted until first use to keep warnings clean
pub fn set_string(name: &str, value: &str) -> windows::core::Result<()> {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(SUBKEY);
        let status = RegCreateKeyW(ROOT, PCWSTR(sub.as_ptr()), &mut hkey);
        if status != ERROR_SUCCESS {
            return Err(windows::core::Error::from_win32());
        }
        let n = wides(name);
        let v = wides(value);
        let bytes: &[u8] = core::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 2);
        let status = RegSetValueExW(hkey, PCWSTR(n.as_ptr()), 0, REG_SZ, Some(bytes));
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

pub fn get_string(name: &str) -> Option<String> {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(SUBKEY);
        if RegOpenKeyExW(ROOT, PCWSTR(sub.as_ptr()), 0, KEY_QUERY_VALUE, &mut hkey) != ERROR_SUCCESS
        {
            return None;
        }
        let n = wides(name);
        let mut len: u32 = 0;
        // Query length
        let _ = RegGetValueW(
            hkey,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_SZ,
            None,
            None,
            Some(&mut len),
        );
        if len < 2 {
            let _ = RegCloseKey(hkey);
            return None;
        }
        let mut buf = vec![0u16; (len as usize + 1) / 2];
        let ok = RegGetValueW(
            hkey,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_SZ,
            None,
            Some(buf.as_mut_ptr() as *mut _),
            Some(&mut len),
        ) == ERROR_SUCCESS;
        let _ = RegCloseKey(hkey);
        if ok {
            if let Some(pos) = buf.iter().position(|&c| c == 0) {
                buf.truncate(pos);
            }
            String::from_utf16(&buf).ok()
        } else {
            None
        }
    }
}

fn wides(s: &str) -> Vec<u16> {
    use std::os::windows::prelude::*;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

// Random Things registry functions (separate subkey)
const RT_SUBKEY: &str = "Software\\Estwhi\\Random Things";

pub fn rt_set_u32(name: &str, value: u32) -> windows::core::Result<()> {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(RT_SUBKEY);
        let status = RegCreateKeyW(ROOT, PCWSTR(sub.as_ptr()), &mut hkey);
        if status != ERROR_SUCCESS {
            return Err(windows::core::Error::from_win32());
        }
        let n = wides(name);
        let bytes = &value.to_le_bytes();
        let status = RegSetValueExW(hkey, PCWSTR(n.as_ptr()), 0, REG_DWORD, Some(bytes));
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

pub fn rt_get_u32(name: &str, default: u32) -> u32 {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(RT_SUBKEY);
        if RegOpenKeyExW(ROOT, PCWSTR(sub.as_ptr()), 0, KEY_QUERY_VALUE, &mut hkey) != ERROR_SUCCESS
        {
            return default;
        }
        let n = wides(name);
        let mut typ: REG_VALUE_TYPE = REG_DWORD;
        let mut buf: u32 = 0;
        let mut len = std::mem::size_of::<u32>() as u32;
        let status = RegGetValueW(
            hkey,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_DWORD,
            Some(&mut typ),
            Some(&mut buf as *mut _ as *mut _),
            Some(&mut len),
        );
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            buf
        } else {
            default
        }
    }
}
