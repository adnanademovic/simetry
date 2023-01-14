use anyhow::{bail, Result};
use log::warn;
use shared_memory::{Shmem, ShmemConf};
use std::marker::PhantomData;
use std::mem::size_of;
use typename::TypeName;

pub struct SharedMemTyped<T: TypeName> {
    memory: Shmem,
    _phantom: PhantomData<T>,
}

impl<T: TypeName> SharedMemTyped<T> {
    pub fn open(src: &str) -> Result<Self> {
        let memory = ShmemConf::new().os_id(src).open()?;
        let memory_size = unsafe { memory.as_slice().len() };
        let structure_size = size_of::<T>();
        if memory_size < structure_size {
            bail!("Data does not fit in the shared memory");
        }
        if memory_size > structure_size {
            warn!(
                "Structure {} of size {} is reading from larger memory of size {}",
                T::type_name(),
                structure_size,
                memory_size,
            );
        }
        Ok(Self {
            memory,
            _phantom: PhantomData,
        })
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_raw(&self) -> &T {
        &(*(self.memory.as_ptr() as *const T))
    }
}
