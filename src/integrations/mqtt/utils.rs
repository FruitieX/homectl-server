use crate::integrations::mqtt::MqttConfig;
use crate::types::color::{Capabilities, DeviceColor};
use crate::types::{
    device::{ControllableDevice, Device, DeviceData, DeviceId, SensorDevice},
    integration::IntegrationId,
};
use color_eyre::Result;
use jsonptr::{Assign, Pointer};
use ordered_float::OrderedFloat;

pub fn mqtt_to_homectl(
    payload: &[u8],
    topic: &str,
    integration_id: IntegrationId,
    config: &MqttConfig,
) -> Option<Device> {
    let value: Result<serde_json::Value, serde_json::Error> = serde_json::from_slice(payload);

    let value = match value {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to parse MQTT message: {topic} {err}");
            return None;
        }
    };

    let id_field = config
        .id_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/id"));
    let name_field = config
        .name_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/name"));
    let color_field = config
        .color_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/color"));
    let power_field = config
        .power_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/power"));
    let brightness_field = config
        .brightness_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/brightness"));
    let sensor_value_fields = config
        .sensor_value_fields
        .as_ref()
        .map(|v| v.iter().map(|p| p.as_ref()).collect())
        .unwrap_or(vec![Pointer::from_static("/sensor_value")]);
    let transition_field = config
        .transition_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/transition"));
    let capabilities_field = config
        .capabilities_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/capabilities"));

    let id = id_field
        .resolve(&value)
        .ok()
        .and_then(serde_json::Value::as_str)
        .map(|id| id.to_string());

    let Some(id) = id else {
        error!("Missing '{id_field}' field in MQTT message");
        return None;
    };

    let name = name_field
        .resolve(&value)
        .ok()
        .and_then(serde_json::Value::as_str)
        .map(|name| name.to_string());

    let Some(name) = name else {
        error!("Missing '{name_field}' field in MQTT message");
        return None;
    };

    let color = color_field
        .resolve(&value)
        .ok()
        .and_then(|value| serde_json::from_value::<DeviceColor>(value.clone()).ok());

    let power = power_field.resolve(&value).ok().and_then(|value| {
        if config
            .power_on_value
            .as_ref()
            .unwrap_or(&serde_json::Value::Bool(true))
            == value
        {
            Some(true)
        } else if config
            .power_off_value
            .as_ref()
            .unwrap_or(&serde_json::Value::Bool(false))
            == value
        {
            Some(false)
        } else {
            None
        }
    });

    let brightness = {
        let range = config.brightness_range.unwrap_or((0.0, 1.0));

        brightness_field
            .resolve(&value)
            .ok()
            .and_then(serde_json::Value::as_f64)
            .map(|value| value as f32)
            // scale value from [range.0, range.1] to [0, 1]
            .map(|value| (value - range.0) / (range.1 - range.0))
    };

    let transition = {
        let range = config.transition_range.unwrap_or((0.0, 1.0));

        transition_field
            .resolve(&value)
            .ok()
            .and_then(serde_json::Value::as_f64)
            .map(|value| value as f32)
            // scale value from [range.0, range.1] to [0, 1]
            .map(|value| (value - range.0) / (range.1 - range.0))
    };

    let resolved_sensor_value_field = sensor_value_fields
        .iter()
        .find_map(|field| Some((field, field.resolve(&value).ok()?)))
        .filter(|(_, v)| !v.is_null());
    let device_state = if let Some((field, value)) = resolved_sensor_value_field {
        DeviceData::Sensor(match value {
            serde_json::Value::Number(value) => SensorDevice::Number {
                value: value.as_f64().unwrap(),
            },

            serde_json::Value::Bool(value) => SensorDevice::Boolean { value: *value },

            // TODO: get rid of this hack and use proper booleans
            serde_json::Value::String(value) if value == "true" => {
                SensorDevice::Boolean { value: true }
            }
            serde_json::Value::String(value) if value == "false" => {
                SensorDevice::Boolean { value: false }
            }

            serde_json::Value::String(value) => SensorDevice::Text {
                value: value.clone(),
            },
            _ => {
                error!("Unsupported value for sensor field '{field}'");
                return None;
            }
        })
    } else if power.is_none() && brightness.is_none() && color.is_none() {
        warn!("Unable to determine device type for {topic}, discarding MQTT message");
        return None;
    } else {
        let capabilities: Capabilities = capabilities_field
            .resolve(&value)
            .ok()
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .or_else(|| config.capabilities_override.clone())
            .unwrap_or_default();

        let controllable_device = ControllableDevice::new(
            None,
            power.unwrap_or_default(),
            brightness,
            color,
            transition,
            capabilities,
            config.managed.clone().unwrap_or_default(),
        );

        DeviceData::Controllable(controllable_device)
    };

    let raw = config
        .raw_field
        .as_deref()
        .unwrap_or(Pointer::from_static("/raw"))
        .resolve(&value)
        .ok()
        .cloned();

    Some(Device {
        id: DeviceId::new(&id),
        name,
        integration_id,
        data: device_state,
        raw,
    })
}

