
use kernel::{
    bindings, c_types,
    file::File,
    file_operations::{FileOpener, FileOperations, IoctlCommand, IoctlHandler, SeekFrom},
    io_buffer::{IoBufferReader, IoBufferWriter},
    prelude::*,
    sync::{Ref, RefBorrow, UniqueRef},
    user_ptr::{UserSlicePtr, UserSlicePtrReader, UserSlicePtrWriter},
};

use crate::{
    sculldevices::ScullDev,
};

pub(crate) struct ScullFile {
    dev: Ref<ScullDev>,
}

impl ScullFile {
    fn new(dev: Ref<ScullDev>) -> Result<Ref<Self>> {
        // TODO
        let mut scullfile = Pin::from(UniqueRef::try_new(Self {
            dev,
        })?);

        Ok(scullfile.into())
    }

}


impl IoctlHandler for ScullFile{
    type Target<'a> = RefBorrow<'a, ScullFile>;

    /*
     * S means "Set" through a ptr,
     * T means "Tell" directly with the argument value
     * G means "Get": reply by setting through a pointer
     * Q means "Query": response is on the return value
     * X means "eXchange": switch G and S atomically
     * H means "sHift": switch T and Q atomically
     */
    fn pure(
        this: RefBorrow<'_, ScullFile>, 
        _file: &File, 
        cmd: u32, 
        _arg: usize
    ) -> Result<i32> {
        //TODO
        // match cmd {
        //     bindings::SCULL_IOCRESET => ,
        //     bindings::SCULL_IOCTQUANTUM => ,
        //     bindings::SCULL_IOCTQSET => ,
        //     bindings::SCULL_IOCQQUANTUM => ,
        //     bindings::SCULL_IOCQQSET => ,
        //     bindings::SCULL_IOCHQUANTUM => ,
        //     bindings::SCULL_IOCHQSET => ,
        //     _ => return Err(Error::EINVAL),
        // }
        Err(Error::EINVAL)
    }

    fn read(
        this: RefBorrow<'_, ScullFile>,
        _file: &File,
        cmd: u32,
        reader: &mut UserSlicePtrReader,
    ) -> Result<i32> {
        // TODO
        // match cmd {
        //     bindings::SCULL_IOCGQUANTUM => ,
        //     bindings::SCULL_IOCGQSET => ,
        //     _ => return Err(Error::EINVAL),
        // }
        Ok(0)
    }

    fn write(
        this: RefBorrow<'_, ScullFile>,
        _file: &File,
        cmd: u32,
        writer: &mut UserSlicePtrWriter,
    ) -> Result<i32> {
        // TODO
        // match cmd {
        //     bindings::SCULL_IOCSQUANTUM =>, 
        //     bindings::SCULL_IOCSQSET => ,
        //     _ => return Err(Error::EINVAL),
        // }
        Ok(0)
    }
    
    fn read_write(
        this: RefBorrow<'_, ScullFile>,
        file: &File,
        cmd: u32,
        data: UserSlicePtr,
    ) -> Result<i32> {
        // TODO
        // match cmd {
        //     bindings::SCULL_IOCXQUANTUM =>,
        //     bindings::SCULL_IOCXQSET => ,
        //     _ => return Err(Error::EINVAL),
        // }
        Ok(0)
    }
}

impl FileOpener<Ref<ScullDev>> for ScullFile{
    fn open(dev: &Ref<ScullDev>, file: &File) -> Result<Self::Wrapper> {
        Self::new(dev.clone(), file.cred().clone())
    }
}

impl FileOperations for ScullFile {
    type Wrapper = Ref<Self>;

    /* the c version: llseek, read, write, unlocked_ioctl, open, release */
    kernel::declare_file_operations!(seek, read, write, ioctl);

    fn release(obj: Self::Wrapper, _file: &File){
        // TODO
    }

    fn seek(
        this: RefBorrow<'_, ScullFile>,
        _file: &File,
        _offset: SeekFrom,
    ){
        // TODO
    }

    /// Cleans up after the last reference to the file goes away.
    ///
    /// Note that the object is moved, so it will be freed automatically unless the implementation
    /// moves it elsewhere.
    fn read(
        this: RefBorrow<'_, ScullFile>,
        _file: &File,
        _data: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        //TODO
    }

    fn write(
        _this: RefBorrow<'_, ScullFile>,
        _file: &File,
        _data: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        // TODO
    }

    fn ioctl(
        this: RefBorrow<'_, ScullFile>, 
        file: &File, 
        cmd: &mut IoctlCommand
    ) -> Result<i32> {
        cmd.dispatch::<Self>(this, file)
    }
}