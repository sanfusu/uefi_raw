#![feature(abi_efiapi)]

use core::ffi::c_void;

pub mod system_table;

pub type EfiStatus = usize;
pub type EfiHandleOut = *mut c_void;
pub type EfiHandleIn = *const c_void;
pub type EfiEventOut = *mut c_void;
pub type EfiEventIn = *const c_void;
pub type EfiLba = u64;
pub type EfiTpl = usize;
pub type EfiMacAddress = [u8; 32];
pub type EfiIpv4Address = [u8; 4];
pub type EfiIpv6Address = [u8; 16];
pub type EfiIpAdress = [u32; 4];
// @TODO, need SystemTable to be defined
pub type EfiImageEntryPoint = extern "efiapi" fn(imageHandle: EfiHandleIn) -> EfiStatus;

