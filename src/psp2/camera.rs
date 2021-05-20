/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub mod SceCameraErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_ERROR_PARAM: Type = 2150498304;
    pub const SCE_CAMERA_ERROR_ALREADY_INIT: Type = 2150498305;
    pub const SCE_CAMERA_ERROR_NOT_INIT: Type = 2150498306;
    pub const SCE_CAMERA_ERROR_ALREADY_OPEN: Type = 2150498307;
    pub const SCE_CAMERA_ERROR_NOT_OPEN: Type = 2150498308;
    pub const SCE_CAMERA_ERROR_ALREADY_START: Type = 2150498309;
    pub const SCE_CAMERA_ERROR_NOT_START: Type = 2150498310;
    pub const SCE_CAMERA_ERROR_FORMAT_UNKNOWN: Type = 2150498311;
    pub const SCE_CAMERA_ERROR_RESOLUTION_UNKNOWN: Type = 2150498312;
    pub const SCE_CAMERA_ERROR_BAD_FRAMERATE: Type = 2150498313;
    pub const SCE_CAMERA_ERROR_TIMEOUT: Type = 2150498314;
    pub const SCE_CAMERA_ERROR_EXCLUSIVE: Type = 2150498315;
    pub const SCE_CAMERA_ERROR_ATTRIBUTE_UNKNOWN: Type = 2150498316;
    pub const SCE_CAMERA_ERROR_MAX_PROCESS: Type = 2150498317;
    pub const SCE_CAMERA_ERROR_NOT_ACTIVE: Type = 2150498318;
    pub const SCE_CAMERA_ERROR_ALREADY_READ: Type = 2150498319;
    pub const SCE_CAMERA_ERROR_NOT_MOUNTED: Type = 2150498320;
    pub const SCE_CAMERA_ERROR_DATA_RANGE_UNKNOWN: Type = 2150498321;
    pub const SCE_CAMERA_ERROR_OTHER_ALREADY_START: Type = 2150498322;
    pub const SCE_CAMERA_ERROR_FATAL: Type = 2150498559;
}
pub mod SceCameraDevice {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_DEVICE_FRONT: Type = 0;
    pub const SCE_CAMERA_DEVICE_BACK: Type = 1;
}
pub mod SceCameraPriority {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_PRIORITY_SHARE: Type = 0;
    pub const SCE_CAMERA_PRIORITY_EXCLUSIVE: Type = 1;
}
pub mod SceCameraFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_FORMAT_INVALID: Type = 0;
    pub const SCE_CAMERA_FORMAT_YUV422_PLANE: Type = 1;
    pub const SCE_CAMERA_FORMAT_YUV422_PACKED: Type = 2;
    pub const SCE_CAMERA_FORMAT_YUV420_PLANE: Type = 3;
    pub const SCE_CAMERA_FORMAT_ARGB: Type = 4;
    pub const SCE_CAMERA_FORMAT_ABGR: Type = 5;
    pub const SCE_CAMERA_FORMAT_RAW8: Type = 6;
}
pub mod SceCameraResolution {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_RESOLUTION_0_0: Type = 0;
    pub const SCE_CAMERA_RESOLUTION_640_480: Type = 1;
    pub const SCE_CAMERA_RESOLUTION_320_240: Type = 2;
    pub const SCE_CAMERA_RESOLUTION_160_120: Type = 3;
    pub const SCE_CAMERA_RESOLUTION_352_288: Type = 4;
    pub const SCE_CAMERA_RESOLUTION_176_144: Type = 5;
    pub const SCE_CAMERA_RESOLUTION_480_272: Type = 6;
    pub const SCE_CAMERA_RESOLUTION_640_360: Type = 8;
}
pub mod SceCameraFrameRate {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_FRAMERATE_3_FPS: Type = 3;
    pub const SCE_CAMERA_FRAMERATE_5_FPS: Type = 5;
    pub const SCE_CAMERA_FRAMERATE_7_FPS: Type = 7;
    pub const SCE_CAMERA_FRAMERATE_10_FPS: Type = 10;
    pub const SCE_CAMERA_FRAMERATE_15_FPS: Type = 15;
    pub const SCE_CAMERA_FRAMERATE_20_FPS: Type = 20;
    pub const SCE_CAMERA_FRAMERATE_30_FPS: Type = 30;
    pub const SCE_CAMERA_FRAMERATE_60_FPS: Type = 60;
    pub const SCE_CAMERA_FRAMERATE_120_FPS: Type = 120;
}
pub mod SceCameraExposureCompensation {
    pub type Type = crate::ctypes::c_int;
    pub const SCE_CAMERA_EV_NEGATIVE_20: Type = -20;
    pub const SCE_CAMERA_EV_NEGATIVE_17: Type = -17;
    pub const SCE_CAMERA_EV_NEGATIVE_15: Type = -15;
    pub const SCE_CAMERA_EV_NEGATIVE_13: Type = -13;
    pub const SCE_CAMERA_EV_NEGATIVE_10: Type = -10;
    pub const SCE_CAMERA_EV_NEGATIVE_7: Type = -7;
    pub const SCE_CAMERA_EV_NEGATIVE_5: Type = -5;
    pub const SCE_CAMERA_EV_NEGATIVE_3: Type = -3;
    pub const SCE_CAMERA_EV_POSITIVE_0: Type = 0;
    pub const SCE_CAMERA_EV_POSITIVE_3: Type = 3;
    pub const SCE_CAMERA_EV_POSITIVE_5: Type = 5;
    pub const SCE_CAMERA_EV_POSITIVE_7: Type = 7;
    pub const SCE_CAMERA_EV_POSITIVE_10: Type = 10;
    pub const SCE_CAMERA_EV_POSITIVE_13: Type = 13;
    pub const SCE_CAMERA_EV_POSITIVE_15: Type = 15;
    pub const SCE_CAMERA_EV_POSITIVE_17: Type = 17;
    pub const SCE_CAMERA_EV_POSITIVE_20: Type = 20;
}
pub mod SceCameraEffect {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_EFFECT_NORMAL: Type = 0;
    pub const SCE_CAMERA_EFFECT_NEGATIVE: Type = 1;
    pub const SCE_CAMERA_EFFECT_BLACKWHITE: Type = 2;
    pub const SCE_CAMERA_EFFECT_SEPIA: Type = 3;
    pub const SCE_CAMERA_EFFECT_BLUE: Type = 4;
    pub const SCE_CAMERA_EFFECT_RED: Type = 5;
    pub const SCE_CAMERA_EFFECT_GREEN: Type = 6;
}
pub mod SceCameraReverse {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_REVERSE_OFF: Type = 0;
    pub const SCE_CAMERA_REVERSE_MIRROR: Type = 1;
    pub const SCE_CAMERA_REVERSE_FLIP: Type = 2;
    pub const SCE_CAMERA_REVERSE_MIRROR_FLIP: Type = 3;
}
pub mod SceCameraSaturation {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_SATURATION_0: Type = 0;
    pub const SCE_CAMERA_SATURATION_5: Type = 5;
    pub const SCE_CAMERA_SATURATION_10: Type = 10;
    pub const SCE_CAMERA_SATURATION_20: Type = 20;
    pub const SCE_CAMERA_SATURATION_30: Type = 30;
    pub const SCE_CAMERA_SATURATION_40: Type = 40;
}
pub mod SceCameraSharpness {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_SHARPNESS_100: Type = 1;
    pub const SCE_CAMERA_SHARPNESS_200: Type = 2;
    pub const SCE_CAMERA_SHARPNESS_300: Type = 3;
    pub const SCE_CAMERA_SHARPNESS_400: Type = 4;
}
pub mod SceCameraAntiFlicker {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_ANTIFLICKER_AUTO: Type = 1;
    pub const SCE_CAMERA_ANTIFLICKER_50HZ: Type = 2;
    pub const SCE_CAMERA_ANTIFLICKER_60HZ: Type = 3;
}
pub mod SceCameraISO {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_ISO_AUTO: Type = 1;
    pub const SCE_CAMERA_ISO_100: Type = 100;
    pub const SCE_CAMERA_ISO_200: Type = 200;
    pub const SCE_CAMERA_ISO_400: Type = 400;
}
pub mod SceCameraGain {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_GAIN_AUTO: Type = 0;
    pub const SCE_CAMERA_GAIN_1: Type = 1;
    pub const SCE_CAMERA_GAIN_2: Type = 2;
    pub const SCE_CAMERA_GAIN_3: Type = 3;
    pub const SCE_CAMERA_GAIN_4: Type = 4;
    pub const SCE_CAMERA_GAIN_5: Type = 5;
    pub const SCE_CAMERA_GAIN_6: Type = 6;
    pub const SCE_CAMERA_GAIN_7: Type = 7;
    pub const SCE_CAMERA_GAIN_8: Type = 8;
    pub const SCE_CAMERA_GAIN_9: Type = 9;
    pub const SCE_CAMERA_GAIN_10: Type = 10;
    pub const SCE_CAMERA_GAIN_11: Type = 11;
    pub const SCE_CAMERA_GAIN_12: Type = 12;
    pub const SCE_CAMERA_GAIN_13: Type = 13;
    pub const SCE_CAMERA_GAIN_14: Type = 14;
    pub const SCE_CAMERA_GAIN_15: Type = 15;
    pub const SCE_CAMERA_GAIN_16: Type = 16;
}
pub mod SceCameraWhiteBalance {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_WB_AUTO: Type = 0;
    pub const SCE_CAMERA_WB_DAY: Type = 1;
    pub const SCE_CAMERA_WB_CWF: Type = 2;
    pub const SCE_CAMERA_WB_SLSA: Type = 4;
}
pub mod SceCameraBacklight {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_BACKLIGHT_OFF: Type = 0;
    pub const SCE_CAMERA_BACKLIGHT_ON: Type = 1;
}
pub mod SceCameraNightmode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_CAMERA_NIGHTMODE_OFF: Type = 0;
    pub const SCE_CAMERA_NIGHTMODE_LESS10: Type = 1;
    pub const SCE_CAMERA_NIGHTMODE_LESS100: Type = 2;
    pub const SCE_CAMERA_NIGHTMODE_OVER100: Type = 3;
}
#[repr(C)]
pub struct SceCameraInfo {
    pub size: SceSize,
    pub priority: u16,
    pub format: u16,
    pub resolution: u16,
    pub framerate: u16,
    pub width: u16,
    pub height: u16,
    pub range: u16,
    pub pad: u16,
    pub sizeIBase: SceSize,
    pub sizeUBase: SceSize,
    pub sizeVBase: SceSize,
    pub pIBase: *mut crate::ctypes::c_void,
    pub pUBase: *mut crate::ctypes::c_void,
    pub pVBase: *mut crate::ctypes::c_void,
    pub pitch: u16,
    pub buffer: u16,
}
#[repr(C)]
pub struct SceCameraRead {
    pub size: SceSize,
    pub mode: crate::ctypes::c_int,
    pub pad: crate::ctypes::c_int,
    pub status: crate::ctypes::c_int,
    pub frame: u64,
    pub timestamp: u64,
    pub sizeIBase: SceSize,
    pub sizeUBase: SceSize,
    pub sizeVBase: SceSize,
    pub pIBase: *mut crate::ctypes::c_void,
    pub pUBase: *mut crate::ctypes::c_void,
    pub pVBase: *mut crate::ctypes::c_void,
}
extern "C" {
    pub fn sceCameraOpen(
        devnum: crate::ctypes::c_int,
        pInfo: *mut SceCameraInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraClose(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraStart(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraStop(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraRead(
        devnum: crate::ctypes::c_int,
        pRead: *mut SceCameraRead,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraIsActive(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetSaturation(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetSaturation(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetBrightness(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetBrightness(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetContrast(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetContrast(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetSharpness(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetSharpness(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetReverse(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetReverse(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetEffect(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetEffect(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetEV(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetEV(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetZoom(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetZoom(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetAntiFlicker(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetAntiFlicker(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetISO(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetISO(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetGain(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetGain(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetWhiteBalance(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetWhiteBalance(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetBacklight(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetBacklight(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetNightmode(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetNightmode(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetExposureCeiling(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetExposureCeiling(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetAutoControlHold(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraSetAutoControlHold(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceCameraGetDeviceLocation(
        devnum: crate::ctypes::c_int,
        pLocation: *mut SceFVector3,
    ) -> crate::ctypes::c_int;
}