/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub type SceKernelThreadEntry = ::core::option::Option<
    unsafe extern "C" fn(args: SceSize, argp: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
>;
#[repr(C)]
pub struct SceKernelThreadOptParam {
    pub size: SceSize,
    pub attr: SceUInt32,
}
#[repr(C)]
pub struct SceKernelThreadInfo {
    pub size: SceSize,
    pub processId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt32,
    pub status: SceUInt32,
    pub entry: SceKernelThreadEntry,
    pub stack: *mut crate::ctypes::c_void,
    pub stackSize: SceInt32,
    pub initPriority: SceInt32,
    pub currentPriority: SceInt32,
    pub initCpuAffinityMask: SceInt32,
    pub currentCpuAffinityMask: SceInt32,
    pub currentCpuId: SceInt32,
    pub lastExecutedCpuId: SceInt32,
    pub waitType: SceUInt32,
    pub waitId: SceUID,
    pub exitStatus: SceInt32,
    pub runClocks: SceKernelSysClock,
    pub intrPreemptCount: SceUInt32,
    pub threadPreemptCount: SceUInt32,
    pub threadReleaseCount: SceUInt32,
    pub changeCpuCount: SceInt32,
    pub fNotifyCallback: SceInt32,
    pub reserved: SceInt32,
}
#[repr(C)]
pub struct SceKernelThreadRunStatus {
    pub size: SceSize,
    pub cpuInfo: [SceKernelThreadRunStatus__bindgen_ty_1; 4usize],
}
#[repr(C)]
pub struct SceKernelThreadRunStatus__bindgen_ty_1 {
    pub processId: SceUID,
    pub threadId: SceUID,
    pub priority: crate::ctypes::c_int,
}
pub mod SceThreadStatus {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_THREAD_RUNNING: Type = 1;
    pub const SCE_THREAD_READY: Type = 2;
    pub const SCE_THREAD_STANDBY: Type = 4;
    pub const SCE_THREAD_WAITING: Type = 8;
    pub const SCE_THREAD_DORMANT: Type = 16;
    pub const SCE_THREAD_DELETED: Type = 32;
    pub const SCE_THREAD_DEAD: Type = 64;
    pub const SCE_THREAD_STAGNANT: Type = 128;
    pub const SCE_THREAD_SUSPENDED: Type = 256;
}
extern "C" {
    pub fn ksceKernelCreateThread(
        name: *const crate::ctypes::c_char,
        entry: SceKernelThreadEntry,
        initPriority: crate::ctypes::c_int,
        stackSize: SceSize,
        attr: SceUInt,
        cpuAffinityMask: crate::ctypes::c_int,
        option: *const SceKernelThreadOptParam,
    ) -> SceUID;
}
extern "C" {
    pub fn ksceKernelDeleteThread(thid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelStartThread(
        thid: SceUID,
        arglen: SceSize,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelExitThread(status: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelExitDeleteThread(status: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelWaitThreadEnd(
        thid: SceUID,
        stat: *mut crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelWaitThreadEndCB(
        thid: SceUID,
        stat: *mut crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDelayThread(delay: SceUInt) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDelayThreadCB(delay: SceUInt) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelChangeThreadPriority(
        thid: SceUID,
        priority: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadId() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadCurrentPriority() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadStackFreeSize(thid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadInfo(
        thid: SceUID,
        info: *mut SceKernelThreadInfo,
    ) -> crate::ctypes::c_int;
}
