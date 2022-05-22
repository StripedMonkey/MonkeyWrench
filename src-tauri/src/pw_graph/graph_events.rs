use std::str::FromStr;

use log::{warn, trace};
use pipewire::{prelude::ReadableDict, registry::GlobalObject, spa::ForeignDict};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum PortType {
    Audio,
    Video,
    Midi,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum PortDirection {
    In,
    Out,
}

impl FromStr for PortDirection {
    type Err = (); // TODO: Actually do this

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() { // Probably don't need to have it be lowercase
            "in" => Ok(PortDirection::In),
            "out" => Ok(PortDirection::Out),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PortEvent {
    object_id: u32,
    node_id: u32,
    direction: PortDirection,
    // TODO: AFAIK there seems to be no way to determine this on the port.
    // Helvum does this by checking `media.class` on the Node.
    // Do Nodes only ever support a single port type? This seems unreasonable to me.
    // What of a node that down samples to 8 bit?
    // channel_type: PortType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct NodeEvent {
    object_id: u32,
    application_name: String,
    node_name: String,
    

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct WireEvent {
    object_id: u32,

    input_node_id: u32,
    output_node_id: u32,

    output_port_id: u32,
    input_port_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum GraphEvent {
    Wire(WireEvent),
    Port(PortEvent),
    Node(NodeEvent),
}

// TODO: This is unwieldy and unmanageable.
// I need a better way to do this.
pub(crate) fn get_event(pw_event: &GlobalObject<ForeignDict>) -> Result<GraphEvent, ()> {
    match &pw_event.type_ {
        pipewire::types::ObjectType::Node => Ok(GraphEvent::Node(parse_node_event(pw_event))),
        pipewire::types::ObjectType::Port => Ok(GraphEvent::Port(parse_port_event(pw_event))),
        pipewire::types::ObjectType::Link => Ok(GraphEvent::Wire(parse_wire_event(pw_event))),

        other => {
            trace!("Unhandled Object Type {:?}", &other);
            Err(())
        }
        // Everything past this is  unhandled and I'm just ignoring it for now.
        // pipewire::types::ObjectType::Endpoint => todo!(),
        // pipewire::types::ObjectType::EndpointLink => todo!(),
        // pipewire::types::ObjectType::EndpointStream => todo!(),

        // pipewire::types::ObjectType::ClientNode => todo!(),
        // pipewire::types::ObjectType::ClientEndpoint => todo!(),
        // pipewire::types::ObjectType::Client => todo!(),
        // pipewire::types::ObjectType::ClientSession => todo!(),

        // pipewire::types::ObjectType::Core => todo!(),
        // pipewire::types::ObjectType::Device => todo!(),
        // pipewire::types::ObjectType::Factory => todo!(),
        // pipewire::types::ObjectType::Metadata => todo!(),
        // pipewire::types::ObjectType::Module => todo!(),
        // pipewire::types::ObjectType::Profiler => todo!(),
        // pipewire::types::ObjectType::Registry => todo!(),
        // pipewire::types::ObjectType::Session => todo!(),
        // pipewire::types::ObjectType::Other(_) => todo!(),
    }
}

fn parse_port_event(pw_event: &GlobalObject<ForeignDict>) -> PortEvent {
    let properties = pw_event.props.as_ref().unwrap();
    PortEvent {
        object_id: pw_event.id,
        node_id: properties.get("node.id").unwrap().parse().unwrap(),
        direction: properties.get("port.direction").unwrap().parse().unwrap(),
    }
}

fn parse_wire_event(pw_event: &GlobalObject<ForeignDict>) -> WireEvent {
    let properties = pw_event.props.as_ref().unwrap();
    WireEvent {
        object_id: pw_event.id,
        input_node_id: properties.get("link.input.node").unwrap().parse().unwrap(),
        input_port_id: properties.get("link.input.port").unwrap().parse().unwrap(),
        output_node_id: properties.get("link.output.node").unwrap().parse().unwrap(),
        output_port_id: properties.get("link.output.port").unwrap().parse().unwrap(),
    }
}

fn parse_node_event(pw_event: &GlobalObject<ForeignDict>) -> NodeEvent {
    let properties = pw_event.props.as_ref().unwrap();
    NodeEvent {
        object_id: pw_event.id,
        application_name: properties.get("application.name").unwrap().into(),
        node_name: properties.get("node.name").unwrap().into(),
    }
}

#[cfg(test)]
mod test {

}
