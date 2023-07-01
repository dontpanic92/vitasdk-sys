/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::rtc::*;
#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_LOCATION_DATA_INVALID: f64 = -9999.0;
pub mod SceLocationErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_LOCATION_SUCCESS: Type = 0;
    pub const SCE_LOCATION_INFO_UNDETERMINED_LOCATION: Type = 2148536832;
    pub const SCE_LOCATION_INFO_INSUFFICIENT_INFORMATION: Type = 2148536833;
    pub const SCE_LOCATION_INFO_GET_LOCATION_CANCELED: Type = 2148536834;
    pub const SCE_LOCATION_INFO_DENIED_BY_USER: Type = 2148536835;
    pub const SCE_LOCATION_ERROR_INVALID_ADDRESS: Type = 2148536836;
    pub const SCE_LOCATION_ERROR_INVALID_HANDLE: Type = 2148536837;
    pub const SCE_LOCATION_ERROR_NO_MEMORY: Type = 2148536838;
    pub const SCE_LOCATION_ERROR_TOO_MANY_HANDLES: Type = 2148536839;
    pub const SCE_LOCATION_ERROR_INVALID_LOCATION_METHOD: Type = 2148536840;
    pub const SCE_LOCATION_ERROR_INVALID_HEADING_METHOD: Type = 2148536841;
    pub const SCE_LOCATION_ERROR_MULTIPLE_CALLBACK: Type = 2148536842;
    pub const SCE_LOCATION_ERROR_NOT_RUNNING_CALLBACK: Type = 2148536843;
    pub const SCE_LOCATION_ERROR_DIALOG_RESULT_NONE: Type = 2148536844;
    pub const SCE_LOCATION_ERROR_DISABLE_APPLICATION: Type = 2148536845;
    pub const SCE_LOCATION_ERROR_MULTIPLE_CONFIRM: Type = 2148536846;
    pub const SCE_LOCATION_ERROR_UNAUTHORIZED: Type = 2148536960;
    pub const SCE_LOCATION_ERROR_PROVIDER_UNAVAILABLE: Type = 2148536961;
    pub const SCE_LOCATION_ERROR_FILE_IO: Type = 2148536962;
    pub const SCE_LOCATION_ERROR_INVALID_FILE_FORMAT: Type = 2148536963;
    pub const SCE_LOCATION_ERROR_TIME_OUT: Type = 2148536964;
    pub const SCE_LOCATION_ERROR_NO_SERVER_MEMORY: Type = 2148536965;
    pub const SCE_LOCATION_ERROR_INVALID_TITLE_ID: Type = 2148536966;
    pub const SCE_LOCATION_ERROR_FATAL_ERROR: Type = 2148537087;
}
pub type SceLocationHandle = SceUInt32;
pub mod SceLocationDialogStatus {
    pub type Type = crate::ctypes::c_int;
    pub const SCE_LOCATION_DIALOG_STATUS_IDLE: Type = 0;
    pub const SCE_LOCATION_DIALOG_STATUS_RUNNING: Type = 1;
    pub const SCE_LOCATION_DIALOG_STATUS_FINISHED: Type = 2;
}
pub mod SceLocationDialogResult {
    pub type Type = SceInt32;
    pub const SCE_LOCATION_DIALOG_RESULT_NONE: Type = 0;
    pub const SCE_LOCATION_DIALOG_RESULT_DISABLE: Type = 1;
    pub const SCE_LOCATION_DIALOG_RESULT_ENABLE: Type = 2;
}
pub mod SceLocationPermissionApplicationStatus {
    pub type Type = SceInt32;
    pub const SCE_LOCATION_PERMISSION_APPLICATION_NONE: Type = 0;
    pub const SCE_LOCATION_PERMISSION_APPLICATION_INIT: Type = 1;
    pub const SCE_LOCATION_PERMISSION_APPLICATION_DENY: Type = 2;
    pub const SCE_LOCATION_PERMISSION_APPLICATION_ALLOW: Type = 3;
}
pub mod SceLocationPermissionStatus {
    pub type Type = SceInt32;
    pub const SCE_LOCATION_PERMISSION_DENY: Type = 0;
    pub const SCE_LOCATION_PERMISSION_ALLOW: Type = 1;
}
pub mod SceLocationLocationMethod {
    pub type Type = SceInt32;
    pub const SCE_LOCATION_LMETHOD_NONE: Type = 0;
    pub const SCE_LOCATION_LMETHOD_AGPS_AND_3G_AND_WIFI: Type = 1;
    pub const SCE_LOCATION_LMETHOD_GPS_AND_WIFI: Type = 2;
    pub const SCE_LOCATION_LMETHOD_WIFI: Type = 3;
    pub const SCE_LOCATION_LMETHOD_3G: Type = 4;
    pub const SCE_LOCATION_LMETHOD_GPS: Type = 5;
}
pub mod SceLocationHeadingMethod {
    pub type Type = SceInt32;
    pub const SCE_LOCATION_HMETHOD_NONE: Type = 0;
    pub const SCE_LOCATION_HMETHOD_AUTO: Type = 1;
    pub const SCE_LOCATION_HMETHOD_VERTICAL: Type = 2;
    pub const SCE_LOCATION_HMETHOD_HORIZONTAL: Type = 3;
    pub const SCE_LOCATION_HMETHOD_CAMERA: Type = 4;
}
#[repr(C)]
pub struct SceLocationLocationInfo {
    pub latitude: SceDouble64,
    pub longitude: SceDouble64,
    pub altitude: SceDouble64,
    pub accuracy: SceFloat32,
    pub reserve: SceFloat32,
    pub direction: SceFloat32,
    pub speed: SceFloat32,
    pub timestamp: SceRtcTick,
}
#[repr(C)]
pub struct SceLocationHeadingInfo {
    pub trueHeading: SceFloat32,
    pub headingVectorX: SceFloat32,
    pub headingVectorY: SceFloat32,
    pub headingVectorZ: SceFloat32,
    pub reserve: SceFloat32,
    pub reserve2: SceFloat32,
    pub timestamp: SceRtcTick,
}
pub type SceLocationLocationInfoCallback = ::core::option::Option<
    unsafe extern "C" fn(
        result: SceInt32,
        handle: SceLocationHandle,
        location: *const SceLocationLocationInfo,
        userdata: *mut crate::ctypes::c_void,
    ),
