/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const SCE_AUDIODEC_ALIGNMENT_SIZE: u32 = 256;
pub const SCE_AUDIODEC_WORD_LENGTH_16BITS: u32 = 16;
pub const SCE_AUDIODEC_AT9_MAX_CH_IN_LIBRARY: u32 = 16;
pub const SCE_AUDIODEC_MP3_MAX_STREAMS: u32 = 8;
pub const SCE_AUDIODEC_AAC_MAX_STREAMS: u32 = 8;
pub const SCE_AUDIODEC_CELP_MAX_STREAMS: u32 = 8;
pub const SCE_AUDIODEC_AT9_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_AUDIODEC_MP3_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_AUDIODEC_AAC_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_AUDIODEC_CELP_MAX_CH_IN_DECODER: u32 = 1;
pub const SCE_AUDIODEC_AT9_MAX_SAMPLES: u32 = 256;
pub const SCE_AUDIODEC_MP3_MAX_SAMPLES: u32 = 1152;
pub const SCE_AUDIODEC_AAC_MAX_SAMPLES: u32 = 2048;
pub const SCE_AUDIODEC_CELP_MAX_SAMPLES: u32 = 320;
pub const SCE_AUDIODEC_AT9_MAX_ES_SIZE: u32 = 1024;
pub const SCE_AUDIODEC_MP3_MAX_ES_SIZE: u32 = 1441;
pub const SCE_AUDIODEC_AAC_MAX_ES_SIZE: u32 = 1536;
pub const SCE_AUDIODEC_CELP_MAX_ES_SIZE: u32 = 24;
pub const SCE_AUDIODEC_AT9_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_MP3_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_AAC_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_CELP_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_AT9_MAX_NFRAMES: u32 = 8;
pub const SCE_AUDIODEC_MP3_MAX_NFRAMES: u32 = 1;
pub const SCE_AUDIODEC_AAC_MAX_NFRAMES: u32 = 1;
pub const SCE_AUDIODEC_CELP_MAX_NFRAMES: u32 = 1;
pub const SCE_AUDIODEC_AT9_MAX_NSTREAMS: u32 = 6;
pub const SCE_AUDIODEC_MP3_MAX_NSTREAMS: u32 = 6;
pub const SCE_AUDIODEC_AAC_MAX_NSTREAMS: u32 = 6;
pub const SCE_AUDIODEC_CELP_MAX_NSTREAMS: u32 = 7;
pub const SCE_AUDIODEC_CELP_MPE: u32 = 0;
pub const SCE_AUDIODEC_CELP_SAMPLING_RATE_8KHZ: u32 = 8000;
pub mod SceAudiodecErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_AUDIODEC_ERROR_API_FAIL: Type = 2155806720;
    pub const SCE_AUDIODEC_ERROR_INVALID_TYPE: Type = 2155806721;
    pub const SCE_AUDIODEC_ERROR_INVALID_INIT_PARAM: Type = 2155806722;
    pub const SCE_AUDIODEC_ERROR_ALREADY_INITIALIZED: Type = 2155806723;
    pub const SCE_AUDIODEC_ERROR_OUT_OF_MEMORY: Type = 2155806724;
    pub const SCE_AUDIODEC_ERROR_NOT_INITIALIZED: Type = 2155806725;
    pub const SCE_AUDIODEC_ERROR_A_HANDLE_IN_USE: Type = 2155806726;
    pub const SCE_AUDIODEC_ERROR_ALL_HANDLES_IN_USE: Type = 2155806727;
    pub const SCE_AUDIODEC_ERROR_INVALID_PTR: Type = 2155806728;
    pub const SCE_AUDIODEC_ERROR_INVALID_HANDLE: Type = 2155806729;
    pub const SCE_AUDIODEC_ERROR_NOT_HANDLE_IN_USE: Type = 2155806730;
    pub const SCE_AUDIODEC_ERROR_CH_SHORTAGE: Type = 2155806731;
    pub const SCE_AUDIODEC_ERROR_INVALID_WORD_LENGTH: Type = 2155806732;
    pub const SCE_AUDIODEC_ERROR_INVALID_SIZE: Type = 2155806733;
    pub const SCE_AUDIODEC_ERROR_UNSUPPORTED: Type = 2155806734;
    pub const SCE_AUDIODEC_ERROR_INVALID_NFRAMES: Type = 2155806735;
    pub const SCE_AUDIODEC_ERROR_INVALID_NSTREAMS: Type = 2155806736;
    pub const SCE_AUDIODEC_ERROR_DIFFERENT_TYPES: Type = 2155806737;
    pub const SCE_AUDIODEC_ERROR_SAME_HANDLES: Type = 2155806738;
    pub const SCE_AUDIODEC_ERROR_BUSY: Type = 2155806739;
    pub const SCE_AUDIODEC_AT9_ERROR_INVALID_CONFIG: Type = 2155814912;
    pub const SCE_AUDIODEC_MP3_ERROR_INVALID_CH: Type = 2155816960;
    pub const SCE_AUDIODEC_MP3_ERROR_INVALID_MPEG_VERSION: Type = 2155816961;
    pub const SCE_AUDIODEC_AAC_ERROR_INVALID_CH: Type = 2155819008;
    pub const SCE_AUDIODEC_CELP_ERROR_INVALID_CONFIG: Type = 2155821056;
}
pub mod SceAudiodecType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_AUDIODEC_TYPE_AT9: Type = 4099;
    pub const SCE_AUDIODEC_TYPE_MP3: Type = 4100;
    pub const SCE_AUDIODEC_TYPE_AAC: Type = 4101;
    pub const SCE_AUDIODEC_TYPE_CELP: Type = 4102;
}
pub mod SceAudiodecMpegVersion {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_AUDIODEC_MP3_MPEG_VERSION_2_5: Type = 0;
    pub const SCE_AUDIODEC_MP3_MPEG_VERSION_RESERVED: Type = 1;
    pub const SCE_AUDIODEC_MP3_MPEG_VERSION_2: Type = 2;
    pub const SCE_AUDIODEC_MP3_MPEG_VERSION_1: Type = 3;
}
pub mod SceAudiodecCelpBitrate {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_3850BPS: Type = 3850;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_4650BPS: Type = 4650;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_5700BPS: Type = 5700;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_6600BPS: Type = 6600;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_7300BPS: Type = 7300;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_8700BPS: Type = 8700;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_9900BPS: Type = 9900;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_10700BPS: Type = 10700;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_11800BPS: Type = 11800;
    pub const SCE_AUDIODEC_CELP_BIT_RATE_12200BPS: Type = 12200;
}
#[repr(C)]
pub struct SceAudiodecInitStreamParam {
    pub size: SceUInt32,
    pub totalStreams: SceUInt32,
}
#[repr(C)]
pub struct SceAudiodecInitChParam {
    pub size: SceUInt32,
    pub totalCh: SceUInt32,
}
#[repr(C)]
pub struct SceAudiodecInitParam {
    pub size: __BindgenUnionField<SceUInt32>,
    pub at9: __BindgenUnionField<SceAudiodecInitChParam>,
    pub mp3: __BindgenUnionField<SceAudiodecInitStreamParam>,
    pub aac: __BindgenUnionField<SceAudiodecInitStreamParam>,
    pub celp: __BindgenUnionField<SceAudiodecInitStreamParam>,
    pub bindgen_union_field: [u32; 2usize],
}
#[repr(C)]
pub struct SceAudiodecInfoAt9 {
    pub size: SceUInt32,
    pub configData: [SceUInt8; 4usize],
    pub ch: SceUInt32,
    pub bitRate: SceUInt32,
    pub samplingRate: SceUInt32,
    pub superFrameSize: SceUInt32,
    pub framesInSuperFrame: SceUInt32,
}
#[repr(C)]
pub struct SceAudiodecInfoMp3 {
    pub size: SceUInt32,
    pub ch: SceUInt32,
    pub version: SceUInt32,
}
#[repr(C)]
pub struct SceAudiodecInfoAac {
    pub size: SceUInt32,
    pub isAdts: SceUInt32,
    pub ch: SceUInt32,
    pub samplingRate: SceUInt32,
    pub isSbr: SceUInt32,
}
#[repr(C)]
pub struct SceAudiodecInfoCelp {
    pub size: SceUInt32,
    pub excitationMode: SceUInt32,
    pub samplingRate: SceUInt32,
    pub bitRate: SceUInt32,
    pub lostCount: SceUInt32,
}
#[repr(C)]
pub struct SceAudiodecInfo {
    pub size: __BindgenUnionField<SceUInt32>,
    pub at9: __BindgenUnionField<SceAudiodecInfoAt9>,
    pub mp3: __BindgenUnionField<SceAudiodecInfoMp3>,
    pub aac: __BindgenUnionField<SceAudiodecInfoAac>,
    pub celp: __BindgenUnionField<SceAudiodecInfoCelp>,
    pub bindgen_union_field: [u32; 7usize],
}
#[repr(C)]
pub struct SceAudiodecCtrl {
    pub size: SceUInt32,
    pub handle: SceInt32,
    pub pEs: *mut SceUInt8,
    pub inputEsSize: SceUInt32,
    pub maxEsSize: SceUInt32,
    pub pPcm: *mut crate::ctypes::c_void,
    pub outputPcmSize: SceUInt32,
    pub maxPcmSize: SceUInt32,
    pub wordLength: SceUInt32,
    pub pInfo: *mut SceAudiodecInfo,
}
extern "C" {
    pub fn sceAudiodecInitLibrary(
        codecType: SceUInt32,
        pInitParam: *mut SceAudiodecInitParam,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecTermLibrary(codecType: SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecCreateDecoder(pCtrl: *mut SceAudiodecCtrl, codecType: SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecDeleteDecoder(pCtrl: *mut SceAudiodecCtrl) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecDecode(pCtrl: *mut SceAudiodecCtrl) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecDecodeNFrames(pCtrl: *mut SceAudiodecCtrl, nFrames: SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecDecodeNStreams(
        pCtrls: *mut *mut SceAudiodecCtrl,
        nStreams: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecClearContext(pCtrl: *mut SceAudiodecCtrl) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecGetInternalError(
        pCtrl: *mut SceAudiodecCtrl,
        pInternalError: *mut SceInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecGetContextSize(pCtrl: *mut SceAudiodecCtrl, codecType: SceUInt32)
        -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecCreateDecoderExternal(
        pCtrl: *mut SceAudiodecCtrl,
        codecType: SceUInt32,
        vaContext: SceUIntVAddr,
        contextSize: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceAudiodecDeleteDecoderExternal(
        pCtrl: *mut SceAudiodecCtrl,
        pvaContext: *mut SceUIntVAddr,
    ) -> SceInt32;
}
