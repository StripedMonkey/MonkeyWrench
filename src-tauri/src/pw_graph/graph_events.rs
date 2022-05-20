use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum PortType {
    Audio,
    Video,
    Midi,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct NodeEvent {
    object_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PortEvent {
    object_id: u32,
    channel_type: PortType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct WireEvent {
    object_id: u32,

    input_node_id: u32,
    output_node_id: u32,

    output_port_id: u32,
    input_port_id: u32,
}

pub(crate) enum GraphEvent {
    WireEvent(WireEvent),
    PortEvent(PortEvent),
    NodeEvent(NodeEvent),
}
