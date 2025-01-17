/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::net::net::*;
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
pub const SCE_NETCTL_INFO_CONFIG_NAME_LEN_MAX: u32 = 64;
pub const SCE_NETCTL_INFO_SSID_LEN_MAX: u32 = 32;
pub mod SceNetCtlInfoType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NETCTL_INFO_GET_CNF_NAME: Type = 1;
    pub const SCE_NETCTL_INFO_GET_DEVICE: Type = 2;
    pub const SCE_NETCTL_INFO_GET_ETHER_ADDR: Type = 3;
    pub const SCE_NETCTL_INFO_GET_MTU: Type = 4;
    pub const SCE_NETCTL_INFO_GET_LINK: Type = 5;
    pub const SCE_NETCTL_INFO_GET_BSSID: Type = 6;
    pub const SCE_NETCTL_INFO_GET_SSID: Type = 7;
    pub const SCE_NETCTL_INFO_GET_WIFI_SECURITY: Type = 8;
    pub const SCE_NETCTL_INFO_GET_RSSI_DBM: Type = 9;
    pub const SCE_NETCTL_INFO_GET_RSSI_PERCENTAGE: Type = 10;
    pub const SCE_NETCTL_INFO_GET_CHANNEL: Type = 11;
    pub const SCE_NETCTL_INFO_GET_IP_CONFIG: Type = 12;
    pub const SCE_NETCTL_INFO_GET_DHCP_HOSTNAME: Type = 13;
    pub const SCE_NETCTL_INFO_GET_PPPOE_AUTH_NAME: Type = 14;
    pub const SCE_NETCTL_INFO_GET_IP_ADDRESS: Type = 15;
    pub const SCE_NETCTL_INFO_GET_NETMASK: Type = 16;
    pub const SCE_NETCTL_INFO_GET_DEFAULT_ROUTE: Type = 17;
    pub const SCE_NETCTL_INFO_GET_PRIMARY_DNS: Type = 18;
    pub const SCE_NETCTL_INFO_GET_SECONDARY_DNS: Type = 19;
    pub const SCE_NETCTL_INFO_GET_HTTP_PROXY_CONFIG: Type = 20;
    pub const SCE_NETCTL_INFO_GET_HTTP_PROXY_SERVER: Type = 21;
    pub const SCE_NETCTL_INFO_GET_HTTP_PROXY_PORT: Type = 22;
}
pub mod SceNetCtlState {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NETCTL_STATE_DISCONNECTED: Type = 0;
    pub const SCE_NETCTL_STATE_CONNECTING: Type = 1;
    pub const SCE_NETCTL_STATE_FINALIZING: Type = 2;
    pub const SCE_NETCTL_STATE_CONNECTED: Type = 3;
}
pub type SceNetCtlCallback = ::core::option::Option<
    unsafe extern "C" fn(
        event_type: crate::ctypes::c_int,
        arg: *mut crate::ctypes::c_void,
    ) -> *mut crate::ctypes::c_void,
>;
#[repr(C)]
pub struct SceNetCtlInfo {
    pub cnf_name: __BindgenUnionField<[crate::ctypes::c_char; 65usize]>,
    pub device: __BindgenUnionField<crate::ctypes::c_uint>,
    pub ether_addr: __BindgenUnionField<SceNetEtherAddr>,
    pub mtu: __BindgenUnionField<crate::ctypes::c_uint>,
    pub link: __BindgenUnionField<crate::ctypes::c_uint>,
    pub bssid: __BindgenUnionField<SceNetEtherAddr>,
    pub ssid: __BindgenUnionField<[crate::ctypes::c_char; 33usize]>,
    pub wifi_security: __BindgenUnionField<crate::ctypes::c_uint>,
    pub rssi_dbm: __BindgenUnionField<crate::ctypes::c_uint>,
    pub rssi_percentage: __BindgenUnionField<crate::ctypes::c_uint>,
    pub channel: __BindgenUnionField<crate::ctypes::c_uint>,
    pub ip_config: __BindgenUnionField<crate::ctypes::c_uint>,
    pub dhcp_hostname: __BindgenUnionField<[crate::ctypes::c_char; 256usize]>,
    pub pppoe_auth_name: __BindgenUnionField<[crate::ctypes::c_char; 128usize]>,
    pub ip_address: __BindgenUnionField<[crate::ctypes::c_char; 16usize]>,
    pub netmask: __BindgenUnionField<[crate::ctypes::c_char; 16usize]>,
    pub default_route: __BindgenUnionField<[crate::ctypes::c_char; 16usize]>,
    pub primary_dns: __BindgenUnionField<[crate::ctypes::c_char; 16usize]>,
    pub secondary_dns: __BindgenUnionField<[crate::ctypes::c_char; 16usize]>,
    pub http_proxy_config: __BindgenUnionField<crate::ctypes::c_uint>,
    pub http_proxy_server: __BindgenUnionField<[crate::ctypes::c_char; 256usize]>,
    pub http_proxy_port: __BindgenUnionField<crate::ctypes::c_uint>,
    pub bindgen_union_field: [u32; 64usize],
}
#[repr(C)]
pub struct SceNetCtlNatInfo {
    pub size: crate::ctypes::c_uint,
    pub stun_status: crate::ctypes::c_int,
    pub nat_type: crate::ctypes::c_int,
    pub mapped_addr: SceNetInAddr,
}
#[repr(C)]
pub struct SceNetCtlAdhocPeerInfo {
    pub next: *mut SceNetCtlAdhocPeerInfo,
    pub inet_addr: SceNetInAddr,
}
extern "C" {
    pub fn sceNetCtlInit() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlTerm();
}
extern "C" {
    pub fn sceNetCtlCheckCallback() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlInetGetResult(
        eventType: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocGetResult(
        eventType: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlInetGetInfo(
        code: crate::ctypes::c_int,
        info: *mut SceNetCtlInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlInetGetState(state: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlGetNatInfo(natinfo: *mut SceNetCtlNatInfo) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlInetRegisterCallback(
        func: SceNetCtlCallback,
        arg: *mut crate::ctypes::c_void,
        cid: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlInetUnregisterCallback(cid: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocRegisterCallback(
        func: SceNetCtlCallback,
        arg: *mut crate::ctypes::c_void,
        cid: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocUnregisterCallback(cid: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocGetState(state: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocDisconnect() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocGetPeerList(
        buflen: *mut crate::ctypes::c_uint,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceNetCtlAdhocGetInAddr(inaddr: *mut SceNetInAddr) -> crate::ctypes::c_int;
}
