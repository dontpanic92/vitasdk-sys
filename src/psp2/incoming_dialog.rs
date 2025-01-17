/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::kernel::clib::*;
#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::defs::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub mod SceIncomingDialogStatus {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_INCOMING_DIALOG_NOT_RUNNING: Type = 0;
    pub const SCE_INCOMING_DIALOG_ACCEPTED: Type = 1;
    pub const SCE_INCOMING_DIALOG_RUNNING: Type = 2;
    pub const SCE_INCOMING_DIALOG_REJECTED: Type = 3;
    pub const SCE_INCOMING_DIALOG_CLOSED: Type = 4;
    pub const SCE_INCOMING_DIALOG_BUSY: Type = 5;
    pub const SCE_INCOMING_DIALOG_TIMEOUT: Type = 6;
}
pub mod SceIncomingDialogErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_INCOMINGDIALOG_ERROR_INVALID_ARG: Type = 2148557313;
}
#[repr(C)]
pub struct SceIncomingDialogParam {
    pub sdkVersion: SceInt32,
    pub audioPath: [SceChar8; 128usize],
    pub titleid: [SceChar8; 16usize],
    pub unk_BC: SceInt32,
    pub dialogTimer: SceUInt32,
    pub reserved1: [SceChar8; 62usize],
    pub buttonRightText: [SceWChar16; 31usize],
    pub separator0: SceInt16,
    pub buttonLeftText: [SceWChar16; 31usize],
    pub separator1: SceInt16,
    pub dialogText: [SceWChar16; 128usize],
    pub separator2: SceInt16,
}
extern "C" {
    pub fn sceIncomingDialogInitialize(init_type: crate::ctypes::c_int) -> SceInt32;
}
extern "C" {
    pub fn sceIncomingDialogOpen(dialogParam: *mut SceIncomingDialogParam) -> SceInt32;
}
extern "C" {
    pub fn sceIncomingDialogGetStatus() -> SceInt32;
}
extern "C" {
    pub fn sceIncomingDialogSwitchToDialog() -> SceInt32;
}
extern "C" {
    pub fn sceIncomingDialogClose() -> SceInt32;
}
extern "C" {
    pub fn sceIncomingDialogFinish() -> SceInt32;
}