>;
pub type SceLocationHeadingInfoCallback = ::core::option::Option<
    unsafe extern "C" fn(
        result: SceInt32,
        handle: SceLocationHandle,
        heading: *const SceLocationHeadingInfo,
        userdata: *mut crate::ctypes::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceLocationPermissionInfo {
    pub parentalstatus: SceLocationPermissionStatus::Type,
    pub mainstatus: SceLocationPermissionStatus::Type,
    pub applicationstatus: SceLocationPermissionApplicationStatus::Type,
    pub unk_0x0C: crate::ctypes::c_int,
    pub unk_0x10: crate::ctypes::c_int,
}
extern "C" {
    pub fn sceLocationOpen(
        handle: *mut SceLocationHandle,
        locateMethod: SceLocationLocationMethod::Type,
        headingMethod: SceLocationHeadingMethod::Type,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationClose(handle: SceLocationHandle) -> SceInt32;
}
extern "C" {
    pub fn sceLocationReopen(
        handle: SceLocationHandle,
        locateMethod: SceLocationLocationMethod::Type,
        headingMethod: SceLocationHeadingMethod::Type,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationGetMethod(
        handle: SceLocationHandle,
        locateMethod: *mut SceLocationLocationMethod::Type,
        headingMethod: *mut SceLocationHeadingMethod::Type,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationGetLocation(
        handle: SceLocationHandle,
        locationInfo: *mut SceLocationLocationInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationCancelGetLocation(handle: SceLocationHandle) -> SceInt32;
}
extern "C" {
    pub fn sceLocationStartLocationCallback(
        handle: SceLocationHandle,
        distance: SceUInt32,
        callback: SceLocationLocationInfoCallback,
        userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationStopLocationCallback(handle: SceLocationHandle) -> SceInt32;
}
extern "C" {
    pub fn sceLocationGetHeading(
        handle: SceLocationHandle,
        headingInfo: *mut SceLocationHeadingInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationStartHeadingCallback(
        handle: SceLocationHandle,
        difference: SceUInt32,
        callback: SceLocationHeadingInfoCallback,
        userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationStopHeadingCallback(handle: SceLocationHandle) -> SceInt32;
}
extern "C" {
    pub fn sceLocationConfirm(handle: SceLocationHandle) -> SceInt32;
}
extern "C" {
    pub fn sceLocationConfirmGetStatus(
        handle: SceLocationHandle,
        status: *mut SceLocationDialogStatus::Type,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationConfirmGetResult(
        handle: SceLocationHandle,
        result: *mut SceLocationDialogResult::Type,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationConfirmAbort(handle: SceLocationHandle) -> SceInt32;
}
extern "C" {
    pub fn sceLocationGetPermission(
        handle: SceLocationHandle,
        info: *mut SceLocationPermissionInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceLocationSetGpsEmulationFile(filename: *mut crate::ctypes::c_uchar) -> SceInt32;
}
