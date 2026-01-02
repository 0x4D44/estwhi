//! Windows Registry persistence layer for Estimation Whist.
//!
//! This module provides type-safe access to the Windows Registry for persisting
//! game configuration and state. All values are stored under HKEY_CURRENT_USER
//! to avoid requiring administrator privileges.

use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::*;

const ROOT: HKEY = HKEY_CURRENT_USER;
const SUBKEY: &str = "Software\\Estwhi";

/// Maximum allowed registry string length (64KB - reasonable limit to prevent DoS)
const MAX_REGISTRY_STRING_LEN: u32 = 65536;

/// Sets a u32 value in the registry under the main Estwhi key.
///
/// # Arguments
/// * `name` - The value name to set
/// * `value` - The u32 value to store
///
/// # Errors
/// Returns an error if the registry key cannot be created or the value cannot be set.
pub fn set_u32(name: &str, value: u32) -> windows::core::Result<()> {
    set_u32_internal(SUBKEY, name, value)
}

/// Gets a u32 value from the registry, returning a default if not found.
///
/// # Arguments
/// * `name` - The value name to retrieve
/// * `default` - The default value to return if the key doesn't exist or cannot be read
///
/// # Returns
/// The stored value, or `default` if the value doesn't exist or an error occurs.
pub fn get_u32(name: &str, default: u32) -> u32 {
    get_u32_internal(SUBKEY, name, default)
}

/// Sets a string value in the registry under the main Estwhi key.
///
/// # Arguments
/// * `name` - The value name to set
/// * `value` - The string value to store
///
/// # Errors
/// Returns an error if the registry key cannot be created or the value cannot be set.
pub fn set_string(name: &str, value: &str) -> windows::core::Result<()> {
    set_string_internal(SUBKEY, name, value)
}

/// Gets a string value from the registry, returning None if not found.
///
/// # Arguments
/// * `name` - The value name to retrieve
///
/// # Returns
/// The stored string value, or `None` if the value doesn't exist, cannot be read,
/// or exceeds the maximum allowed length.
pub fn get_string(name: &str) -> Option<String> {
    get_string_internal(SUBKEY, name)
}

/// Converts a Rust string to a null-terminated wide (UTF-16) string for Win32 APIs.
fn wides(s: &str) -> Vec<u16> {
    use std::os::windows::prelude::*;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

// Random Things registry functions (separate subkey)
const RT_SUBKEY: &str = "Software\\Estwhi\\Random Things";

/// Sets a u32 value in the registry under the Random Things subkey.
///
/// # Arguments
/// * `name` - The value name to set
/// * `value` - The u32 value to store
///
/// # Errors
/// Returns an error if the registry key cannot be created or the value cannot be set.
pub fn rt_set_u32(name: &str, value: u32) -> windows::core::Result<()> {
    set_u32_internal(RT_SUBKEY, name, value)
}

/// Gets a u32 value from the Random Things subkey, returning a default if not found.
///
/// # Arguments
/// * `name` - The value name to retrieve
/// * `default` - The default value to return if the key doesn't exist or cannot be read
///
/// # Returns
/// The stored value, or `default` if the value doesn't exist or an error occurs.
pub fn rt_get_u32(name: &str, default: u32) -> u32 {
    get_u32_internal(RT_SUBKEY, name, default)
}

// Internal implementation functions to avoid code duplication

/// Internal function to set a u32 value in a specific registry subkey.
fn set_u32_internal(subkey: &str, name: &str, value: u32) -> windows::core::Result<()> {
    // SAFETY: Win32 registry calls with valid parameters and proper resource cleanup.
    // The registry key handle is created, used, and closed within this function.
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(subkey);
        let status = RegCreateKeyW(ROOT, PCWSTR(sub.as_ptr()), &mut hkey);
        if status != ERROR_SUCCESS {
            return Err(windows::core::Error::from_win32());
        }
        let n = wides(name);
        let bytes = value.to_le_bytes();
        let status = RegSetValueExW(hkey, PCWSTR(n.as_ptr()), 0, REG_DWORD, Some(&bytes));
        // Always close the key handle, even on error
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

/// Internal function to get a u32 value from a specific registry subkey.
fn get_u32_internal(subkey: &str, name: &str, default: u32) -> u32 {
    // SAFETY: Win32 registry calls with valid parameters and proper resource cleanup.
    // All error cases return the default value. The registry key handle is properly
    // closed in all code paths.
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(subkey);
        if RegOpenKeyExW(ROOT, PCWSTR(sub.as_ptr()), 0, KEY_QUERY_VALUE, &mut hkey) != ERROR_SUCCESS
        {
            return default;
        }
        let n = wides(name);
        let mut buf: u32 = 0;
        let mut len = std::mem::size_of::<u32>() as u32;
        let status = RegQueryValueExW(
            hkey,
            PCWSTR(n.as_ptr()),
            None,
            None,
            Some(&mut buf as *mut u32 as *mut u8),
            Some(&mut len),
        );
        // Always close the key handle
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS && len == 4 {
            buf
        } else {
            default
        }
    }
}

/// Internal function to set a string value in a specific registry subkey.
fn set_string_internal(subkey: &str, name: &str, value: &str) -> windows::core::Result<()> {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(subkey);
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

/// Internal function to get a string value from a specific registry subkey.
fn get_string_internal(subkey: &str, name: &str) -> Option<String> {
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(subkey);
        if RegOpenKeyExW(ROOT, PCWSTR(sub.as_ptr()), 0, KEY_QUERY_VALUE, &mut hkey) != ERROR_SUCCESS
        {
            return None;
        }
        let n = wides(name);
        let mut len: u32 = 0;
        let _ = RegGetValueW(
            hkey,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_SZ,
            None,
            None,
            Some(&mut len),
        );
        if !(2..=MAX_REGISTRY_STRING_LEN).contains(&len) {
            let _ = RegCloseKey(hkey);
            return None;
        }
        #[allow(clippy::manual_div_ceil)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn get_random_key() -> String {
        let mut rng = rand::thread_rng();
        let suffix: u32 = rng.gen();
        format!("Software\\Estwhi\\Test_{}", suffix)
    }

    fn cleanup(key: &str) {
        unsafe {
            let sub = wides(key);
            let _ = RegDeleteKeyW(ROOT, PCWSTR(sub.as_ptr()));
        }
    }

    #[test]
    fn test_u32_roundtrip() {
        let key = get_random_key();
        
        // Default when missing
        assert_eq!(get_u32_internal(&key, "Missing", 42), 42);

        // Write and Read
        assert!(set_u32_internal(&key, "MyVal", 100).is_ok());
        assert_eq!(get_u32_internal(&key, "MyVal", 0), 100);

        // Overwrite
        assert!(set_u32_internal(&key, "MyVal", 200).is_ok());
        assert_eq!(get_u32_internal(&key, "MyVal", 0), 200);

        cleanup(&key);
    }

    #[test]
    fn test_string_roundtrip() {
        let key = get_random_key();

        // Missing
        assert!(get_string_internal(&key, "Missing").is_none());

        // Write and Read
        assert!(set_string_internal(&key, "MyStr", "Hello World").is_ok());
        assert_eq!(
            get_string_internal(&key, "MyStr").unwrap(),
            "Hello World"
        );

        cleanup(&key);
    }
}
