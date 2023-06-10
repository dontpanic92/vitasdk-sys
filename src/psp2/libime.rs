/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::defs::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

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
pub const SCE_IME_WORK_BUFFER_SIZE: u32 = 20480;
pub const SCE_IME_MAX_PREEDIT_LENGTH: u32 = 30;
pub const SCE_IME_MAX_TEXT_LENGTH: u32 = 2048;
pub mod SceImeErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_ERROR_ALREADY_OPENED: Type = 2148534016;
    pub const SCE_IME_ERROR_NOT_OPENED: Type = 2148534017;
    pub const SCE_IME_ERROR_INVALID_POINTER: Type = 2148534018;
    pub const SCE_IME_ERROR_INVALID_PARAM: Type = 2148534019;
    pub const SCE_IME_ERROR_NO_MEMORY: Type = 2148534020;
    pub const SCE_IME_ERROR_CONNECTION_FAILED: Type = 2148534021;
    pub const SCE_IME_ERROR_INVALID_TEXT: Type = 2148534022;
    pub const SCE_IME_ERROR_TOO_MANY_REQUESTS: Type = 2148534023;
    pub const SCE_IME_ERROR_INVALID_SIZE: Type = 2148534032;
    pub const SCE_IME_ERROR_INVALID_INPUT_METHOD: Type = 2148534034;
    pub const SCE_IME_ERROR_INVALID_SUPPORTED_LANGUAGES: Type = 2148534035;
    pub const SCE_IME_ERROR_INVALID_TYPE: Type = 2148534036;
    pub const SCE_IME_ERROR_INVALID_OPTION: Type = 2148534037;
    pub const SCE_IME_ERROR_INVALID_WORK: Type = 2148534038;
    pub const SCE_IME_ERROR_INVALID_ARG: Type = 2148534039;
    pub const SCE_IME_ERROR_INVALID_HANDLER: Type = 2148534040;
    pub const SCE_IME_ERROR_INVALID_MAX_TEXT_LENGTH: Type = 2148534041;
    pub const SCE_IME_ERROR_INVALID_INPUT_TEXT_BUFFER: Type = 2148534042;
    pub const SCE_IME_ERROR_INVALID_RESERVED: Type = 2148534043;
    pub const SCE_IME_ERROR_INVALID_ENTER_LABEL: Type = 2148534044;
    pub const SCE_IME_ERROR_INTERNAL: Type = 2148534096;
}
pub mod SceImeLanguage {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_LANGUAGE_DANISH: Type = 1;
    pub const SCE_IME_LANGUAGE_GERMAN: Type = 2;
    pub const SCE_IME_LANGUAGE_ENGLISH: Type = 4;
    pub const SCE_IME_LANGUAGE_SPANISH: Type = 8;
    pub const SCE_IME_LANGUAGE_FRENCH: Type = 16;
    pub const SCE_IME_LANGUAGE_ITALIAN: Type = 32;
    pub const SCE_IME_LANGUAGE_DUTCH: Type = 64;
    pub const SCE_IME_LANGUAGE_NORWEGIAN: Type = 128;
    pub const SCE_IME_LANGUAGE_POLISH: Type = 256;
    pub const SCE_IME_LANGUAGE_PORTUGUESE: Type = 512;
    pub const SCE_IME_LANGUAGE_RUSSIAN: Type = 1024;
    pub const SCE_IME_LANGUAGE_FINNISH: Type = 2048;
    pub const SCE_IME_LANGUAGE_SWEDISH: Type = 4096;
    pub const SCE_IME_LANGUAGE_JAPANESE: Type = 8192;
    pub const SCE_IME_LANGUAGE_KOREAN: Type = 16384;
    pub const SCE_IME_LANGUAGE_SIMPLIFIED_CHINESE: Type = 32768;
    pub const SCE_IME_LANGUAGE_TRADITIONAL_CHINESE: Type = 65536;
    pub const SCE_IME_LANGUAGE_PORTUGUESE_BR: Type = 131072;
    pub const SCE_IME_LANGUAGE_ENGLISH_GB: Type = 262144;
    pub const SCE_IME_LANGUAGE_TURKISH: Type = 524288;
}
pub mod SceImeType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_TYPE_DEFAULT: Type = 0;
    pub const SCE_IME_TYPE_BASIC_LATIN: Type = 1;
    pub const SCE_IME_TYPE_NUMBER: Type = 2;
    pub const SCE_IME_TYPE_EXTENDED_NUMBER: Type = 3;
    pub const SCE_IME_TYPE_URL: Type = 4;
    pub const SCE_IME_TYPE_MAIL: Type = 5;
}
pub mod SceImeEnterLabel {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_ENTER_LABEL_DEFAULT: Type = 0;
    pub const SCE_IME_ENTER_LABEL_SEND: Type = 1;
    pub const SCE_IME_ENTER_LABEL_SEARCH: Type = 2;
    pub const SCE_IME_ENTER_LABEL_GO: Type = 3;
}
pub mod SceImeOption {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_OPTION_MULTILINE: Type = 1;
    pub const SCE_IME_OPTION_NO_AUTO_CAPITALIZATION: Type = 2;
    pub const SCE_IME_OPTION_NO_ASSISTANCE: Type = 4;
}
pub mod SceImeEvent {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IME_EVENT_OPEN: Type = 0;
    pub const SCE_IME_EVENT_UPDATE_TEXT: Type = 1;
    pub const SCE_IME_EVENT_UPDATE_CARET: Type = 2;
    pub const SCE_IME_EVENT_CHANGE_SIZE: Type = 3;
    pub const SCE_IME_EVENT_PRESS_CLOSE: Type = 4;
    pub const SCE_IME_EVENT_PRESS_ENTER: Type = 5;
}
#[repr(C)]
pub struct SceImeRect {
    pub x: SceUInt32,
    pub y: SceUInt32,
    pub width: SceUInt32,
    pub height: SceUInt32,
}
#[repr(C)]
pub struct SceImeEditText {
    pub preeditIndex: SceUInt32,
    pub preeditLength: SceUInt32,
    pub caretIndex: SceUInt32,
    pub str_: *mut SceWChar16,
    pub editIndex: SceUInt32,
    pub editLengthChange: SceInt32,
}
#[repr(C)]
pub struct SceImeEventParam {
    pub rect: __BindgenUnionField<SceImeRect>,
    pub text: __BindgenUnionField<SceImeEditText>,
    pub caretIndex: __BindgenUnionField<SceUInt32>,
    pub reserved: __BindgenUnionField<[SceUChar8; 40usize]>,
    pub bindgen_union_field: [u32; 10usize],
}
#[repr(C)]
pub struct SceImeEventData {
    pub id: SceUInt32,
    pub param: SceImeEventParam,
}
#[repr(C)]
pub struct SceImeCaret {
    pub x: SceUInt32,
    pub y: SceUInt32,
    pub height: SceUInt32,
    pub index: SceUInt32,
}
#[repr(C)]
pub struct SceImePreeditGeometry {
    pub x: SceUInt32,
    pub y: SceUInt32,
    pub height: SceUInt32,
}
pub type SceImeTextFilter = ::core::option::Option<
    unsafe extern "C" fn(
        outText: *mut SceWChar16,
        outTextLength: *mut SceUInt32,
        srcText: *const SceWChar16,
        srcTextLength: SceUInt32,
    ) -> SceInt32,
