/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

extern "C" {
    pub fn ksceAppMgrKillProcess(pid: SceUID) -> crate::ctypes::c_int;
}
#[repr(C)]
pub struct SceAppMgrLaunchParam {
    pub size: SceSize,
    pub unk_4: crate::ctypes::c_uint,
    pub unk_8: crate::ctypes::c_uint,
    pub unk_C: crate::ctypes::c_uint,
    pub unk_10: crate::ctypes::c_uint,
    pub unk_14: crate::ctypes::c_uint,
    pub unk_18: crate::ctypes::c_uint,
    pub unk_1C: crate::ctypes::c_uint,
    pub unk_20: crate::ctypes::c_uint,
    pub unk_24: crate::ctypes::c_uint,
    pub unk_28: crate::ctypes::c_uint,
    pub unk_2C: crate::ctypes::c_uint,
    pub unk_30: crate::ctypes::c_uint,
}
extern "C" {
    pub fn ksceAppMgrLaunchAppByPath(
        path: *const crate::ctypes::c_char,
        args: *const crate::ctypes::c_char,
        arg_size: SceSize,
        type_: crate::ctypes::c_uint,
        param: *const SceAppMgrLaunchParam,
        unk: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
