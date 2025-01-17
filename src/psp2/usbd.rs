/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const USB_DESCRIPTOR_DEVICE: u32 = 1;
pub const USB_DESCRIPTOR_CONFIGURATION: u32 = 2;
pub const USB_DESCRIPTOR_STRING: u32 = 3;
pub const USB_DESCRIPTOR_INTERFACE: u32 = 4;
pub const USB_DESCRIPTOR_ENDPOINT: u32 = 5;
pub const USB_DESCRIPTOR_DEVICE_QUALIFIER: u32 = 6;
pub const USB_DESCRIPTOR_OTHER_SPEED: u32 = 7;
pub const USB_DESCRIPTOR_INTERFACE_POWER: u32 = 8;
pub const USB_DESCRIPTOR_OTG: u32 = 9;
pub const SCE_USBD_CLASS_PER_INTERFACE: u32 = 0;
pub const SCE_USBD_CLASS_AUDIO: u32 = 1;
pub const SCE_USBD_CLASS_COMMUNICATIONS: u32 = 2;
pub const SCE_USBD_CLASS_HID: u32 = 3;
pub const SCE_USBD_CLASS_MONITOR: u32 = 4;
pub const SCE_USBD_CLASS_PHYSICAL: u32 = 5;
pub const SCE_USBD_CLASS_POWER: u32 = 6;
pub const SCE_USBD_CLASS_PRINTER: u32 = 7;
pub const SCE_USBD_CLASS_STORAGE: u32 = 8;
pub const SCE_USBD_CLASS_HUB: u32 = 9;
pub const SCE_USBD_CLASS_DATA: u32 = 10;
pub const SCE_USBD_CLASS_VENDOR_SPECIFIC: u32 = 255;
pub const SCE_USBD_CONFIGURATION_RESERVED_ZERO: u32 = 31;
pub const SCE_USBD_CONFIGURATION_REMOTE_WAKEUP: u32 = 32;
pub const SCE_USBD_CONFIGURATION_SELF_POWERED: u32 = 64;
pub const SCE_USBD_CONFIGURATION_RESERVED_ONE: u32 = 128;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_BITS: u32 = 3;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_SHIFT: u32 = 0;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_CONTROL: u32 = 0;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_ISOCHRONOUS: u32 = 1;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_BULK: u32 = 2;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_INTERRUPT: u32 = 3;
pub const SCE_USBD_ENDPOINT_NUMBER_BITS: u32 = 31;
pub const SCE_USBD_ENDPOINT_NUMBER_SHIFT: u32 = 0;
pub const SCE_USBD_ENDPOINT_DIRECTION_BITS: u32 = 128;
pub const SCE_USBD_ENDPOINT_DIRECTION_SHIFT: u32 = 7;
pub const SCE_USBD_ENDPOINT_DIRECTION_OUT: u32 = 0;
pub const SCE_USBD_ENDPOINT_DIRECTION_IN: u32 = 128;
pub const SCE_USBD_DEVICE_SPEED_LS: u32 = 0;
pub const SCE_USBD_DEVICE_SPEED_FS: u32 = 1;
pub const SCE_USBD_DEVICE_SPEED_HS: u32 = 2;
pub mod SceUsbdErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_ERROR_NOT_INITIALIZED: Type = 2149842945;
    pub const SCE_USBD_ERROR_ALREADY_INITIALIZED: Type = 2149842946;
    pub const SCE_USBD_ERROR_INVALID_PARAM: Type = 2149842947;
    pub const SCE_USBD_ERROR_PIPE_NOT_FOUND: Type = 2149842948;
    pub const SCE_USBD_ERROR_NO_MEMORY: Type = 2149842949;
    pub const SCE_USBD_ERROR_DEVICE_NOT_FOUND: Type = 2149842950;
    pub const SCE_USBD_ERROR_80240007: Type = 2149842951;
    pub const SCE_USBD_ERROR_80240009: Type = 2149842953;
    pub const SCE_USBD_ERROR_8024000A: Type = 2149842954;
    pub const SCE_USBD_ERROR_FATAL: Type = 2149843199;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceInfo {
    pub port: crate::ctypes::c_uint,
    pub device_num: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdTransferData {
    pub pipe: crate::ctypes::c_uint,
    pub data: *const crate::ctypes::c_void,
    pub data_size: crate::ctypes::c_uint,
    pub transferred: *mut crate::ctypes::c_void,
    pub timeout: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdReceiveEvent {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
    pub transfer_id: crate::ctypes::c_uint,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceAddress {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_ushort,
}
extern "C" {
    pub fn sceUsbdInit(uid: *mut SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdEnd(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDeviceList(
        uid: SceUID,
        num: SceSize,
        info: *mut SceUsbdDeviceInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDescriptorSize(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDescriptor(
        uid: SceUID,
        device_id: SceUID,
        descriptor: *mut crate::ctypes::c_uchar,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
pub mod SceUsbdDescriptorType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_DESCRIPTOR_DEVICE: Type = 1;
    pub const SCE_USBD_DESCRIPTOR_CONFIGURATION: Type = 2;
    pub const SCE_USBD_DESCRIPTOR_STRING: Type = 3;
    pub const SCE_USBD_DESCRIPTOR_INTERFACE: Type = 4;
    pub const SCE_USBD_DESCRIPTOR_ENDPOINT: Type = 5;
    pub const SCE_USBD_DESCRIPTOR_DEVICE_QUALIFIER: Type = 6;
    pub const SCE_USBD_DESCRIPTOR_OTHER_SPEED: Type = 7;
    pub const SCE_USBD_DESCRIPTOR_INTERFACE_POWER: Type = 8;
    pub const SCE_USBD_DESCRIPTOR_OTG: Type = 9;
    pub const SCE_USBD_DESCRIPTOR_HID: Type = 33;
    pub const SCE_USBD_DESCRIPTOR_REPORT: Type = 34;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubclass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdConfigurationDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdInterfaceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubclass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdEndpointDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct SceUsbdStringDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bString: __IncompleteArrayField<u8>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdHidSubDescriptorInfo {
    pub bDescriptorType: u8,
    pub wDescriptorLength0: u8,
    pub wDescriptorLength1: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct SceUsbdHidDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdHID0: u8,
    pub bcdHID1: u8,
    pub bCountryCode: u8,
    pub bNumDescriptors: u8,
    pub SubDescriptorInfo: __IncompleteArrayField<SceUsbdHidSubDescriptorInfo>,
}
extern "C" {
    pub fn sceUsbdGetDeviceSpeed(
        uid: SceUID,
        device_id: SceUID,
        speed: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdRegisterCallback(
        cbid: SceUID,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdResetDevice(uid: SceUID, device_id: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdAttach(
        uid: SceUID,
        driver_id: SceUID,
        bus: SceUInt,
        device: SceUInt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDeviceAddress(
        uid: SceUID,
        device_id: SceUID,
        addr: *mut SceUsbdDeviceAddress,
    ) -> crate::ctypes::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdTransferStatus {
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdIsochTransferStatus {
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: *mut usize,
    pub unk3: u32,
}
extern "C" {
    pub fn sceUsbdGetTransferStatus(
        transfer_id: SceUID,
        status: *mut SceUsbdTransferStatus,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetIsochTransferStatus(
        transfer_id: SceUID,
        status: *mut SceUsbdIsochTransferStatus,
    ) -> crate::ctypes::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDevicePipe {
    pub device_id: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
}
extern "C" {
    pub fn sceUsbdOpenPipe(uid: SceUID, pipe: *mut SceUsbdDevicePipe) -> SceUID;
}
extern "C" {
    pub fn sceUsbdOpenDefaultPipe(uid: SceUID, device_id: SceUID) -> SceUID;
}
extern "C" {
    pub fn sceUsbdClosePipe(uid: SceUID, pipe_id: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdTransferData(uid: SceUID, data: *mut SceUsbdTransferData) -> SceUID;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdIsochTransfer {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
    pub unk6: crate::ctypes::c_uint,
    pub unk7: crate::ctypes::c_uint,
    pub unk8: crate::ctypes::c_uint,
    pub unk9: crate::ctypes::c_uint,
}
extern "C" {
    pub fn sceUsbdIsochTransferData(
        uid: SceUID,
        pipe_id: SceUID,
        transfer: *mut SceUsbdIsochTransfer,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdReceiveEvent(
        uid: SceUID,
        event: *mut SceUsbdReceiveEvent,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdRegisterLdd(uid: SceUID, name: *mut crate::ctypes::c_char) -> SceUID;
}
extern "C" {
    pub fn sceUsbdRegisterCompositeLdd(uid: SceUID, name: *mut crate::ctypes::c_char) -> SceUID;
}
extern "C" {
    pub fn sceUsbdUnregisterLdd(
        uid: SceUID,
        name: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdAttachCompositeParam {
    pub driver_id: u32,
    pub bus: u32,
    pub device: u32,
    pub unk3: u32,
    pub unk4: u32,
}
extern "C" {
    pub fn sceUsbdAttachCompositeLdd(
        uid: SceUID,
        param: *mut SceUsbdAttachCompositeParam,
    ) -> crate::ctypes::c_int;
}
