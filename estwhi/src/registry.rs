//! Windows Registry persistence layer for Estimation Whist.
//!
//! This module provides type-safe access to the Windows Registry for persisting
//! game configuration and state. All values are stored under HKEY_CURRENT_USER
//! to avoid requiring administrator privileges.

use std::cell::RefCell;
use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::*;

const ROOT: HKEY = HKEY_CURRENT_USER;
const SUBKEY: &str = "Software\\Estwhi";

// Random Things registry functions (separate subkey)
const RT_SUBKEY: &str = "Software\\Estwhi\\Random Things";

/// Maximum allowed registry string length (64KB - reasonable limit to prevent DoS)
const MAX_REGISTRY_STRING_LEN: u32 = 65536;

thread_local! {
    static TEST_KEY_OVERRIDE: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn resolve_key(default: &str) -> String {
    TEST_KEY_OVERRIDE.with(|cell| {
        if let Some(s) = cell.borrow().as_ref() {
            if default == SUBKEY {
                s.clone()
            } else if default == RT_SUBKEY {
                format!("{}\\Random Things", s)
            } else {
                default.to_string()
            }
        } else {
            default.to_string()
        }
    })
}

#[cfg(test)]
pub fn with_test_key<F: FnOnce()>(key: &str, f: F) {
    TEST_KEY_OVERRIDE.with(|cell| *cell.borrow_mut() = Some(key.to_string()));
    f();
    TEST_KEY_OVERRIDE.with(|cell| *cell.borrow_mut() = None);
}

/// RAII wrapper for HKEY to ensure handle closure.
struct RegKey(HKEY);

impl Drop for RegKey {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            // SAFETY: Closing a valid registry key handle.
            unsafe {
                let _ = RegCloseKey(self.0);
            }
        }
    }
}

impl RegKey {
    /// Opens an existing registry key or creates it.
    fn open(subkey: &str, create: bool) -> windows::core::Result<Self> {
        let mut hkey = HKEY::default();
        let sub = wides(subkey);
        let status = unsafe {
            if create {
                RegCreateKeyW(ROOT, PCWSTR(sub.as_ptr()), &mut hkey)
            } else {
                // Try opening with query access first, fallback to set if needed by caller logic
                // But for our helpers, we usually know what we want.
                // We'll use broad enough access for both get/set in internal helpers.
                RegOpenKeyExW(
                    ROOT,
                    PCWSTR(sub.as_ptr()),
                    0,
                    KEY_QUERY_VALUE | KEY_SET_VALUE,
                    &mut hkey,
                )
            }
        };

        if status == ERROR_SUCCESS {
            Ok(RegKey(hkey))
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

/// Sets a u32 value in the registry under the main Estwhi key.
pub fn set_u32(name: &str, value: u32) -> windows::core::Result<()> {
    set_u32_internal(&resolve_key(SUBKEY), name, value)
}

/// Gets a u32 value from the registry, returning a default if not found.
pub fn get_u32(name: &str, default: u32) -> u32 {
    get_u32_internal(&resolve_key(SUBKEY), name, default)
}

/// Sets a string value in the registry under the main Estwhi key.
pub fn set_string(name: &str, value: &str) -> windows::core::Result<()> {
    set_string_internal(&resolve_key(SUBKEY), name, value)
}

/// Gets a string value from the registry, returning None if not found.
pub fn get_string(name: &str) -> Option<String> {
    get_string_internal(&resolve_key(SUBKEY), name)
}

/// Sets a u32 value in the registry under the Random Things subkey.
pub fn rt_set_u32(name: &str, value: u32) -> windows::core::Result<()> {
    set_u32_internal(&resolve_key(RT_SUBKEY), name, value)
}

/// Gets a u32 value from the Random Things subkey, returning a default if not found.
pub fn rt_get_u32(name: &str, default: u32) -> u32 {
    get_u32_internal(&resolve_key(RT_SUBKEY), name, default)
}

// Internal implementation functions

fn set_u32_internal(subkey: &str, name: &str, value: u32) -> windows::core::Result<()> {
    let key = RegKey::open(subkey, true)?;
    let n = wides(name);
    let bytes = value.to_le_bytes();
    // SAFETY: Setting a DWORD value with valid buffer and size.
    let status = unsafe { RegSetValueExW(key.0, PCWSTR(n.as_ptr()), 0, REG_DWORD, Some(&bytes)) };
    if status == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(windows::core::Error::from_win32())
    }
}

fn get_u32_internal(subkey: &str, name: &str, default: u32) -> u32 {
    let key = match RegKey::open(subkey, false) {
        Ok(k) => k,
        Err(_) => return default,
    };
    let n = wides(name);
    let mut buf: u32 = 0;
    let mut len = std::mem::size_of::<u32>() as u32;
    // SAFETY: Querying a DWORD value. Handle is managed by RegKey.
    let status = unsafe {
        RegQueryValueExW(
            key.0,
            PCWSTR(n.as_ptr()),
            None,
            None,
            Some(&mut buf as *mut u32 as *mut u8),
            Some(&mut len),
        )
    };
    if status == ERROR_SUCCESS && len == 4 {
        buf
    } else {
        default
    }
}

fn set_string_internal(subkey: &str, name: &str, value: &str) -> windows::core::Result<()> {
    let key = RegKey::open(subkey, true)?;
    let n = wides(name);
    let v = wides(value);
    // SAFETY: Setting a string value. Buffer contains null-terminated UTF-16.
    let bytes: &[u8] = unsafe { core::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 2) };
    let status = unsafe { RegSetValueExW(key.0, PCWSTR(n.as_ptr()), 0, REG_SZ, Some(bytes)) };
    if status == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(windows::core::Error::from_win32())
    }
}

