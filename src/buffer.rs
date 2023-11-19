use std::os::fd::AsRawFd;

use anyhow::Result;
use memmap2::MmapMut;
use wayland_client as wc;
use wayland_client::protocol::{wl_shm, wl_buffer, wl_shm_pool};

use crate::State;

pub struct BufferDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: wl_shm::Format,
}

impl BufferDescriptor {
    pub fn size(&self) -> usize {
        self.height as usize * self.stride as usize
    }
}

pub struct MmapBuffer {
    pub id: wl_buffer::WlBuffer,
    pub desc: BufferDescriptor,
    pub bytes: MmapMut,
}

struct UserData;

impl MmapBuffer {
    pub(crate) fn from_shm_pool(desc: BufferDescriptor,
                         fd: std::os::fd::OwnedFd,
                         shm_pool: &wl_shm_pool::WlShmPool,
                         qh: &wayland_client::QueueHandle<State>,
    ) -> Result<MmapBuffer>
    {
        let bytes = unsafe { MmapMut::map_mut(fd.as_raw_fd())? };
        let id = shm_pool.create_buffer(0, 512, 512, 512 * 4, wl_shm::Format::Argb8888, &qh, UserData);
        Ok(MmapBuffer { id, desc, bytes })
    }
}

impl wc::Dispatch<wl_buffer::WlBuffer, UserData> for State {
    fn event(
        _state: &mut State,
        _proxy: &wl_buffer::WlBuffer,
        event: wl_buffer::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<State>,
    ) {
        log::trace!("Got unexpected wl_buffer event: {:?}", event);
    }
}
