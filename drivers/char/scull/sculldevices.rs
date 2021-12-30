// SPDX-License-Identifier: GPL-2.0

use kernel::{
    bindings,
    prelude::*,
    security,
    sync::{Mutex, Ref, UniqueRef},
    file_operations,
    linked_list::List,
};

use {
    scullfile::ScullFile,
};

pub struct ScullQset {
    // void** data
}

pub(crate) struct ScullDev {
    data: List<Box<ScullQset>>,   /* Pointer to first quantum set */
    quantum: i32,           /* the current quantum size */
    qset: i32,      /* the current array size */
    size: u64,      /* amount of data stored here */
    // cdev: &Cdev, /* Char device structure		*/
}

pub(crate) struct ScullDevices {
    scull_devices: Vec<Mutex<ScullDev>>,
}


impl ScullDevices {
    pub(crate) fn new(
        scull_nr_devs: u32,
        scull_quantum: u32,
        scull_qset: u32,
        /*, TODO cdevs*/
    ) -> Result<Ref<Self>> {
        /* allocate the devices */
        let mut devices = Pin::from(UniqueRef::try_new(Self {
            Vec::new()
        )?);
        
        /* initialize each device */
        for _ in 0..scull_nr_devs {
            let mut dev = unsafe{
                Mutex::new(ScullDev {
                    data: List::new(),
                    quantum: scull_quantum, 
                    qset: scull_qset,
                    size: 0,
                    // cdev:  TODO,
                })
            };
            devices.as_mut().try_push(dev)?;
        }
        Ok(devices)
    }

}
