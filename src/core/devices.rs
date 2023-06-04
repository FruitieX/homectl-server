use crate::db::actions::{db_find_device, db_update_device};
use crate::types::color::DeviceColor;

use super::scenes::Scenes;
use crate::types::device::{DeviceId, ManagedDeviceState};
use crate::types::group::GroupId;
use crate::types::{
    device::{Device, DeviceData, DeviceKey, DevicesState},
    event::{Message, TxEventChannel},
    integration::IntegrationId,
    scene::{SceneDescriptor, SceneDevicesConfig, SceneId},
};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Devices {
    sender: TxEventChannel,
    state: Arc<Mutex<DevicesState>>,
    scenes: Scenes,
}

fn cmp_light_color(
    a: &Option<DeviceColor>,
    a_bri: &Option<f32>,
    b: &Option<DeviceColor>,
    b_bri: &Option<f32>,
) -> bool {
    let hue_delta = 1;
    let sat_delta = 0.05;
    let bri_delta = 0.05;
    let cct_delta = 10;

    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(DeviceColor::Hs(a)), Some(DeviceColor::Hs(b))) => {
            // Light state is equal if all components differ by less than a given delta
            (u16::abs_diff(a.h, b.h) <= hue_delta)
                && (f32::abs(a.s - b.s) <= sat_delta)
                && (f32::abs(a_bri.unwrap_or(1.0) - b_bri.unwrap_or(1.0)) <= bri_delta)
        }
        (Some(DeviceColor::Ct(a)), Some(DeviceColor::Ct(b))) => {
            (u16::abs_diff(a.ct, b.ct) <= cct_delta)
                && (f32::abs(a_bri.unwrap_or(1.0) - b_bri.unwrap_or(1.0)) <= bri_delta)
        }
        (_, _) => false,
    }
}

fn cmp_device_states(device: &ManagedDeviceState, expected: &ManagedDeviceState) -> bool {
    if device.power != expected.power {
        return false;
    }

    // If both lights are turned off, state matches
    if !device.power && !expected.power {
        return true;
    }

    // Compare colors if supported
    if device.color.is_some() {
        return cmp_light_color(
            &device.color,
            &device.brightness,
            &expected.color,
            &expected.brightness,
        );
    }

    true
}

impl Devices {
    pub fn new(sender: TxEventChannel, scenes: Scenes) -> Self {
        Devices {
            sender,
            state: Default::default(),
            scenes,
        }
    }

    pub fn get_devices(&self) -> DevicesState {
        self.state.lock().unwrap().clone()
    }

    /// Checks whether device values were changed or not due to refresh
    pub async fn handle_integration_device_refresh(&mut self, incoming: &Device) {
        // println!("handle_integration_device_refresh {:?}", device);
        let current = self.get_device(&incoming.get_device_key());

        // recompute expected_state here as it may have changed since we last
        // computed it
        let expected_state = current
            .as_ref()
            .and_then(|d| self.get_expected_state(d, false));

        // Take action if the device state has changed from stored state
        if Some(incoming) != current.as_ref()
            || expected_state.as_ref() != incoming.get_managed_state()
        {
            let data = incoming.data.clone();

            match (&data, current, expected_state) {
                // Device was seen for the first time
                (_, None, _) => {
                    let db_device = db_find_device(&incoming.get_device_key()).await.ok();

                    match db_device {
                        Some(db_device) => {
                            // Note that we only restore a device from DB once it has been
                            // discovered by the integration. This way we don't end up with a lot
                            // of possibly old/stale devices.

                            // Restore device scene from DB
                            let scene = db_device.get_scene();

                            let device = incoming.set_scene(scene);

                            println!("Discovered previously seen device, restored scene from DB: {:?}", device);

                            self.set_device_state(&device, true, true, false).await;
                        }
                        None => {
                            println!("Discovered device: {:?}", incoming);
                            self.set_device_state(incoming, true, false, false).await;
                            db_update_device(incoming).await.ok();
                        }
                    }
                }

                // Sensor state has changed, defer handling of this update
                // to other subsystems
                (DeviceData::Sensor(_), Some(_), _) => {
                    self.set_device_state(incoming, false, false, true).await;
                }

                // Device state does not match expected state, maybe the
                // device missed a state update or forgot its state? Try
                // fixing this by emitting a SetIntegrationDeviceState
                // message back to integration
                (DeviceData::Managed(ref incoming_managed), _, Some(expected_state)) => {
                    if cmp_device_states(&incoming_managed.state, &expected_state) {
                        return;
                    }

                    println!(
                        "Device state mismatch detected ({}/{}): (was: {:?}, expected: {:?})",
                        incoming.integration_id, incoming.name, incoming.data, expected_state
                    );

                    let mut device = incoming.clone();
                    device.data = data;

                    self.sender.send(Message::SetIntegrationDeviceState {
                        device,
                        state_changed: true,
                    });
                }

                // Expected device state was not found
                (_, _, None) => {
                    self.set_device_state(incoming, false, false, true).await;
                }
            }
        }
    }

