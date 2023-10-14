#![allow(unused_imports, unused_variables, dead_code)]

mod shm;

use anyhow::Result;
use wayland_client as wc;
use wayland_client::protocol::{wl_callback, wl_compositor, wl_display, wl_registry, wl_shm};
use wc::protocol::{wl_shm_pool, wl_surface};
use wayland_protocols::xdg::shell::client::{xdg_wm_base, xdg_surface, xdg_toplevel};

struct State {
    exit: bool,
    shm: shm::Shm,
}

struct UserData;

fn main() -> Result<()> {
    env_logger::init();

    let connection = wc::Connection::connect_to_env()?;
    let (globals, mut queue) = wc::globals::registry_queue_init::<State>(&connection).unwrap();
    let qh = queue.handle();
    let mut state = State {
        exit: false,
        shm: shm::Shm::default(),
    };

    println!("Globals:");
    globals.contents().with_list(|globals| {
        for global in globals {
            println!(
                "    {} v{}: {}",
                global.interface, global.version, global.name
            );
        }
    });

    let shm = globals.bind::<wl_shm::WlShm, State, shm::UserData>(&qh, 1..=1, shm::UserData)?;
    let compositor = globals.bind::<wl_compositor::WlCompositor, State, UserData>(&qh, 5..=5, UserData)?;
    let xdg_wm_base = globals.bind::<xdg_wm_base::XdgWmBase, State, UserData>(&qh, 4..=4, UserData)?;
    queue.roundtrip(&mut state)?;
    assert!(state.shm.formats.contains(&wl_shm::Format::Xrgb8888));
    let (shm_pool, fd) = shm::create_pool(&shm, "play-wayland wl_shm_pool", 512 * 512 * 4, &qh)?;
    let mut mapping = unsafe { memmap2::MmapMut::map_mut(std::os::fd::AsRawFd::as_raw_fd(&fd))? };
    for (i, pixel) in mapping.chunks_mut(4).enumerate() {
        // We're going to draw a partially transparent circle filled
        // with a blue-green gradient.
        //
        // Our buffer is in `Argb8888` format: 32 bits per pixel,
        // providing red, green, blue, and alpha. The `Argb` means
        // that `A` is the most significant byte and `b` the least.
        // These appear in little-endian byte order, so `A` is the
        // third byte, and `b` is the first.
        let x = i & 511;
        let y = i >> 9;
        let in_circle = {
            let cx = x as i32 - 256;
            let cy = y as i32 - 256;
            cx * cx + cy * cy < 200 * 200
        };
        if in_circle {
            pixel[0] = (x / 2) as u8;
            pixel[1] = (y / 2) as u8;
            pixel[2] = 0;
            pixel[3] = 192; // mostly opaque
        } else {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 0; // transparent
            
        }
    }
    let buffer = shm_pool.create_buffer(0, 512, 512, 512 * 4, wl_shm::Format::Argb8888, &qh, shm::UserData);
    let surface = compositor.create_surface(&qh, UserData);
    let xdg_surface = xdg_wm_base.get_xdg_surface(&surface, &qh, UserData);
    let xdg_toplevel = xdg_surface.get_toplevel(&qh, UserData);
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    queue.roundtrip(&mut state)?;
    state.shm.dump_formats();
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}

impl wc::Dispatch<wl_registry::WlRegistry, wc::globals::GlobalListContents> for State {
    fn event(
        _state: &mut State,
        _proxy: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &wc::globals::GlobalListContents,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<Self>,
    ) {
        log::trace!("Got wl_registry event: {:?}", event);
    }
}

impl wc::Dispatch<wl_compositor::WlCompositor, UserData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &wl_compositor::WlCompositor,
        event: wl_compositor::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<Self>,
    ) {
        log::trace!("Got wl_compositor event: {:?}", event);
    }
}

impl wc::Dispatch<wl_surface::WlSurface, UserData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &wl_surface::WlSurface,
        event: wl_surface::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<Self>,
    ) {
        log::trace!("Got wl_surface event: {:?}", event);
    }
}

impl wc::Dispatch<xdg_wm_base::XdgWmBase, UserData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<Self>,
    ) {
        log::trace!("Got xdg_wm_base event: {:?}", event);
    }
}

impl wc::Dispatch<xdg_surface::XdgSurface, UserData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<Self>,
    ) {
        log::trace!("Got xdg_wm_base event: {:?}", event);
    }
}

impl wc::Dispatch<xdg_toplevel::XdgToplevel, UserData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<Self>,
    ) {
        log::trace!("Got xdg_wm_base event: {:?}", event);
    }
}
