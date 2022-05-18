use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ChannelType {
    Audio,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeEvent {
    object_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PortEvent {
    object_id: u32,
    channel_type: ChannelType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WireEvent {
    object_id: u32,

    input_node_id: u32,
    output_node_id: u32,

    output_port_id: u32,
    input_port_id: u32,
}



struct PWManager {
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
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
}
