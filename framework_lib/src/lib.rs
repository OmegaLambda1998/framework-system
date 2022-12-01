#![cfg_attr(feature = "uefi", no_std)]
#![feature(prelude_import)]

#[cfg(feature = "uefi")]
#[macro_use]
extern crate uefi_std as std;

#[cfg(feature = "uefi")]
#[allow(unused_imports)]
#[prelude_import]
use std::prelude::*;

#[cfg(feature = "uefi")]
extern crate alloc;

#[cfg(feature = "linux_pio")]
#[macro_use]
extern crate lazy_static;

pub mod capsule;
pub mod chromium_ec;
pub mod commandline;
pub mod csme;
pub mod ec_binary;
pub mod esrt;
mod os_specific;
pub mod pd_binary;
pub mod power;
pub mod smbios;
#[cfg(feature = "uefi")]
pub mod uefi;
mod util;