>;
pub type SceImeEventHandler = ::core::option::Option<
    unsafe extern "C" fn(arg: *mut crate::ctypes::c_void, e: *const SceImeEventData),
>;
#[repr(C)]
pub struct SceImeParam {
    pub sdkVersion: SceUInt32,
    pub inputMethod: SceUInt32,
    pub supportedLanguages: SceUInt64,
    pub languagesForced: SceBool,
    pub type_: SceUInt32,
    pub option: SceUInt32,
    pub work: *mut crate::ctypes::c_void,
    pub arg: *mut crate::ctypes::c_void,
    pub handler: SceImeEventHandler,
    pub filter: SceImeTextFilter,
    pub initialText: *mut SceWChar16,
    pub maxTextLength: SceUInt32,
    pub inputTextBuffer: *mut SceWChar16,
    pub enterLabel: SceUChar8,
    pub reserved: [SceUChar8; 7usize],
}
extern "C" {
    pub fn sceImeOpen(param: *const SceImeParam) -> SceInt32;
}
extern "C" {
    pub fn sceImeUpdate() -> SceInt32;
}
extern "C" {
    pub fn sceImeSetText(text: *const SceWChar16, length: SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceImeSetCaret(caret: *const SceImeCaret) -> SceInt32;
}
extern "C" {
    pub fn sceImeSetPreeditGeometry(preedit: *const SceImePreeditGeometry) -> SceInt32;
}
extern "C" {
    pub fn sceImeClose() -> SceInt32;
}