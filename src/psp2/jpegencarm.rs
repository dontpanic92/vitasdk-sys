/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_JPEGENCARM_MIN_COMP_RATIO: u32 = 1;
pub const SCE_JPEGENCARM_DEFAULT_COMP_RATIO: u32 = 64;
pub const SCE_JPEGENCARM_MAX_COMP_RATIO: u32 = 255;
pub type SceJpegArmEncoderContext = *mut crate::ctypes::c_void;
pub mod SceJpegEncArmErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENCARM_ERROR_IMAGE_SIZE: Type = 2154103552;
    pub const SCE_JPEGENCARM_ERROR_INSUFFICIENT_BUFFER: Type = 2154103553;
    pub const SCE_JPEGENCARM_ERROR_INVALID_COMP_RATIO: Type = 2154103554;
    pub const SCE_JPEGENCARM_ERROR_INVALID_PIXELFORMAT: Type = 2154103555;
    pub const SCE_JPEGENCARM_ERROR_INVALID_HEADER_MODE: Type = 2154103556;
    pub const SCE_JPEGENCARM_ERROR_INVALID_POINTER: Type = 2154103557;
}
pub mod SceJpegArmEncoderPixelFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENCARM_PIXELFORMAT_YCBCR420: Type = 8;
    pub const SCE_JPEGENCARM_PIXELFORMAT_YCBCR422: Type = 9;
}
pub mod SceJpegArmEncoderHeaderMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENCARM_HEADER_MODE_JPEG: Type = 0;
    pub const SCE_JPEGENCARM_HEADER_MODE_MJPEG: Type = 1;
}
extern "C" {
    pub fn sceJpegArmEncoderGetContextSize() -> SceSize;
}
extern "C" {
    pub fn sceJpegArmEncoderInit(
        context: SceJpegArmEncoderContext,
        inWidth: SceUInt16,
        inHeight: SceUInt16,
        pixelformat: SceJpegArmEncoderPixelFormat::Type,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegArmEncoderEnd(context: SceJpegArmEncoderContext) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegArmEncoderEncode(
        context: SceJpegArmEncoderContext,
        inBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegArmEncoderSetCompressionRatio(
        context: SceJpegArmEncoderContext,
        ratio: SceUInt8,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegArmEncoderSetOutputAddr(
        context: SceJpegArmEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegArmEncoderSetValidRegion(
        context: SceJpegArmEncoderContext,
        regionWidth: SceUInt16,
        regionHeight: SceUInt16,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegArmEncoderSetHeaderMode(
        context: SceJpegArmEncoderContext,
        mode: SceJpegArmEncoderHeaderMode::Type,
    ) -> crate::ctypes::c_int;
}
