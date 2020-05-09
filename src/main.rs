#[macro_use]
extern crate diesel;

mod db;
mod homectl_core;
mod integrations;

use db::{actions::find_floorplans, establish_connection};
use homectl_core::integrations_manager::IntegrationsManager;

fn main() {
    let config = homectl_core::config::read_config();

    println!("Using config:");
    println!("{:#?}", config);

    let mut integrations_manager = IntegrationsManager::new();

    for (id, module_name) in &config.integrations {
        integrations_manager.load(module_name, id).unwrap();
    }

    integrations_manager.register();

  println!("Hello, world!");
  println!("{:#?}", settings);
}
