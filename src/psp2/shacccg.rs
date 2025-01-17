/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub type SceShaccCgParameter = *const crate::ctypes::c_void;
pub type SceShaccCgCallbackOpenFile = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        includedFrom: *const SceShaccCgSourceLocation,
        compileOptions: *const SceShaccCgCompileOptions,
        errorString: *mut *const crate::ctypes::c_char,
    ) -> *mut SceShaccCgSourceFile,
>;
pub type SceShaccCgCallbackReleaseFile = ::core::option::Option<
    unsafe extern "C" fn(
        file: *const SceShaccCgSourceFile,
        compileOptions: *const SceShaccCgCompileOptions,
    ),
>;
pub type SceShaccCgCallbackLocateFile = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        includedFrom: *const SceShaccCgSourceLocation,
        searchPathCount: SceUInt32,
        searchPaths: *const *const crate::ctypes::c_char,
        compileOptions: *const SceShaccCgCompileOptions,
        errorString: *mut *const crate::ctypes::c_char,
    ) -> *const crate::ctypes::c_char,
>;
pub type SceShaccCgCallbackAbsolutePath = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        includedFrom: *const SceShaccCgSourceLocation,
        compileOptions: *const SceShaccCgCompileOptions,
    ) -> *const crate::ctypes::c_char,
>;
pub type SceShaccCgCallbackReleaseFileName = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        compileOptions: *const SceShaccCgCompileOptions,
    ),
>;
pub type SceShaccCgCallbackFileDate = ::core::option::Option<
    unsafe extern "C" fn(
        file: *const SceShaccCgSourceFile,
        includedFrom: *const SceShaccCgSourceLocation,
        compileOptions: *const SceShaccCgCompileOptions,
        timeLastStatusChange: *mut i64,
        timeLastModified: *mut i64,
    ) -> SceInt32,
>;
pub mod SceShaccCgDiagnosticLevel {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHACCCG_DIAGNOSTIC_LEVEL_INFO: Type = 0;
    pub const SCE_SHACCCG_DIAGNOSTIC_LEVEL_WARNING: Type = 1;
    pub const SCE_SHACCCG_DIAGNOSTIC_LEVEL_ERROR: Type = 2;
}
pub mod SceShaccCgTargetProfile {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHACCCG_PROFILE_VP: Type = 0;
    pub const SCE_SHACCCG_PROFILE_FP: Type = 1;
}
pub mod SceShaccCgCallbackDefaults {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHACCCG_SYSTEM_FILES: Type = 0;
    pub const SCE_SHACCCG_TRIVIAL: Type = 1;
}
pub mod SceShaccCgLocale {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHACCCG_ENGLISH: Type = 0;
    pub const SCE_SHACCCG_JAPANESE: Type = 1;
}
#[repr(C)]
pub struct SceShaccCgSourceFile {
    pub fileName: *const crate::ctypes::c_char,
    pub text: *const crate::ctypes::c_char,
    pub size: SceUInt32,
}
#[repr(C)]
pub struct SceShaccCgSourceLocation {
    pub file: *const SceShaccCgSourceFile,
    pub lineNumber: SceUInt32,
    pub columnNumber: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgCallbackList {
    pub openFile: SceShaccCgCallbackOpenFile,
    pub releaseFile: SceShaccCgCallbackReleaseFile,
    pub locateFile: SceShaccCgCallbackLocateFile,
    pub absolutePath: SceShaccCgCallbackAbsolutePath,
    pub releaseFileName: SceShaccCgCallbackReleaseFileName,
    pub fileDate: SceShaccCgCallbackFileDate,
}
#[repr(C)]
pub struct SceShaccCgCompileOptions {
    pub mainSourceFile: *const crate::ctypes::c_char,
    pub targetProfile: SceShaccCgTargetProfile::Type,
    pub entryFunctionName: *const crate::ctypes::c_char,
    pub searchPathCount: SceUInt32,
    pub searchPaths: *const *const crate::ctypes::c_char,
    pub macroDefinitionCount: SceUInt32,
    pub macroDefinitions: *const *const crate::ctypes::c_char,
    pub includeFileCount: SceUInt32,
    pub includeFiles: *const *const crate::ctypes::c_char,
    pub suppressedWarningsCount: SceUInt32,
    pub suppressedWarnings: *const SceUInt32,
    pub locale: SceShaccCgLocale::Type,
    pub useFx: SceInt32,
    pub noStdlib: SceInt32,
    pub optimizationLevel: SceInt32,
    pub useFastmath: SceInt32,
    pub useFastprecision: SceInt32,
    pub useFastint: SceInt32,
    pub field_48: crate::ctypes::c_int,
    pub warningsAsErrors: SceInt32,
    pub performanceWarnings: SceInt32,
    pub warningLevel: SceInt32,
    pub pedantic: SceInt32,
    pub pedanticError: SceInt32,
    pub field_60: crate::ctypes::c_int,
    pub field_64: crate::ctypes::c_int,
}
#[repr(C)]
pub struct SceShaccCgDiagnosticMessage {
    pub level: SceShaccCgDiagnosticLevel::Type,
    pub code: SceUInt32,
    pub location: *const SceShaccCgSourceLocation,
    pub message: *const crate::ctypes::c_char,
}
#[repr(C)]
pub struct SceShaccCgCompileOutput {
    pub programData: *const u8,
    pub programSize: SceUInt32,
    pub diagnosticCount: SceInt32,
    pub diagnostics: *const SceShaccCgDiagnosticMessage,
}
extern "C" {
    pub fn sceShaccCgInitializeCompileOptions(
        options: *mut SceShaccCgCompileOptions,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceShaccCgCompileProgram(
        options: *const SceShaccCgCompileOptions,
        callbacks: *const SceShaccCgCallbackList,
        unk: crate::ctypes::c_int,
    ) -> *const SceShaccCgCompileOutput;
}
extern "C" {
    pub fn sceShaccCgSetDefaultAllocator(
        malloc_cb: ::core::option::Option<
            unsafe extern "C" fn(arg1: crate::ctypes::c_uint) -> *mut crate::ctypes::c_void,
        >,
        free_cb: ::core::option::Option<unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void)>,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceShaccCgInitializeCallbackList(
        callbacks: *mut SceShaccCgCallbackList,
        defaults: SceShaccCgCallbackDefaults::Type,
    );
}
extern "C" {
    pub fn sceShaccCgDestroyCompileOutput(output: *const SceShaccCgCompileOutput);
}
extern "C" {
    pub fn sceShaccCgReleaseCompiler();
}
extern "C" {
    pub fn sceShaccCgGetVersionString() -> *const crate::ctypes::c_char;
}
