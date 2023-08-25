use crate::{types::{
    custom_integration::CustomIntegration,
    device::{Device, DeviceData, SensorDevice, DeviceId, ManagedDevice, ManagedDeviceState},
    event::{Message, TxEventChannel},
    integration::{IntegrationActionPayload, IntegrationId}, color::Capabilities,
}, integrations::neato::api::debug_robot_states};
use crate::{
    db::actions::{db_get_neato_last_run, db_set_neato_last_run},
    utils::from_hh_mm,
};
use async_trait::async_trait;
use chrono::{Datelike, Weekday};
use color_eyre::Result;
use eyre::Context;
use serde::Deserialize;

mod api;

use api::clean_house;

use self::api::{Robot, RobotCmd, get_robots, update_robot_states};

#[derive(Clone, Debug, Deserialize)]
pub struct NeatoConfig {
    email: String,
    password: String,

    /// Only clean on these days
    cleaning_days: Vec<Weekday>,

    /// Earliest possible time when cleaning is allowed to start
    #[serde(deserialize_with = "from_hh_mm")]
    cleaning_time_start: chrono::NaiveTime,

    /// Latest possible time when cleaning is allowed to start
    #[serde(deserialize_with = "from_hh_mm")]
    cleaning_time_end: chrono::NaiveTime,

    /// If set to true, will request robot info instead of sending start cleaning command
    dummy: bool,
}

pub struct Neato {
    integration_id: IntegrationId,

    config: NeatoConfig,

    prev_run: Option<chrono::NaiveDateTime>,

    event_tx: TxEventChannel,
}

#[async_trait]
impl CustomIntegration for Neato {
    fn new(
        integration_id: &IntegrationId,
        config: &config::Value,
        event_tx: TxEventChannel,
    ) -> Result<Neato> {
        let config = config
            .clone()
            .try_deserialize()
            .wrap_err("Failed to deserialize config of Neato integration")?;
        Ok(Neato {
            integration_id: integration_id.clone(),
            config,
            prev_run: None,
            event_tx
        })
    }

    async fn register(&mut self) -> color_eyre::Result<()> {
        let prev_run = db_get_neato_last_run(&self.integration_id).await;

        if let Ok(prev_run) = prev_run {
            self.prev_run = Some(prev_run);
        }

        // let device = mk_neato_device(&self.id, &self.config, false);
        // let robots = get_robots(&self.config).await?;

        // for robot in robots {
        //     debug!("Found robot: {:?}", robot.name);
        // }
        // self.event_tx.send(Message::RecvDeviceState { device });

        Ok(())
    }

    async fn start(&mut self) -> color_eyre::Result<()> {
        let robots = update_robot_states(get_robots(&self.config).await?).await?;

        for robot in robots {
            let r = robot.clone();
            debug!("Found robot: {:?}", robot.name);
            debug!("Robot state: {:?}", robot.state);
            // debug_robot_states(robot).await?;
            let device = mk_neato_device(self, &r);
            debug!("Device: {:?}", device);
            self.event_tx.send(Message::RecvDeviceState { device })
        }
        Ok(())
    }

    async fn set_integration_device_state(&mut self, _: &Device) -> Result<()> {
        Ok(())
    }

    async fn run_integration_action(&mut self, payload: &IntegrationActionPayload) -> Result<()> {
        match payload.to_string().as_str() {
            "clean_house" | "clean_house_force" => {
                let force = payload.to_string() == "clean_house_force";
                let local = chrono::Local::now().naive_local();

                if !(self.config.cleaning_time_start..self.config.cleaning_time_end)
                    .contains(&local.time())
                {
                    info!("Skipping cleaning due to wrong time of day");
                    return Ok(());
                }

                if !force && !self.config.dummy {
                    let weekday = local.weekday();

                    if !self.config.cleaning_days.contains(&weekday) {
                        info!("Skipping cleaning due to wrong weekday");
                        return Ok(());
                    }

                    if !(self.config.cleaning_time_start..self.config.cleaning_time_end)
                        .contains(&local.time())
                    {
                        info!("Skipping cleaning due to wrong time of day");
                        return Ok(());
                    }

                    if let Some(prev_run) = self.prev_run {
                        if prev_run.num_days_from_ce() == local.num_days_from_ce() {
                            info!("Skipping cleaning due to previous run being today");
                            return Ok(());
                        }
                    }
                }

                self.prev_run = Some(local);
                let result = clean_house(&self.config, &RobotCmd::StartCleaning).await;

                db_set_neato_last_run(&self.integration_id, local)
                    .await
                    .ok();

                result
            }
            "stop_cleaning" => clean_house(&self.config, &RobotCmd::StopCleaning).await,
            _ => Ok(()),
        }
    }
}


fn mk_neato_device(config: &Neato, robot: &Robot) -> Device {
    let r_state = robot.state.as_ref().unwrap();
    let r_state_string = format!("{}_{}", r_state.state, r_state.action);
    let state = DeviceData::Sensor(SensorDevice::Text { value: r_state_string });

    Device {
        id: DeviceId::new(&robot.serial.clone()),
        name: robot.name.clone(),
        integration_id: config.integration_id.clone(),
        data: state,
    }
}