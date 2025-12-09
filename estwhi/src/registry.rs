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
    // SAFETY: Calls Win32 registry functions with valid parameters.
    // The registry key handle is properly closed before returning.
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(SUBKEY);
        let status = RegCreateKeyW(ROOT, PCWSTR(sub.as_ptr()), &mut hkey);
        if status != ERROR_SUCCESS {
            return Err(windows::core::Error::from_win32());
        }
        let n = wides(name);
        let v = wides(value);
        // SAFETY: Creating a byte slice from the wide string buffer is safe because
        // we know the exact length and the buffer is valid for the duration of this call.
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

/// Gets a string value from the registry, returning None if not found.
///
/// # Arguments
/// * `name` - The value name to retrieve
///
/// # Returns
/// The stored string value, or `None` if the value doesn't exist, cannot be read,
/// or exceeds the maximum allowed length.
pub fn get_string(name: &str) -> Option<String> {
    // SAFETY: Calls Win32 registry functions with valid parameters.
    // All buffer allocations are bounds-checked and the registry key handle
    // is properly closed before returning.
    unsafe {
        let mut hkey = HKEY::default();
        let sub = wides(SUBKEY);
        if RegOpenKeyExW(ROOT, PCWSTR(sub.as_ptr()), 0, KEY_QUERY_VALUE, &mut hkey) != ERROR_SUCCESS
        {
            return None;
        }
        let n = wides(name);
        let mut len: u32 = 0;
        // Query length first
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
        // Protect against malicious/corrupted registry values with excessive length
        if len > MAX_REGISTRY_STRING_LEN {
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
///
/// # Safety
/// This function makes Win32 registry API calls. The caller must ensure that:
/// - `subkey` is a valid registry path string
/// - The function properly handles all error cases
/// - The registry key handle is closed before returning
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
        let bytes = &value.to_le_bytes();
        let status = RegSetValueExW(hkey, PCWSTR(n.as_ptr()), 0, REG_DWORD, Some(bytes));
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
///
/// # Safety
/// This function makes Win32 registry API calls. The caller must ensure that:
/// - `subkey` is a valid registry path string
/// - The function properly handles all error cases
/// - The registry key handle is closed before returning
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
        // Always close the key handle
        let _ = RegCloseKey(hkey);
        if status == ERROR_SUCCESS {
            buf
        } else {
            default
        }
    }
}
