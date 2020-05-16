use crate::homectl_core::{
    device::{Device, DeviceKind, OnOffDevice},
    integration::Integration,
};
use async_trait::async_trait;
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize)]
pub struct DummyConfig {
    asd: String,
}

pub struct Dummy {
    id: String,
    devices: Vec<Device>,
}

impl Integration for Dummy {
    fn new(
        id: &IntegrationId,
        config: &config::Value,
        shared_integrations_manager: SharedIntegrationsManager,
    ) -> Self {
        println!(
            "asdasd: {:?}",
            config.clone().try_into::<DummyConfig>().unwrap()
        );

        // test that we can call methods on integrations_manager
        // {
        //     let mut integrations_manager = shared_integrations_manager.lock().unwrap();
        //     integrations_manager.load_integration(
        //         &String::from("asd"),
        //         &String::from("asd"),
        //         shared_integrations_manager.clone(),
        //     );
        // }
        Dummy {
            id,
            devices: Vec::new(),
        }
    }

    fn register(&self) {
        println!("registered dummy integration");
    }

    fn get_devices(&self) -> Vec<Device> {
        self.devices.clone()
    }
}
