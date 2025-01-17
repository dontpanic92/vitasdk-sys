/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub mod SceUsbAudioInErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBAUDIO_IN_ERROR_INVALID_ARGUMENT: Type = 2151546881;
    pub const SCE_USBAUDIO_IN_ERROR_DUPLICATE_ID: Type = 2151546882;
    pub const SCE_USBAUDIO_IN_ERROR_NO_MEMORY: Type = 2151546883;
    pub const SCE_USBAUDIO_IN_ERROR_DEVICE_NOT_FOUND: Type = 2151546884;
    pub const SCE_USBAUDIO_IN_ERROR_NOT_SUPPORTED: Type = 2151546885;
    pub const SCE_USBAUDIO_IN_ERROR_CANNOT_GET_PORT_OWNERSHIP: Type = 2151546886;
    pub const SCE_USBAUDIO_IN_ERROR_PORT_IS_ALREADY_OPENED: Type = 2151546887;
    pub const SCE_USBAUDIO_IN_ERROR_PROCESS_HAS_NOT_A_DEVICE_OWNERSHIP: Type = 2151546888;
    pub const SCE_USBAUDIO_IN_ERROR_FAILED_TO_READ_STREAM: Type = 2151546889;
    pub const SCE_USBAUDIO_IN_ERROR_DEVICE_WAS_HALTED: Type = 2151546890;
    pub const SCE_USBAUDIO_IN_ERROR_NO_DATA_TO_READ: Type = 2151546891;
    pub const SCE_USBAUDIO_IN_ERROR_FAILED_TO_COPY_BUFFER: Type = 2151546892;
    pub const SCE_USBAUDIO_IN_ERROR_FAILED_TO_REQUEST_ISOCHRONOUS: Type = 2151546893;
    pub const SCE_USBAUDIO_IN_ERROR_TIMEOUT: Type = 2151546894;
    pub const SCE_USBAUDIO_IN_ERROR_PROCESS_CANNOT_OPEN_MORE_DEVICE: Type = 2151546895;
}
#[repr(C)]
pub struct SceUsbAudioInDeviceInfo {
    pub vendor: u16,
    pub product: u16,
    pub _reserved: [SceUInt32; 5usize],
}
#[repr(C)]
pub struct SceUsbAudioInDeviceListItem {
    pub device_id: SceUInt32,
}
extern "C" {
    pub fn sceUsbAudioInOpenDevice(
        device_id: SceUInt32,
        bits: crate::ctypes::c_int,
        rate: crate::ctypes::c_int,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInCloseDevice(device_id: SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInGetDeviceIdList(
        list: *mut SceUsbAudioInDeviceListItem,
        device_count: *mut SceUInt32,
        list_size: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInGetDeviceInformation(
        device_id: SceUInt32,
        info: *mut SceUsbAudioInDeviceInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInGetMaxValueOfVolume(
        device_id: SceUInt32,
        volume: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInGetMinValueOfVolume(
        device_id: SceUInt32,
        volume: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInSetCurrentValueOfVolume(
        device_id: SceUInt32,
        volume: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceUsbAudioInInput(device_id: SceUInt32, buffer: *mut crate::ctypes::c_void)
        -> SceInt32;
}