fn get_string_internal(subkey: &str, name: &str) -> Option<String> {
    let key = RegKey::open(subkey, false).ok()?;
    let n = wides(name);
    let mut len: u32 = 0;
    // SAFETY: Sizing query for string value.
    unsafe {
        let _ = RegGetValueW(
            key.0,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_SZ,
            None,
            None,
            Some(&mut len),
        );
    }
    if !(2..=MAX_REGISTRY_STRING_LEN).contains(&len) {
        return None;
    }

    let mut buf = vec![0u16; (len as usize).div_ceil(2)];
    // SAFETY: Retrieving string value into pre-allocated buffer.
    let ok = unsafe {
        RegGetValueW(
            key.0,
            None,
            PCWSTR(n.as_ptr()),
            RRF_RT_REG_SZ,
            None,
            Some(buf.as_mut_ptr() as *mut _),
            Some(&mut len),
        ) == ERROR_SUCCESS
    };

    if ok {
        if let Some(pos) = buf.iter().position(|&c| c == 0) {
            buf.truncate(pos);
        }
        String::from_utf16(&buf).ok()
    } else {
        None
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
            // Try to delete RT subkey too if it exists
            let rt = format!("{}\\Random Things", key);
            let sub_rt = wides(&rt);
            let _ = RegDeleteKeyW(ROOT, PCWSTR(sub_rt.as_ptr()));
        }
    }

    #[test]
    fn test_u32_roundtrip() {
        let key = get_random_key();

        with_test_key(&key, || {
            // Default when missing
            assert_eq!(get_u32("Missing", 42), 42);

            // Write and Read
            assert!(set_u32("MyVal", 100).is_ok());
            assert_eq!(get_u32("MyVal", 0), 100);

            // Overwrite
            assert!(set_u32("MyVal", 200).is_ok());
            assert_eq!(get_u32("MyVal", 0), 200);

            // Random Things
            assert!(rt_set_u32("RTVal", 999).is_ok());
            assert_eq!(rt_get_u32("RTVal", 0), 999);
        });

        cleanup(&key);
    }

    #[test]
    fn test_string_roundtrip() {
        let key = get_random_key();

        with_test_key(&key, || {
            // Missing
            assert!(get_string("Missing").is_none());

            // Write and Read
            assert!(set_string("MyStr", "Hello World").is_ok());
            assert_eq!(get_string("MyStr").unwrap(), "Hello World");
        });

        cleanup(&key);
    }

    #[test]
    fn test_resolve_key_no_override() {
        // Ensure no override is set for this test
        assert_eq!(resolve_key("MyDefault"), "MyDefault");
    }

    #[test]
    fn test_resolve_key_overrides() {
        // Test override logic
        let override_key = "Software\\Estwhi\\TestOverride";

        with_test_key(override_key, || {
            // SUBKEY should resolve to override
            assert_eq!(resolve_key(SUBKEY), override_key);

            // RT_SUBKEY should resolve to override + "\\Random Things"
            let expected_rt = format!("{}\\Random Things", override_key);
            assert_eq!(resolve_key(RT_SUBKEY), expected_rt);

            // Other keys should remain unchanged
            assert_eq!(resolve_key("Other"), "Other");
        });
    }
}