pub fn homectl_to_mqtt(device: Device, config: &MqttConfig) -> Result<serde_json::Value> {
    let mut payload = serde_json::Value::default();

    let id_field = config
        .id_field
        .clone()
        .unwrap_or_else(|| jsonptr::PointerBuf::from_tokens(["id"]));
    let name_field = config
        .name_field
        .clone()
        .unwrap_or_else(|| jsonptr::PointerBuf::from_tokens(["name"]));
    let color_field = config
        .color_field
        .clone()
        .unwrap_or_else(|| jsonptr::PointerBuf::from_tokens(["color"]));
    let power_field = config
        .power_field
        .clone()
        .unwrap_or_else(|| jsonptr::PointerBuf::from_tokens(["power"]));
    let brightness_field = config
        .brightness_field
        .clone()
        .unwrap_or_else(|| jsonptr::PointerBuf::from_tokens(["brightness"]));
    let transition_field = config
        .transition_field
        .clone()
        .unwrap_or_else(|| jsonptr::PointerBuf::from_tokens(["transition"]));

    if config.include_id_name_in_set_payload.unwrap_or_default() {
        payload.assign(&id_field, serde_json::Value::String(device.id.to_string()))?;
        payload.assign(&name_field, serde_json::Value::String(device.name))?;
    }

    if let DeviceData::Controllable(device) = device.data {
        let power_value = if device.state.power {
            config
                .power_on_value
                .clone()
                .unwrap_or(serde_json::Value::Bool(true))
        } else {
            config
                .power_off_value
                .clone()
                .unwrap_or(serde_json::Value::Bool(false))
        };
        payload.assign(&power_field, power_value)?;

        if let Some(brightness) = device.state.brightness {
            let range = config.brightness_range.unwrap_or((0.0, 1.0));
            // scale value from [0, 1] to [range.0, range.1]
            let value = brightness * (range.1 - range.0) + range.0;
            payload.assign(
                &brightness_field,
                serde_json::Number::from_f64((*value).into())
                    .map(serde_json::Value::Number)
                    .unwrap(),
            )?;
        }

        if let Some(color) = &device.state.color {
            payload.assign(&color_field, serde_json::to_value(color)?)?;
        }

        let transition = device
            .state
            .transition
            .or(config.default_transition.map(OrderedFloat));
        if let Some(transition) = transition {
            let range = config.transition_range.unwrap_or((0.0, 1.0));
            // scale value from [0, 1] to [range.0, range.1]
            let value = transition * (range.1 - range.0) + range.0;
            payload.assign(
                &transition_field,
                serde_json::Number::from_f64((*value).into())
                    .map(serde_json::Value::Number)
                    .unwrap(),
            )?;
        }
    };

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use crate::types::{
        color::{Capabilities, ColorMode, Hs},
        device::ManageKind,
    };

    use super::*;
    use ordered_float::OrderedFloat;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn test_homectl_to_mqtt() {
        // Create a device and MqttConfig
        let device = Device {
            id: DeviceId::new("device1"),
            name: "Device 1".to_string(),
            integration_id: IntegrationId::from_str("mqtt").unwrap(),
            data: DeviceData::Controllable(ControllableDevice::new(
                None,
                true,
                Some(0.5),
                Some(DeviceColor::Hs(Hs {
                    h: 45,
                    s: OrderedFloat(1.0),
                })),
                None,
                Capabilities::default(),
                ManageKind::Full,
            )),
            raw: None,
        };

        let config = MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            topic: "homectl/devices/{id}".to_string(),
            topic_set: "homectl/set/{id}".to_string(),
            include_id_name_in_set_payload: Some(true),
            ..Default::default()
        };

        let mqtt_json = homectl_to_mqtt(device, &config).unwrap();

        let expected = json!({
            "id": "device1",
            "name": "Device 1",
            "color": { "h": 45, "s": 1.0 },
            "power": true,
            "brightness": 0.5,
        });

        assert_eq!(mqtt_json, expected);
    }

    #[test]
    fn test_mqtt_to_homectl() {
        let mqtt_json = json!({
            "id": "device1",
            "name": "Device 1",
            "color": { "h": 45, "s": 1.0 },
            "power": true,
            "brightness": 0.5,
            "transition": 0.6,
            "capabilities": { "ct": serde_json::Value::Null, "hs": true, "rgb": false, "xy": false }
        });

        let config = MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            topic: "homectl/devices/{id}".to_string(),
            topic_set: "homectl/set/{id}".to_string(),
            managed: Some(ManageKind::Unmanaged),
            ..Default::default()
        };

        let integration_id = IntegrationId::from_str("mqtt").unwrap();
        let device = mqtt_to_homectl(
            mqtt_json.to_string().as_bytes(),
            "homectl/devices/device1",
            integration_id.clone(),
            &config,
        )
        .unwrap();

        let expected = Device {
            id: DeviceId::new("device1"),
            name: "Device 1".to_string(),
            integration_id,
            data: DeviceData::Controllable(ControllableDevice::new(
                None,
                true,
                Some(0.5),
                Some(DeviceColor::Hs(Hs {
                    h: 45,
                    s: OrderedFloat(1.0),
                })),
                Some(0.6),
                Capabilities::singleton(ColorMode::Hs),
                ManageKind::Unmanaged,
            )),
            raw: None,
        };

        assert_eq!(device, expected);
    }

    #[tokio::test]
    async fn test_integration() {
        let mqtt_json = json!({
            "id": "device1",
            "name": "Device 1",
            "color": { "h": 45, "s": 1.0 },
            "power": true,
            "brightness": 0.5,
        });

        let config = MqttConfig {
            host: "localhost".to_string(),
            port: 1883,
            topic: "homectl/devices/{id}".to_string(),
            topic_set: "homectl/set/{id}".to_string(),
            managed: Some(ManageKind::Unmanaged),
            include_id_name_in_set_payload: Some(true),
            ..Default::default()
        };

        let integration_id = IntegrationId::from_str("mqtt").unwrap();
        let device = mqtt_to_homectl(
            mqtt_json.to_string().as_bytes(),
            "homectl/devices/device1",
            integration_id,
            &config,
        )
        .unwrap();
        let mqtt_message_value = homectl_to_mqtt(device, &config).unwrap();

        assert_eq!(mqtt_json, mqtt_message_value);
    }
}
