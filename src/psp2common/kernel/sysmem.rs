/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_L1WBWA_RW: u32 = 155205728;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_R: u32 = 155222080;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW: u32 = 155222112;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_DEVICE_RW: u32 = 203425888;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_R: u32 = 203477056;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_RW: u32 = 203477088;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_NC_RW: u32 = 203456608;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_GAME_RW: u32 = 206622816;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_RW: u32 = 209768544;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_RW: u32 = 226525280;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_CDIALOG_RW: u32 = 211865696;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_CDIALOG_NC_RW: u32 = 211845216;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_RW: u32 = 217108576;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_NC_RW: u32 = 217088096;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_R: u32 = 237031488;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_RW: u32 = 237031520;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_NC_R: u32 = 237011008;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_NC_RW: u32 = 237011040;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE: u32 = 203456608;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RW: u32 = 203477088;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_TOOL_NC_RW: u32 = 217088096;
pub type SceKernelMemBlockType = SceUInt32;
pub mod SceKernelAllocMemBlockAttr {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_PADDR: Type = 2;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_ALIGNMENT: Type = 4;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_MIRROR_BLOCKID: Type = 64;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_PID: Type = 128;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_PADDR_LIST: Type = 4096;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_PHYCONT: Type = 2097152;
    pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_ALLOW_PARTIAL_OP: Type = 67108864;
}
pub mod SceKernelModel {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_MODEL_VITA: Type = 65536;
    pub const SCE_KERNEL_MODEL_VITATV: Type = 131072;
}
