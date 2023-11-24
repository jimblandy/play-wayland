use anyhow::Result;
use wayland_client as wc;
use wc::protocol::wl_registry::{Event, WlRegistry};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct RegistryState;

pub type UserData = Arc<Mutex<Registry>>;

#[derive(Debug, Default)]
pub struct Registry {
    pub globals: HashMap<u32, Global>,
}

#[derive(Debug)]
pub struct Global {
    pub interface: String,
    pub version: u32,
}

pub fn new() -> UserData {
    Arc::new(Mutex::new(Registry::default()))
}

impl Registry {
    pub fn bind<I, U, D>(&self, registry: &wc::WlRegistry, qh: &wc::QueueHandle<D>, udata: U)
        -> Result<I>
    where I: wc::Proxy + 'static,
          U: Send + Sync + 'static,
          D: wc::Dispatch<I, U> + 'static,
    {
        
        registry.bind(
    }
}

impl<State> wc::Dispatch<WlRegistry, UserData, State> for RegistryState
where State: wc::Dispatch<WlRegistry, UserData>,
{
    fn event(
        _state: &mut State,
        _proxy: &WlRegistry,
        event: Event,
        data: &UserData,
        _conn: &wc::Connection,
        _qhandle: &wc::QueueHandle<State>,
    ) {
        log::trace!("Got wl_registry event: {:?}", event);
        match event {
            Event::Global { name, interface, version } => {
                let mut registry = data.lock().unwrap();
                registry.globals.insert(name, Global { interface, version });
            }
            Event::GlobalRemove { name } => {
                let mut registry = data.lock().unwrap();
                registry.globals.remove(&name);
            }
            _ => log::trace!("   ... unexpected"),
        }
    }
}
