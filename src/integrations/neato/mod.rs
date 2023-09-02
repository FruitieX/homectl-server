use crate::types::{
    custom_integration::CustomIntegration,
    device::{Device, DeviceData, SensorDevice, DeviceId},
    event::{Message, TxEventChannel},
    integration::{IntegrationActionPayload, IntegrationId},
};
use crate::{
    db::actions::{db_get_neato_last_run, db_set_neato_last_run},
    utils::from_hh_mm,
};
use async_trait::async_trait;
use chrono::{Datelike, Weekday};
use color_eyre::Result;
use eyre::Context;
use serde::Deserialize;
use std::time::Duration;
use tokio::time;

mod api;

use self::api::{Robot, RobotCmd, get_robots, update_robot_states, RobotState, run_actions};

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

#[derive(Clone)]
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
            event_tx,
        })
    }

    async fn register(&mut self) -> color_eyre::Result<()> {
        let prev_run = db_get_neato_last_run(&self.integration_id).await;

        if let Ok(prev_run) = prev_run {
            self.prev_run = Some(prev_run);
        }

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
        
        // let neato = self.clone();
        // tokio::spawn(async { poll_robots_until_all_idle(neato).await });
        Ok(())
    }

    async fn set_integration_device_state(&mut self, _: &Device) -> Result<()> {
        Ok(())
    }

    async fn run_integration_action(&mut self, payload: &IntegrationActionPayload) -> Result<()> {
        let result = match payload.to_string().as_str() {
            "clean_house" | "clean_house_force" => {
                let force = payload.to_string() == "clean_house_force";
                let local = chrono::Local::now().naive_local();

                if self.config.dummy {
                    let robots = update_robot_states(get_robots(&self.config).await?).await?;
                    debug!("Found robots: {:?}", robots);
                    return Ok(());
                }

                if !force && !(self.config.cleaning_time_start..self.config.cleaning_time_end)
                    .contains(&local.time())
                {
                    info!("Skipping cleaning due to wrong time of day");
                    return Ok(());
                }

                if !force {
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
                let result = run_actions(&self.config, &RobotCmd::StartCleaning).await;

                db_set_neato_last_run(&self.integration_id, local)
                    .await
                    .ok();

                let event_tx = self.event_tx.clone();

                let robots = update_robot_states(get_robots(&self.config).await?).await?;
        
                for robot in robots {
                    let r = robot.clone();
                    let device = mk_neato_device(self, &r);
                    debug!("Neato: {:?}", device);
                    event_tx.send(Message::SetExpectedState {
                        device,
                        set_scene: false,
                    });
                }

                result
            }
            "stop_cleaning" => run_actions(&self.config, &RobotCmd::StopCleaning).await,
            "pause_cleaning" => run_actions(&self.config, &RobotCmd::PauseCleaning).await,
            "resume_cleaning" => run_actions(&self.config, &RobotCmd::ResumeCleaning).await,
            "send_to_base" | "go_to_base" => run_actions(&self.config, &RobotCmd::SendToBase).await,
            "debug" => run_actions(&self.config, &RobotCmd::GetRobotState).await,
            _ => Ok(()),
        };
        
        if !self.config.dummy && payload.to_string() != "debug" {
            let neato = self.clone();
            tokio::spawn(async { poll_robots_until_all_idle(neato).await });
        }

        result
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

static POLL_RATE: u64 = 60 * 1000;

async fn poll_robots_until_all_idle(neato: Neato) -> Result<()> {
    let poll_rate = Duration::from_millis(POLL_RATE);
    let mut interval = time::interval(poll_rate);

    loop {
        interval.tick().await;


        let event_tx = neato.event_tx.clone();

        let robots = update_robot_states(get_robots(&neato.config).await?).await?;

        let mut idle_vec = Vec::new();
        for robot in robots {
            let r = robot.clone();

            // Add state to idle_vec to determine if we want to quit polling
            idle_vec.push(r.state.as_ref().unwrap().state == RobotState::Idle);

            let device = mk_neato_device(&neato, &r);
            debug!("Neato: {:?}", device);
            event_tx.send(Message::RecvDeviceState {
                device,
                // set_scene: false,
            });
        }

        if idle_vec.iter().all(|&x| x) {
            break;
        }
    }
    Ok(())
}