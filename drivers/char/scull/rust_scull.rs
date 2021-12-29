// SPDX-License-Identifier: GPL-2.0

//! Scull -- the linux device driver example
//!
//! TODO: This module is a work in progress.

#![no_std]
#![feature(global_asm, allocator_api, concat_idents, generic_associated_types)]

use kernel::{
    bindings,
    io_buffer::IoBufferWriter,
    linked_list::{GetLinks, GetLinksWrapped, Links},
    chrdev::Registration,
    prelude::*,
    str::CStr,
    sync::Ref,
    file::File,
};

// mod allocation;
mod context;
mod scullfile;
mod sculldevices;
// mod defs;
// mod range_alloc;

use {context::Context};

module! {
    type: ScullModule,
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

struct ScullModule {
    _reg: Pin<Box<chrdev::Registration<scull_nr_devs>>>,
    _devices: Ref<ScullDevices>,

}

impl KernelModule for ScullModule {
    fn init(name: &'static CStr, module: &'static kernel::ThisModule) -> Result<Self> {
        pr_info!("Rust scull(init)\n");

        // chardev::new_pinned does not register the device
        let mut reg = Registration::new_pinned(name, scull_minor, module)?;

        // there are $(scull_nr_devs) minors in this case.
        for _ in 0..scull_nr_devs {
            reg.as_mut().register::<ScullFile>()?;
        }

        // TODO
        // let mut devices = sculldevices::ScullDevices::new(scull_nr_devs,
        //         scull_quantum, 
        //         scull_qset,
        //         reg.into().)
        
        Ok(Self { _reg: reg })
    }
}

impl Drop for ScullModule {
    fn drop(&mut Self){
        pr_info!("Rust scull(exit)\n");
    }
}

