use super::integration::Integration;
use crate::integrations::dummy::Dummy;
use std::collections::HashMap;

fn load_integration(
    module_name: &String,
    id: &String,
    config: String,
) -> Result<Box<dyn Integration>, String> {
    match module_name.as_str() {
        "dummy" => Ok(Box::new(Dummy::new(id.clone()))),
        _ => Err(format!("Unknown module name {}!", module_name)),
    }
}

pub struct IntegrationsManager {
    integrations: HashMap<String, Box<dyn Integration>>,
}

impl IntegrationsManager {
    pub fn new() -> Self {
        IntegrationsManager {
            integrations,
            devices_manager,
        }
    }

    pub fn load_integration(
        &self,
        module_name: &String,
        integration_id: &IntegrationId,
        config: &config::Value,
        shared_integrations_manager: SharedIntegrationsManager,
    ) -> Result<(), String> {
        println!("loading integration with module_name {}", module_name);

        let integration = load_integration(
            module_name,
            integration_id,
            config,
            shared_integrations_manager,
        )?;

        let devices = HashMap::new();
        let managed = ManagedIntegration {
            integration,
            devices,
        };

        {
            let mut integrations = self.integrations.lock().unwrap();
            integrations.insert(integration_id.clone(), managed);
        }
    }

    pub fn load(&mut self, module_name: &String, id: &String) -> Result<(), String> {
        let integration = load_integration(module_name, id, "".into())?;
        self.integrations.insert(id.clone(), integration);

        Ok(())
    }

    pub fn register(&self) {
        // for (_id, integration) in self.integrations {
        //     integration.register();
        // }
    }

// integrations will perhaps one day be loaded dynamically:
// https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
fn load_integration(
    module_name: &String,
    id: &IntegrationId,
    config: &config::Value,
    integrations_manager: SharedIntegrationsManager,
) -> Result<ThreadsafeIntegration, String> {
    match module_name.as_str() {
        "dummy" => Ok(Box::new(Dummy::new(id, config, integrations_manager))),
        _ => Err(format!("Unknown module name {}!", module_name)),
    }
}
