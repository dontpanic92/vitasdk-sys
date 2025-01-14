/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::display::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
pub struct SceDisplayFrameBufInfo {
    pub size: SceSize,
    pub pid: SceUID,
    pub vblankcount: crate::ctypes::c_uint,
    pub paddr: usize,
    pub framebuf: SceDisplayFrameBuf,
    pub resolution: crate::ctypes::c_uint,
}
extern "C" {
    pub fn ksceDisplaySetFrameBuf(
        pParam: *const SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplaySetFrameBufInternal(
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        pParam: *const SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetFrameBuf(
        pParam: *mut SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetProcFrameBufInternal(
        pid: SceUID,
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        info: *mut SceDisplayFrameBufInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetMaximumFrameBufResolution(
        width: *mut crate::ctypes::c_int,
        height: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetPrimaryHead() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayGetVcountInternal(display: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStart() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartInternal(
        display: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartCB() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartCBInternal(
        display: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMultiInternal(
        display: crate::ctypes::c_int,
        vcount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMultiCB(vcount: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitVblankStartMultiCBInternal(
        display: crate::ctypes::c_int,
        vcount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBuf() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBufCB() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBufMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayWaitSetFrameBufMultiCB(vcount: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterVblankStartCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayUnregisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayUnregisterVblankStartCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterFrameBufCallback(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplayRegisterFrameBufCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplaySetInvertColors(
        display: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDisplaySetOwner(
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        pid: SceUID,
    ) -> crate::ctypes::c_int;
}
