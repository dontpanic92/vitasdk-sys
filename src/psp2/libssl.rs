/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::rtc::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub mod SceSslErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SSL_ERROR_BEFORE_INIT: Type = 2151895041;
    pub const SCE_SSL_ERROR_ALREADY_INITED: Type = 2151895072;
    pub const SCE_SSL_ERROR_OUT_OF_MEMORY: Type = 2151895074;
    pub const SCE_SSL_ERROR_NOT_FOUND: Type = 2151895077;
    pub const SCE_SSL_ERROR_INTERNAL: Type = 2151895078;
    pub const SCE_SSL_ERROR_INVALID_FORMAT: Type = 2151895304;
    pub const SCE_SSL_ERROR_INVALID_VALUE: Type = 2151895550;
}
pub type SceSslCert = crate::ctypes::c_void;
pub type SceSslCertName = crate::ctypes::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSslMemoryPoolStats {
    pub poolSize: crate::ctypes::c_uint,
    pub maxInuseSize: crate::ctypes::c_uint,
    pub currentInuseSize: crate::ctypes::c_uint,
    pub reserved: crate::ctypes::c_int,
}
extern "C" {
    pub fn sceSslInit(poolSize: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslTerm() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetMemoryPoolStats(
        currentStat: *mut SceSslMemoryPoolStats,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetSerialNumber(
        sslCert: *mut SceSslCert,
        sboData: *mut *const crate::ctypes::c_char,
        sboLen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetNotBefore(
        sslCert: *mut SceSslCert,
        begin: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetNotAfter(
        sslCert: *mut SceSslCert,
        limit: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetNameEntryCount(certName: *mut SceSslCertName) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetNameEntryInfo(
        certName: *mut SceSslCertName,
        entryNum: crate::ctypes::c_int,
        oidname: *mut crate::ctypes::c_char,
        maxOidnameLen: crate::ctypes::c_uint,
        value: *mut crate::ctypes::c_char,
        maxValueLen: crate::ctypes::c_uint,
        valueLen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSslGetSubjectName(sslCert: *mut SceSslCert) -> *mut SceSslCertName;
}
extern "C" {
    pub fn sceSslGetIssuerName(sslCert: *mut SceSslCert) -> *mut SceSslCertName;
}
extern "C" {
    pub fn sceSslFreeSslCertName(certName: *mut SceSslCertName) -> crate::ctypes::c_int;
}
