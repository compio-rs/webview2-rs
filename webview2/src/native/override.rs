use std::env::current_exe;

use windows::Win32::{
    Foundation::ERROR_INSUFFICIENT_BUFFER,
    Storage::Packaging::Appx::GetCurrentApplicationUserModelId, System::Registry::KEY_QUERY_VALUE,
};
use windows_core::PWSTR;
use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE, Type};

use super::*;

const REDIST_OVERRIDE_KEY: &str = "Software\\Policies\\Microsoft\\Edge\\WebView2\\";

const EMBEDDED_OVERRIDE_KEY: &str =
    "Software\\Policies\\Microsoft\\EmbeddedBrowserWebView\\LoaderOverride\\";

enum RegistryValue {
    DWord(u32),
    String(HSTRING),
}

pub fn update(params: &mut WebView2EnvironmentParams) {
    if let Some(sub_folder) = get_param(
        "WEBVIEW2_BROWSER_EXECUTABLE_FOLDER",
        "browserExecutableFolder",
    ) && let RegistryValue::String(sub_folder) = sub_folder
    {
        params.embedded_edge_sub_folder = CowPCWSTR::Owned(sub_folder);
    }
    if let Some(user_data_dir) = get_param("WEBVIEW2_USER_DATA_FOLDER", "userDataFolder")
        && let RegistryValue::String(user_data_dir) = user_data_dir
    {
        params.user_data_dir = CowPCWSTR::Owned(user_data_dir);
    }
    if let Some(channel) = get_param(
        "WEBVIEW2_RELEASE_CHANNEL_PREFERENCE",
        "releaseChannelPreference",
    ) {
        let canary = match channel {
            RegistryValue::DWord(v) => v == 1,
            RegistryValue::String(s) => s == "1",
        };
        params.release_channel_preference = if canary {
            WebView2ReleaseChannelPreference::Canary
        } else {
            WebView2ReleaseChannelPreference::Stable
        };
    }
}

fn get_param(env: &str, key: &str) -> Option<RegistryValue> {
    if let Ok(env_var) = std::env::var(env) {
        return Some(RegistryValue::String(env_var.into()));
    }

    let check_key_override = policy_exists(CURRENT_USER) || policy_exists(LOCAL_MACHINE);
    get_param_reg(key, LOCAL_MACHINE, true, check_key_override)
        .or_else(|| get_param_reg(key, CURRENT_USER, true, check_key_override))
        .or_else(|| get_param_reg(key, LOCAL_MACHINE, false, check_key_override))
        .or_else(|| get_param_reg(key, CURRENT_USER, false, check_key_override))
}

fn policy_exists(key: &Key) -> bool {
    key.options().read().open(REDIST_OVERRIDE_KEY).is_ok()
}

fn get_param_reg(
    key: &str,
    root: &Key,
    redist: bool,
    check_key_override: bool,
) -> Option<RegistryValue> {
    let current = current_exe().unwrap_or_default();
    let exe_name = current.file_name().unwrap_or_default().to_string_lossy();
    let id = app_user_mode_id().unwrap_or_default().to_string_lossy();

    if check_key_override
        && redist
        && let Some(v) = read_override(key, root, &id, redist)
            .or_else(|| read_override(key, root, &exe_name, redist))
            .or_else(|| read_override(key, root, "*", redist))
    {
        return Some(v);
    }

    if let Some(v) = read_override(&id, root, key, redist)
        .or_else(|| read_override(&exe_name, root, key, redist))
        .or_else(|| read_override("*", root, key, redist))
    {
        return Some(v);
    }

    None
}

fn read_override(key: &str, root: &Key, value: &str, redist: bool) -> Option<RegistryValue> {
    if key.is_empty() {
        return None;
    }

    let sub_key = if redist {
        REDIST_OVERRIDE_KEY
    } else {
        EMBEDDED_OVERRIDE_KEY
    };

    let phk = root
        .options()
        .access(KEY_QUERY_VALUE.0)
        .open(sub_key)
        .ok()?;

    if matches!(phk.get_type(value).ok()?, Type::U32 | Type::U64) {
        phk.get_u32(value).ok().map(RegistryValue::DWord)
    } else {
        phk.get_hstring(value).ok().map(RegistryValue::String)
    }
}

fn app_user_mode_id() -> Option<HSTRING> {
    let mut buffer = vec![0u16; 0x100];
    let mut len = buffer.len() as u32;
    let mut res =
        unsafe { GetCurrentApplicationUserModelId(&mut len, Some(PWSTR(buffer.as_mut_ptr()))) };
    if res == ERROR_INSUFFICIENT_BUFFER {
        buffer.resize(len as usize, 0);
        res =
            unsafe { GetCurrentApplicationUserModelId(&mut len, Some(PWSTR(buffer.as_mut_ptr()))) };
    }
    if res.is_err() {
        return None;
    }
    buffer.truncate(len as usize);
    Some(HSTRING::from_wide(&buffer))
}
