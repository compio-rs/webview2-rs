#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemAlloc(cb) }
}
#[inline]
pub unsafe fn FreeLibrary(hlibmodule: HMODULE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> windows_core::BOOL);
    unsafe { FreeLibrary(hlibmodule) }
}
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(
    applicationusermodelidlength: *mut u32,
    applicationusermodelid: Option<windows_core::PWSTR>,
) -> WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> WIN32_ERROR);
    unsafe {
        GetCurrentApplicationUserModelId(
            applicationusermodelidlength as _,
            applicationusermodelid.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn GetCurrentPackageInfo(
    flags: u32,
    bufferlength: *mut u32,
    buffer: Option<*mut u8>,
    count: Option<*mut u32>,
) -> WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> WIN32_ERROR);
    unsafe {
        GetCurrentPackageInfo(
            flags,
            bufferlength as _,
            buffer.unwrap_or(core::mem::zeroed()) as _,
            count.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeW<P0>(lptstrfilename: P0, lpdwhandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoSizeW(lptstrfilename : windows_core::PCWSTR, lpdwhandle : *mut u32) -> u32);
    unsafe {
        GetFileVersionInfoSizeW(
            lptstrfilename.param().abi(),
            lpdwhandle.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn GetFileVersionInfoW<P0>(
    lptstrfilename: P0,
    dwhandle: Option<u32>,
    dwlen: u32,
    lpdata: *mut core::ffi::c_void,
) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoW(lptstrfilename : windows_core::PCWSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe {
        GetFileVersionInfoW(
            lptstrfilename.param().abi(),
            dwhandle.unwrap_or(core::mem::zeroed()) as _,
            dwlen,
            lpdata as _,
        )
    }
}
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_core::PCWSTR) -> HMODULE);
    unsafe { GetModuleHandleW(lpmodulename.param().abi()) }
}
#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: HMODULE, lpprocname: P1) -> FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_core::PCSTR) -> FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn GetProcessHeap() -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
    unsafe { GetProcessHeap() }
}
#[inline]
pub unsafe fn HeapFree(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    lpmem: Option<*const core::ffi::c_void>,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe {
        HeapFree(
            hheap as _,
            dwflags,
            lpmem.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
}
#[inline]
pub unsafe fn VerQueryValueW<P1>(
    pblock: *const core::ffi::c_void,
    lpsubblock: P1,
    lplpbuffer: *mut *mut core::ffi::c_void,
    pulen: *mut u32,
) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn VerQueryValueW(pblock : *const core::ffi::c_void, lpsubblock : windows_core::PCWSTR, lplpbuffer : *mut *mut core::ffi::c_void, pulen : *mut u32) -> windows_core::BOOL);
    unsafe {
        VerQueryValueW(
            pblock,
            lpsubblock.param().abi(),
            lplpbuffer as _,
            pulen as _,
        )
    }
}
pub type AddPackageDependencyOptions = i32;
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0;
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1;
pub type CreatePackageDependencyOptions = i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution:
    CreatePackageDependencyOptions = 1;
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0;
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2;
pub const ERROR_FILE_NOT_FOUND: WIN32_ERROR = 2;
pub const ERROR_INSUFFICIENT_BUFFER: WIN32_ERROR = 122;
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
pub type HEAP_FLAGS = u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMODULE(pub *mut core::ffi::c_void);
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PACKAGEDEPENDENCY_CONTEXT(pub *mut core::ffi::c_void);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSID(pub *mut core::ffi::c_void);
pub type PackageDependencyLifetimeKind = i32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1;
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0;
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2;
pub type PackageDependencyProcessorArchitectures = i32;
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8;
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures =
    16;
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures =
    1;
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0;
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4;
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2;
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures =
    32;
pub type REG_SAM_FLAGS = u32;
pub type WIN32_ERROR = u32;
