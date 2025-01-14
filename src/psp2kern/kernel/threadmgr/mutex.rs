/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_KERNEL_MUTEX_ATTR_RECURSIVE: u32 = 2;
#[repr(C)]
pub struct SceKernelMutexOptParam {
    pub size: SceSize,
    pub ceilingPriority: crate::ctypes::c_int,
}
#[repr(C)]
pub struct SceKernelMutexInfo {
    pub size: SceSize,
    pub mutexId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub initCount: crate::ctypes::c_int,
    pub currentCount: crate::ctypes::c_int,
    pub currentOwnerId: SceUID,
    pub numWaitThreads: crate::ctypes::c_int,
}
extern "C" {
    pub fn ksceKernelCreateMutex(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initCount: crate::ctypes::c_int,
        option: *mut SceKernelMutexOptParam,
    ) -> SceUID;
}
extern "C" {
    pub fn ksceKernelDeleteMutex(mutexid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelLockMutex(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelTryLockMutex(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelUnlockMutex(
        mutexid: SceUID,
        unlockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelCancelMutex(
        mutexid: SceUID,
        newCount: crate::ctypes::c_int,
        numWaitThreads: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetMutexInfo(
        mutexid: SceUID,
        info: *mut SceKernelMutexInfo,
    ) -> crate::ctypes::c_int;
}
