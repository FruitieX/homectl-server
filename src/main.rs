#[macro_use]
extern crate diesel;

mod db;
mod homectl_core;
mod integrations;

use db::{actions::find_floorplans, establish_connection};
use homectl_core::integrations_manager::IntegrationsManager;
use integrations::dummy::DummyConfig;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

// https://github.com/actix/examples/blob/master/diesel/src/main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (config, opaque_integrations_configs) = homectl_core::config::read_config();

    println!("Using config:");
    println!("{:#?}", config);

    let mut integrations_manager = IntegrationsManager::new();

    for (id, integration_config) in &config.integrations {
        let integrations_manager = shared_integrations_manager.lock().unwrap();

        let opaque_integration_config: &config::Value =
            opaque_integrations_configs.get(id).unwrap();

        integrations_manager
            .load_integration(
                &integration_config.plugin,
                id,
                opaque_integration_config,
                shared_integrations_manager.clone(),
            )
            .unwrap();
    }

    integrations_manager.register();

  println!("Hello, world!");
  println!("{:#?}", settings);
}
