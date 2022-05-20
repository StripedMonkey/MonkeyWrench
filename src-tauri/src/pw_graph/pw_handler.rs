use std::sync::mpsc::Sender;

use log::warn;
use pipewire::{registry::GlobalObject, spa::ForeignDict, Context, MainLoop};

use super::graph_events::GraphEvent;


struct PWManager {
    object_history: Vec<GraphEvent>,
    listeners: Vec<Box<dyn Fn()>>,
}

impl PWManager {
    fn create_loop(sender: Sender<GraphEvent>) {
        let main_loop = pipewire::MainLoop::new().unwrap();
        let context = pipewire::Context::new(&main_loop).unwrap();
        let core = context.connect(None).unwrap();
        let registry = core.get_registry().unwrap();
        
        let _listener = registry
            .add_listener_local()
            .global(move |global| sender.send(get_event(global).unwrap()).unwrap())
            .register();
    }

    fn register_listener<F: 'static + Fn()>(&mut self, listener: F) {
        self.listeners.push(Box::new(listener));
    }

    fn new() -> Self {
        let (sender, reciever) = std::sync::mpsc::channel::<GraphEvent>();

        Self {
            object_history: Vec::new(),
            listeners: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::PWManager;

    #[test]
    #[ignore]
    // Just print any GlobalObjects that are listenable for
    fn inspect_registry() {
        let mainloop = pipewire::MainLoop::new().unwrap();
        let context = pipewire::Context::new(&mainloop).unwrap();
        let core = context.connect(None).unwrap();
        let registry = core.get_registry().unwrap();
        let _listener = registry
            .add_listener_local()
            .global(|global| println!("New global: {:?}", global))
            .register();
        mainloop.run()
    }

    #[test]
    #[ignore]
    fn manager_creation() {
        let mgr = PWManager::new();
    }
}
