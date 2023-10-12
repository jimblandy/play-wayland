use super::State;

use anyhow::Result;
use wayland_client as wc;
use wayland_client::protocol::{wl_buffer, wl_shm, wl_shm_pool};

#[derive(Default)]
pub struct Shm {
    pub formats: Vec<wl_shm::Format>,
}

pub struct UserData;

wc::delegate_dispatch!(State: [wl_shm::WlShm: UserData] => Shm);

impl wc::Dispatch<wl_shm::WlShm, UserData, State> for Shm {
    fn event(
        state: &mut State,
        _proxy: &wl_shm::WlShm,
        event: wl_shm::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<State>,
    ) {
        match event {
            wl_shm::Event::Format { format } => {
                // Throw away unrecognized enum values.
                if let wc::WEnum::Value(format) = format {
                    state.shm.formats.push(format);
                }
            }
            other => log::trace!("Got unexpected wl_shm event: {:?}", other),
        }
    }
}

/// `name` is used only for debugging. It shows up in the symlinks in
/// /proc/PID/fds, for example.
pub(crate) fn create_pool(
    shm: &wl_shm::WlShm,
    name: &str,
    size: usize,
    qh: &wc::QueueHandle<State>,
) -> Result<wl_shm_pool::WlShmPool> {
    use nix::sys::memfd;
    use std::os::fd::AsFd;

    let name = std::ffi::CString::new(name)?;
    let size = i32::try_from(size)?; // wl_shm::create_pool takes an `int`
    let fd = memfd::memfd_create(&name, memfd::MemFdCreateFlag::empty())?;
    nix::unistd::ftruncate(fd.as_fd(), size as nix::libc::off_t)?;

    Ok(shm.create_pool(fd.as_fd(), size, qh, UserData))
}

impl wc::Dispatch<wl_shm_pool::WlShmPool, UserData> for State {
    fn event(
        state: &mut State,
        _proxy: &wl_shm_pool::WlShmPool,
        event: wl_shm_pool::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<State>,
    ) {
        log::trace!("Got unexpected wl_shm_pool event: {:?}", event);
    }
}

impl wc::Dispatch<wl_buffer::WlBuffer, UserData> for State {
    fn event(
        state: &mut State,
        _proxy: &wl_buffer::WlBuffer,
        event: wl_buffer::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<State>,
    ) {
        log::trace!("Got unexpected wl_shm_pool event: {:?}", event);
    }
}

impl Shm {
    pub(crate) fn dump_formats(&self) {
        eprintln!("formats: {:#?}", self.formats);
    }
}

