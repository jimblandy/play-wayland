mod buffer;
mod draw;
mod registry;
mod shm;
mod react;

use anyhow::Result;
use wayland_client as wc;
use wayland_client::protocol::{wl_compositor, wl_registry, wl_shm};
use wc::protocol::wl_surface;
use wayland_protocols::xdg::shell::client::{xdg_wm_base, xdg_surface, xdg_toplevel};

struct State {
    shm: shm::Shm,
}

struct UserData;

fn main() -> Result<()> {
    env_logger::init();

    let connection = wc::Connection::connect_to_env()?;
    let mut state = State {
        shm: shm::Shm::default(),
    };

    let queue = connection.new_event_queue();
    let display = connection.display();
    let qh = queue.handle();
    let registry_data = registry::new();
    let registry = display.get_registry(&qh, registry_data.clone());
    queue.roundtrip(&mut state)?;
    {
        println!("Globals:");
        let registry_data = registry_data.lock().unwrap();
        for (name, global) in &registry_data.globals {
            println!(
                "    {} v{}: {}",
                global.interface, global.version, name
            );
        }
    }

    let shm = registry.bind::<wl_shm::WlShm, State, shm::UserData>(&qh, 1..=1, shm::UserData)?;
    let compositor = globals.bind::<wl_compositor::WlCompositor, State, UserData>(&qh, 5..=5, UserData)?;
    let xdg_wm_base = globals.bind::<xdg_wm_base::XdgWmBase, State, UserData>(&qh, 4..=4, UserData)?;
    queue.roundtrip(&mut state)?;
    assert!(state.shm.formats.contains(&wl_shm::Format::Argb8888));
    let desc = buffer::BufferDescriptor {
        width: 512,
        height: 512,
        stride: 512 * 4,
        format: wl_shm::Format::Argb8888,
    };
    let (shm_pool, fd) = shm::create_pool(&shm, "play-wayland wl_shm_pool", desc.size(), &qh)?;
    let mut buffer = buffer::MmapBuffer::from_shm_pool(desc, fd, &shm_pool, &qh)?;
    draw::draw(&mut buffer)?;
    let surface = compositor.create_surface(&qh, UserData);
    let xdg_surface = xdg_wm_base.get_xdg_surface(&surface, &qh, UserData);
    let xdg_toplevel = xdg_surface.get_toplevel(&qh, UserData);
    surface.attach(Some(&buffer.id), 0, 0);
    surface.commit();
    queue.roundtrip(&mut state)?;
    state.shm.dump_formats();
    std::thread::sleep(std::time::Duration::from_secs(5));

    drop(xdg_toplevel);
    Ok(())
}

impl wc::Dispatch<wl_registry::WlRegistry, registry::UserData> for State {
    fn event(
        state: &mut State,
        proxy: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        data: &registry::UserData,
        conn: &wc::Connection,
        qhandle: &wc::QueueHandle<Self>,
    ) {
        registry::RegistryState::event(state, proxy, event, data, conn, qhandle);
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