    /// Returns expected state for given device based on possible active scene.
    /// If no scene active and use_passed_state is false, previous device state is returned.
    /// If no scene active and use_passed_state is true, passed device state is returned.
    fn get_expected_state(
        &self,
        device: &Device,
        use_passed_state: bool,
    ) -> Option<ManagedDeviceState> {
        match device.data {
            DeviceData::Sensor(_) => None,

            DeviceData::Managed(_) => {
                let scene_device_state = {
                    let state = self.state.lock().unwrap();

                    // Ignore transition specified by scene if we're setting state
                    let ignore_transition = use_passed_state;
                    self.scenes
                        .find_scene_device_state(device, &state, ignore_transition)
                };

                match scene_device_state {
                    // Return state from active scene
                    Some(state) => Some(state),
                    None => {
                        if use_passed_state {
                            // Return passed device state
                            device.get_managed_state().cloned()
                        } else {
                            let state = self.state.lock().unwrap();

                            // Return previous device state
                            let device = state
                                .0
                                .get(&device.get_device_key())
                                .unwrap_or(device)
                                .clone();

                            device.get_managed_state().cloned()
                        }
                    }
                }
            }
        }
    }

    /// Sets stored state for given device and dispatches DeviceUpdate
    pub async fn set_device_state(
        &mut self,
        device: &Device,
        set_scene: bool,
        skip_db: bool,
        skip_integration: bool,
    ) -> Device {
        let old: Option<Device> = self.get_device(&device.get_device_key());
        let old_states = { self.state.lock().unwrap().clone() };

        let mut device = device.clone();

        // Restore scene if set_scene is false
        if let (false, Some(old)) = (set_scene, &old) {
            let old_device_scene = old.get_scene();
            device = device.set_scene(old_device_scene);
        }

        // Allow active scene to override device state
        let expected_state = self.get_expected_state(&device, true);

        if let Some(expected_state) = expected_state {
            device = device.set_managed_state(expected_state);
        }

        let mut states = self.state.lock().unwrap();

        states.0.insert(device.get_device_key(), device.clone());

        let state_changed = old.as_ref() != Some(&device);

        self.sender.send(Message::DeviceUpdate {
            old_state: old_states,
            new_state: states.clone(),
            old,
            new: device.clone(),
        });

        if state_changed && !skip_db {
            // FIXME: compiler error without task::spawn()
            let device = device.clone();
            tokio::spawn(async move {
                db_update_device(&device).await.ok();
            });
        }

        if !skip_integration {
            self.sender.send(Message::SetIntegrationDeviceState {
                device: device.clone(),
                state_changed,
            });
        }

        device
    }

    pub fn get_device(&self, device_key: &DeviceKey) -> Option<Device> {
        self.state.lock().unwrap().0.get(device_key).cloned()
    }

    fn find_scene_devices_config(&self, sd: &SceneDescriptor) -> Option<SceneDevicesConfig> {
        self.scenes
            .find_scene_devices_config(&self.state.lock().unwrap(), sd)
    }

