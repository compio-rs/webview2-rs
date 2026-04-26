use std::{
    env::current_exe,
    ffi::{OsString, c_void},
    os::windows::ffi::OsStringExt,
    path::{Component, Path, PathBuf, Prefix},
};

use super::*;
use windows::Win32::{
    Foundation::{E_FAIL, ERROR_FILE_NOT_FOUND, ERROR_INSUFFICIENT_BUFFER, FARPROC, FreeLibrary},
    Security::PSID,
    Storage::Packaging::Appx::{
        AddPackageDependency, AddPackageDependencyOptions_None,
        CreatePackageDependencyOptions_None, GetCurrentPackageInfo, PACKAGE_INFO, PACKAGE_VERSION,
        PACKAGEDEPENDENCY_CONTEXT, PackageDependencyLifetimeKind_Process,
        PackageDependencyProcessorArchitectures_None, TryCreatePackageDependency,
    },
    System::{
        LibraryLoader::{GetProcAddress, LoadLibraryW},
        Memory::{GetProcessHeap, HEAP_FLAGS, HeapFree},
        Registry::KEY_WOW64_32KEY,
    },
};
use windows_core::{Error, HRESULT, HSTRING, PCWSTR, PWSTR, Param, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
enum WebView2RunTimeType {
    Installed = 0,
    Redistributable = 1,
}

const NUM_CHANNELS: usize = 5;

const CHANNEL_UUID: [&str; NUM_CHANNELS] = [
    "{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
    "{2CD8A007-E189-409D-A2C8-9AF4EF3C72AA}",
    "{0D50BFEC-CD6A-4F9A-964C-C7416E3ACB10}",
    "{65C35B14-6C1D-4122-AC46-7148CC9D6497}",
    "{BE59E8FD-089A-411B-A3B0-051D9E417818}",
];

const CHANNEL_PACKAGE_NAME: [&HSTRING; NUM_CHANNELS] = [
    windows_core::h!("Microsoft.WebView2Runtime.Stable_8wekyb3d8bbwe"),
    windows_core::h!("Microsoft.WebView2Runtime.Beta_8wekyb3d8bbwe"),
    windows_core::h!("Microsoft.WebView2Runtime.Dev_8wekyb3d8bbwe"),
    windows_core::h!("Microsoft.WebView2Runtime.Canary_8wekyb3d8bbwe"),
    windows_core::h!("Microsoft.WebView2Runtime.Internal_8wekyb3d8bbwe"),
];

const INSTALL_KEY_PATH: &str = "Software\\Microsoft\\EdgeUpdate\\ClientState\\";

const MIN_COMPATIBLE_VER: [u16; 4] = [86, 0, 616, 0];

#[cfg(target_arch = "x86_64")]
const EMBEDDED_WEBVIEW_PATH: &str = "EBWebView\\x64\\EmbeddedBrowserWebView.dll";
#[cfg(target_arch = "x86")]
const EMBEDDED_WEBVIEW_PATH: &str = "EBWebView\\x86\\EmbeddedBrowserWebView.dll";
#[cfg(target_arch = "aarch64")]
const EMBEDDED_WEBVIEW_PATH: &str = "EBWebView\\arm64\\EmbeddedBrowserWebView.dll";

pub fn create_env_impl(
    params: WebView2EnvironmentParams,
    handler: &ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
) -> Result<()> {
    let (runtime_type, path) = if let Some(sub_folder) =
        ptr_to_pathbuf(params.embedded_edge_sub_folder)
        && !sub_folder.as_os_str().is_empty()
    {
        let path = find_embedded_client_dll(sub_folder)?;
        (WebView2RunTimeType::Redistributable, path)
    } else {
        let path = find_installed_client_dll(params.release_channel_preference)?;
        (WebView2RunTimeType::Installed, path)
    };
    let path = HSTRING::from(path.into_os_string());
    create_env_with_client_dll(
        path,
        true,
        runtime_type,
        params.user_data_dir,
        params.environment_options.as_ref(),
        handler,
    )
}

fn ptr_to_pathbuf(ptr: PCWSTR) -> Option<PathBuf> {
    if ptr.is_null() {
        None
    } else {
        Some(PathBuf::from(OsString::from_wide(unsafe { ptr.as_wide() })))
    }
}

fn find_installed_client_dll(preference: WebView2ReleaseChannelPreference) -> Result<PathBuf> {
    for i in 0..NUM_CHANNELS {
        let channel = if preference == WebView2ReleaseChannelPreference::Canary {
            4 - i
        } else {
            i
        };
        let sub_key = format!("{}{}", INSTALL_KEY_PATH, CHANNEL_UUID[channel]);
        if let Some(path) = find_installed_client_dll_for_channel(&sub_key, false) {
            return Ok(path);
        }
        if let Some(path) = find_installed_client_dll_for_channel(&sub_key, true) {
            return Ok(path);
        }

        struct DepId(PWSTR);

        impl Drop for DepId {
            fn drop(&mut self) {
                unsafe {
                    if let Ok(heap) = GetProcessHeap() {
                        HeapFree(heap, HEAP_FLAGS(0), Some(self.0.0.cast())).ok();
                    }
                }
            }
        }

        unsafe {
            if let Ok(dep) = TryCreatePackageDependency(
                PSID::default(),
                CHANNEL_PACKAGE_NAME[channel],
                PACKAGE_VERSION::default(),
                PackageDependencyProcessorArchitectures_None,
                PackageDependencyLifetimeKind_Process,
                None,
                CreatePackageDependencyOptions_None,
            )
            .map(DepId)
            {
                let mut ctx = PACKAGEDEPENDENCY_CONTEXT::default();
                AddPackageDependency(dep.0, 0, AddPackageDependencyOptions_None, &mut ctx, None)
                    .ok();
            }
        }

        let mut len = 0;
        let mut packages = 0;
        let flags = 0x180001;
        if unsafe { GetCurrentPackageInfo(flags, &mut len, None, Some(&mut packages)) }
            != ERROR_INSUFFICIENT_BUFFER
        {
            continue;
        }
        let mut buffer = Vec::<PACKAGE_INFO>::with_capacity(packages as usize);
        if unsafe {
            GetCurrentPackageInfo(
                flags,
                &mut len,
                Some(buffer.as_mut_ptr().cast()),
                Some(&mut packages),
            )
        }
        .is_err()
        {
            continue;
        }
        unsafe { buffer.set_len(packages as usize) };
        let Some(package) = buffer.iter().find(|package| unsafe {
            let package_family_name =
                std::ptr::addr_of!(package.packageFamilyName).read_unaligned();
            &package_family_name.to_hstring() == CHANNEL_PACKAGE_NAME[channel]
        }) else {
            continue;
        };
        let package_id = unsafe { std::ptr::addr_of!(package.packageId).read_unaligned() };
        let version = unsafe {
            [
                package_id.version.Anonymous.Anonymous.Major,
                package_id.version.Anonymous.Anonymous.Minor,
                package_id.version.Anonymous.Anonymous.Build,
                package_id.version.Anonymous.Anonymous.Revision,
            ]
        };
        let path = unsafe {
            std::ptr::addr_of!(package.path)
                .read_unaligned()
                .to_hstring()
        };
        let path = PathBuf::from(path.to_os_string());
        if let Some(path) = check_version_and_find_dll(version, path) {
            return Ok(path);
        }
    }
    Err(ERROR_FILE_NOT_FOUND.into())
}

fn find_installed_client_dll_for_channel(sub_key: &str, system: bool) -> Option<PathBuf> {
    let key = if system {
        windows_registry::LOCAL_MACHINE
    } else {
        windows_registry::CURRENT_USER
    }
    .options()
    .read()
    .access(KEY_WOW64_32KEY.0)
    .open(sub_key)
    .ok()?;
    let path = key.get_hstring("EBWebView").ok()?;
    let path = PathBuf::from(path.to_os_string());
    let version = parse_version(path.file_name()?.to_str()?)?;
    check_version_and_find_dll(version, path)
}

fn parse_version(s: &str) -> Option<[u16; 4]> {
    let mut parts = s.split('.').map(str::parse::<u16>);
    Some([
        parts.next()?.ok()?,
        parts.next()?.ok()?,
        parts.next()?.ok()?,
        parts.next()?.ok()?,
    ])
}

fn check_version_and_find_dll(version: [u16; 4], path: PathBuf) -> Option<PathBuf> {
    if version >= MIN_COMPATIBLE_VER {
        find_client_dll_in_folder(path).ok()
    } else {
        None
    }
}

type CreateWebViewEnvironmentWithOptionsInternalFn = Option<
    unsafe extern "system" fn(
        bool,
        WebView2RunTimeType,
        PCWSTR,
        *mut c_void,
        *mut c_void,
    ) -> HRESULT,
>;

type DllCanUnloadNowFn = Option<unsafe extern "system" fn() -> HRESULT>;

fn create_env_with_client_dll(
    path: HSTRING,
    unknown: bool,
    runtime_type: WebView2RunTimeType,
    user_data_folder: PCWSTR,
    options: Option<&ICoreWebView2EnvironmentOptions>,
    handler: &ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
) -> Result<()> {
    unsafe {
        let client_dll = LoadLibraryW(PCWSTR(path.as_ptr()))?;
        let Some(create_proc) = std::mem::transmute::<
            FARPROC,
            CreateWebViewEnvironmentWithOptionsInternalFn,
        >(GetProcAddress(
            client_dll,
            windows_core::s!("CreateWebViewEnvironmentWithOptionsInternal"),
        )) else {
            return Err(Error::from_thread());
        };
        let can_unload_proc = std::mem::transmute::<FARPROC, DllCanUnloadNowFn>(GetProcAddress(
            client_dll,
            windows_core::s!("DllCanUnloadNow"),
        ));

        let hr = create_proc(
            unknown,
            runtime_type,
            user_data_folder,
            options.param().abi(),
            Some(handler).param().abi(),
        );

        if let Some(can_unload) = can_unload_proc
            && can_unload().is_ok()
        {
            FreeLibrary(client_dll).ok();
        }

        hr.ok()
    }
}

fn find_embedded_client_dll(sub_folder: PathBuf) -> Result<PathBuf> {
    let prefix = path_prefix(&sub_folder);
    if sub_folder.is_absolute()
        && prefix
            .map(|p| matches!(p, Prefix::Disk(_) | Prefix::VerbatimDisk(_)))
            .unwrap_or_default()
    {
        return find_client_dll_in_folder(sub_folder);
    }

    if sub_folder.is_relative() {
        let path = current_exe()?
            .parent()
            .ok_or_else(|| Error::from_hresult(E_FAIL))?
            .join(sub_folder);
        return find_client_dll_in_folder(path);
    }

    find_client_dll_in_folder(sub_folder)
}

fn path_prefix(path: &Path) -> Option<Prefix<'_>> {
    match path.components().next()? {
        Component::Prefix(prefix) => Some(prefix.kind()),
        _ => None,
    }
}

fn find_client_dll_in_folder(folder: PathBuf) -> Result<PathBuf> {
    let path = folder.join(EMBEDDED_WEBVIEW_PATH);
    if path.exists() {
        Ok(path)
    } else {
        Err(ERROR_FILE_NOT_FOUND.into())
    }
}
