#![allow(non_snake_case, clippy::missing_safety_doc)]

use windows::Win32::Foundation::E_POINTER;
use windows_core::{Error, PCWSTR, Param, Ref, Result};

use crate::*;

mod load;

#[inline]
pub unsafe fn CreateCoreWebView2Environment<P0>(handler: P0) -> Result<()>
where
    P0: Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
{
    unsafe { CreateCoreWebView2EnvironmentWithOptions(None, None, None, handler) }
}

#[inline]
pub unsafe fn CreateCoreWebView2EnvironmentWithOptions<P0, P1, P2, P3>(
    browser_executable_folder: P0,
    user_data_folder: P1,
    options: P2,
    handler: P3,
) -> Result<()>
where
    P0: Param<PCWSTR>,
    P1: Param<PCWSTR>,
    P2: Param<ICoreWebView2EnvironmentOptions>,
    P3: Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
{
    unsafe {
        let handler = handler.param();
        let handler = handler.borrow();
        let Some(handler) = handler.as_ref() else {
            return Err(Error::from_hresult(E_POINTER));
        };

        let options = options.param();
        let params = WebView2EnvironmentParams {
            embedded_edge_sub_folder: browser_executable_folder.param().abi(),
            user_data_dir: user_data_folder.param().abi(),
            environment_options: options.borrow(),
            release_channel_preference: WebView2ReleaseChannelPreference::Stable,
        };
        // TODO: UpdateWebViewEnvironmentParamsWithOverrideValues
        load::create_env_impl(params, handler)
    }
}

struct WebView2EnvironmentParams<'a> {
    embedded_edge_sub_folder: PCWSTR,
    user_data_dir: PCWSTR,
    environment_options: Ref<'a, ICoreWebView2EnvironmentOptions>,
    release_channel_preference: WebView2ReleaseChannelPreference,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WebView2ReleaseChannelPreference {
    Stable = 0,
    Canary = 1,
}

const REDIST_OVERRIDE_KEY: &str = "Software\\Policies\\Microsoft\\Edge\\WebView2\\";

const EMBEDDED_OVERRIDE_KEY: &str =
    "Software\\Policies\\Microsoft\\EmbeddedBrowserWebView\\LoaderOverride\\";