    pub async fn activate_scene(
        &mut self,
        scene_id: &SceneId,
        device_keys: &Option<Vec<DeviceKey>>,
        group_keys: &Option<Vec<GroupId>>,
    ) -> Option<bool> {
        println!("Activating scene {:?}", scene_id);

        let scene_devices_config = self.find_scene_devices_config(&SceneDescriptor {
            scene_id: scene_id.clone(),
            device_keys: device_keys.clone(),
            group_keys: group_keys.clone(),
        })?;

        for (integration_id, devices) in scene_devices_config {
            for (device_id, _) in devices {
                let device_key = &DeviceKey::new(integration_id.clone(), device_id);

                let device = self.get_device(device_key);

                if let Some(device) = device {
                    let device = device.set_scene(Some(scene_id.clone()));
                    self.set_device_state(&device, true, false, false).await;
                }
            }
        }

        Some(true)
    }

    pub async fn cycle_scenes(
        &mut self,
        scene_descriptors: &[SceneDescriptor],
        nowrap: bool,
    ) -> Option<bool> {
        let mut scenes_common_devices: Vec<(IntegrationId, DeviceId)> = Vec::new();

        let scene_devices_configs: Vec<(&SceneDescriptor, Option<SceneDevicesConfig>)> =
            scene_descriptors
                .iter()
                .map(|sd| (sd, self.find_scene_devices_config(sd)))
                .collect();

        // gather a Vec<Vec(IntegrationId, DeviceId)>> of all devices in cycled scenes
        let scenes_devices: Vec<Vec<(IntegrationId, DeviceId)>> = scene_devices_configs
            .iter()
            .map(|(_, scene_devices_config)| {
                let mut scene_devices: Vec<(IntegrationId, DeviceId)> = Vec::new();
                if let Some(integrations) = scene_devices_config {
                    for (integration_id, integration) in integrations {
                        for device_id in integration.keys() {
                            scene_devices.push((integration_id.clone(), device_id.clone()));
                        }
                    }
                }

                scene_devices
            })
            .collect();

        // gather devices which exist in all cycled scenes into scenes_common_devices
        if let Some(first_scene_devices) = scenes_devices.first() {
            for scene_device in first_scene_devices {
                if scenes_devices
                    .iter()
                    .all(|scene_devices| scene_devices.contains(scene_device))
                {
                    scenes_common_devices.push(scene_device.clone());
                }
            }
        }

        let state = self.state.lock().unwrap().clone();

        let active_scene_index =
            scene_devices_configs
                .iter()
                .position(|(sd, scene_devices_config)| {
                    // try finding any device in scene_devices_config that has this scene active
                    if let Some(integrations) = scene_devices_config {
                        integrations.iter().any(|(integration_id, devices)| {
                            devices.iter().any(|(device_id, _)| {
                                // only consider devices which are common across all cycled scenes
                                if !scenes_common_devices
                                    .contains(&(integration_id.clone(), device_id.clone()))
                                {
                                    return false;
                                }

                                let device =
                                    find_device(&state, integration_id, Some(device_id), None);
                                let device_scene = device.and_then(|d| d.get_scene());

                                device_scene.map_or(false, |ds| ds == sd.scene_id)
                            })
                        })
                    } else {
                        false
                    }
                });

        let next_scene = match active_scene_index {
            Some(index) => {
                let next_scene_index = if nowrap {
                    (index + 1).min(scene_descriptors.len() - 1)
                } else {
                    (index + 1) % scene_descriptors.len()
                };
                scene_descriptors.get(next_scene_index)
            }
            None => scene_descriptors.first(),
        }?;

        self.activate_scene(
            &next_scene.scene_id,
            &next_scene.device_keys,
            &next_scene.group_keys,
        )
        .await;

        Some(true)
    }
}

pub fn find_device(
    devices: &DevicesState,
    integration_id: &IntegrationId,
    device_id: Option<&DeviceId>,
    name: Option<&String>,
) -> Option<Device> {
    let device = devices
        .0
        .iter()
        .find(
            |(
                DeviceKey {
                    integration_id: candidate_integration_id,
                    device_id: candidate_device_id,
                },
                candidate_device,
            )| {
                if integration_id != candidate_integration_id {
                    return false;
                }
                if device_id.is_some() && device_id != Some(candidate_device_id) {
                    return false;
                }

                // TODO: regex matches
                if name.is_some() && name != Some(&candidate_device.name) {
                    return false;
                }

                true
            },
        )
        .map(|(_, device)| device)?;

    Some(device.clone())
}
