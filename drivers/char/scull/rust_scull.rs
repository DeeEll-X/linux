// SPDX-License-Identifier: GPL-2.0

//! Scull -- the linux device driver example
//!
//! TODO: This module is a work in progress.

#![no_std]
#![feature(global_asm, allocator_api, concat_idents, generic_associated_types)]

use kernel::{
    chrdev::Registration,
    prelude::*,
    str::CStr,
};

mod scullfile;
mod sculldevices;

use {
    scullfile::ScullFile
};

module! {
    type: ScullModule<scull_nr_devs>,
    name: b"rust_scull",
    author: b"Xiang Yuxin",
    description: b"Linux Device Driver Example in Rust",
    license: b"GPL v2",
    params: {
        scull_minor: u32 {
            default: 0,
            permissions: 0o444,
            description: b"scull minor number",
        },
        scull_nr_devs: u32 {
            default: bindings::SCULL_NR_DEVS,
            permissions: 0o444,
            description: b"scull device quantity",
        },
        scull_quantum: u32 {
            default: bindings::SCULL_QUANTUM,
            permissions: 0o444,
            description: b"scull quantum",
        },
        scull_qset: u32 {
            default: bindings::SCULL_QSET,
            permissions: 0o444,
            description: b"scull qset",
        }
    }
}

struct ScullModule<const N:usize> {
    _reg: Pin<Box<Registration<N>>>
}

impl<const N:usize> KernelModule for ScullModule<N> {
    fn init(name: &'static CStr, module: &'static kernel::ThisModule) -> Result<Self> {
        // chardev::new_pinned does not register the device
        let mut reg = Registration::new_pinned(name, scull_minor, module)?;

        // there are $(scull_nr_devs) minors in this case.
        for _ in 0..N {
            reg.as_mut().register::<ScullFile>()?;
        }

        Ok(Self { _reg: reg })
    }
}

impl<const N:usize> Drop for ScullModule<N> {
    fn drop(&mut self){
    }
}

