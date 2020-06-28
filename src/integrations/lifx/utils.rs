use crate::homectl_core::device::{Device, DeviceState, Light};
use byteorder::{ByteOrder, LittleEndian};
use palette::Hsv;
use std::{error::Error, net::SocketAddr};

#[derive(Clone)]
pub struct LifxState {
    pub hue: u16,
    pub sat: u16,
    pub bri: u16,
    pub power: u16,
    pub label: String,
    pub addr: SocketAddr,
    pub transition: Option<u16>,
}

#[derive(Clone)]
pub enum LifxMsg {
    Get(SocketAddr),
    SetColor(LifxState),
    State(LifxState),
    SetPower(LifxState),
    Unknown,
}

pub fn lifx_msg_type_to_u16(msg_type: LifxMsg) -> u16 {
    match msg_type {
        LifxMsg::Get(_) => 101,
        LifxMsg::SetColor(_) => 102,
        LifxMsg::State(_) => 107,
        LifxMsg::SetPower(_) => 117,
        LifxMsg::Unknown => panic!("Cannot convert LifxMsg::Unknown to u16"),
    }
}

fn mk_lifx_msg_payload(lifx_msg: LifxMsg) -> Option<Vec<u8>> {
    match lifx_msg {
        LifxMsg::SetColor(state) => {
            let mut buf: [u8; 16 + 32] = [0; 16 + 32];

            LittleEndian::write_u16(&mut buf, state.power);

            state
                .transition
                .map(|t| LittleEndian::write_u16(&mut buf[2..], t));

            Some(buf.to_vec())
        }
        LifxMsg::SetPower(state) => {
            let mut buf: [u8; 8 + 16 * 4 + 32] = [0; 8 + 16 * 4 + 32];

            LittleEndian::write_u16(&mut buf[1..], state.hue);
            LittleEndian::write_u16(&mut buf[3..], state.sat);
            LittleEndian::write_u16(&mut buf[5..], state.bri);
            LittleEndian::write_u16(&mut buf[7..], 6500); // lifx requires this weird color temperature parameter?

            state
                .transition
                .map(|t| LittleEndian::write_u16(&mut buf[9..], t));

            Some(buf.to_vec())
        }
        _ => None,
    }
}

pub fn mk_lifx_udp_msg(lifx_msg: LifxMsg) -> Vec<u8> {
    // frame
    // https://lan.developer.lifx.com/docs/header-description#frame
    let mut frame: [u8; 8] = [0; 8];
    let protocol = 1024;
    let origin = 0;
    let tagged = 1;
    let addressable = 1;

    LittleEndian::write_u16(&mut frame, 0); // size to be filled in later
    LittleEndian::write_u16(
        &mut frame[2..],
        protocol | (origin << 14) | (tagged << 13) | (addressable << 12),
    );
    LittleEndian::write_u16(&mut frame[1..], 4);

    // frame address
    // https://lan.developer.lifx.com/docs/header-description#frame-address
    let mut frame_address: [u8; 16] = [0; 16];
    let ack_required = 0;
    let res_required = match lifx_msg {
        LifxMsg::Get(_) => 1,
        _ => 0,
    };

    frame_address[14] = (ack_required << 1) | res_required;

    // protocol header
    // https://lan.developer.lifx.com/docs/header-description#protocol-header
    let mut protocol_header: [u8; 12] = [0; 12];
    LittleEndian::write_u16(
        &mut protocol_header[8..],
        lifx_msg_type_to_u16(lifx_msg.clone()),
    );

    let payload = mk_lifx_msg_payload(lifx_msg.clone());
    let payload_size = payload.clone().map(|p| p.len()).unwrap_or(0);
    let msg_size = frame.len() + frame_address.len() + protocol_header.len() + payload_size;

    // we now know the total size - write it into the beginning of the frame header
    LittleEndian::write_u16(&mut frame, msg_size as u16);

    let mut msg: Vec<u8> = vec![];
    msg.append(&mut frame.to_vec());
    msg.append(&mut frame_address.to_vec());
    msg.append(&mut protocol_header.to_vec());

    match payload {
        Some(payload) => {
            msg.append(&mut payload.to_vec());
        }
        None => {}
    };

    msg
}

pub fn read_lifx_msg(buf: &[u8], addr: SocketAddr) -> LifxMsg {
    let msg_type = LittleEndian::read_u16(&buf[32..]);
    let payload = &buf[36..];

    match msg_type {
        107 => {
            // State (107) message, response to Get (101)
            // https://lan.developer.lifx.com/docs/light-messages#section-state-107

            let hue = LittleEndian::read_u16(&payload);
            let sat = LittleEndian::read_u16(&payload[2..]);
            let bri = LittleEndian::read_u16(&payload[4..]);

            let power = LittleEndian::read_u16(&payload[10..]);

            let label = std::str::from_utf8(&payload[12..(12 + 32)])
                .unwrap_or("Unknown")
                .to_owned()
                .replace("\0", "");

            let state = LifxState {
                hue,
                sat,
                bri,
                power,
                label,
                addr,
                transition: None,
            };

            LifxMsg::State(state)
        }
        _ => LifxMsg::Unknown,
    }
}

pub fn from_lifx_state(lifx_state: LifxState, integration_id: String) -> Device {
    let hue = (f32::from(lifx_state.hue) / 65535.0) * 360.0;
    let sat = f32::from(lifx_state.sat) / 65535.0;
    let bri = f32::from(lifx_state.bri) / 65535.0;

    let power = lifx_state.power == 65535;

    let color = Hsv::new(hue, sat, bri);

    let state = DeviceState::Light(Light {
        power,
        brightness: None,
        color: Some(color.into()),
    });

    let device = Device {
        id: lifx_state.addr.ip().to_string(),
        name: lifx_state.label.clone(),
        integration_id: integration_id.clone(),
        scene: None,
        state,
    };

    device
}

pub fn to_lifx_state(device: Device) -> Result<LifxState, Box<dyn Error>> {
    let light_state = match device.state {
        DeviceState::Light(Light {
            brightness,
            color,
            power,
        }) => Ok(Light {
            power,
            brightness,
            color,
        }),
        _ => Err("Unsupported device state"),
    }?;

    let color = light_state
        .color
        .map(|color| color.into())
        .unwrap_or(Hsv::new(0.0, 255.0, 255.0));

    let hue = ((color.hue.to_positive_degrees() / 360.0) * 65535.0).floor() as u16;
    let sat = (color.saturation * 65535.0).floor() as u16;
    let bri = (color.value * 65535.0).floor() as u16;

    let power = if light_state.power { 65535 } else { 0 };

    Ok(LifxState {
        hue,
        sat,
        bri,
        power,
        label: device.name,
        addr: device.id.parse()?,
        transition: Some(500),
    })
}