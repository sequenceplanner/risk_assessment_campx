use futures::{Stream, StreamExt};
use r2r::risk_assessment_msgs::srv::TriggerRobot;
use rand::prelude::SliceRandom;
use r2r::QosProfile;
use r2r::ServiceRequest;
use rand::Rng;
use std::sync::{Arc, Mutex};

pub async fn spawn_robot_emulator_server(
    arc_node: Arc<Mutex<r2r::Node>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = arc_node
        .lock()
        .unwrap()
        .create_service::<TriggerRobot::Service>(
            "/robot_emulator_service",
            QosProfile::default(),
        )?;

    tokio::task::spawn(async move {
        let result = robot_emlator_server(service).await;
        match result {
            Ok(()) => r2r::log_info!("robot_emulator", "Service call succeeded."),
            Err(e) => r2r::log_error!("robot_emulator", "Service call failed with: {}.", e),
        };
    });
    Ok(())
}

async fn robot_emlator_server(
    mut service: impl Stream<Item = ServiceRequest<TriggerRobot::Service>> + Unpin,
) -> Result<(), Box<dyn std::error::Error>> {
    r2r::log_info!("robot_emulator", "Spawned.");
    loop {
        match service.next().await {
            Some(request) => {
                // emulate request execution time
                let delay: u64 = match request.message.emulated_response.emulate_execution_time {
                    0 => 0,
                    1 => request.message.emulated_response.emulated_execution_time as u64,
                    2 => {
                        let mut rng = rand::thread_rng();
                        rng.gen_range(0..request.message.emulated_response.emulated_execution_time) as u64
                    },
                    _ => 0
                };
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;

                // emulate failure rate
                let mut fail = match request.message.emulated_response.emulate_failure_rate {
                    0 => false,
                    1 => true,
                    2 => rand::thread_rng().gen_range(0..=100) <= request.message.emulated_response.emulated_failure_rate as u64,
                    _ => false
                };

                // emulate failure cause
                let cause = match request.message.emulated_response.emulate_failure_cause {
                    0 => "generic_failure".to_string(),
                    1 => request.message.emulated_response.emulated_failure_cause[0].to_string(),
                    2 => request.message.emulated_response.emulated_failure_cause
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string(),
                    _ => "generic_failure".to_string(),
                };

                let mut checked_mounted_tool = "UNKNOWN".to_string();
                match request.message.command.as_str() {
                    "move" => r2r::log_info!(
                        "robot_emulator",
                        "Got request to move to {}.",
                        request.message.position
                    ),
                    "pick" => {
                        r2r::log_info!("robot_emulator", "Got request to pick.")
                    }
                    "place" => r2r::log_info!("robot_emulator", "Got request to place."),
                    "mount" => r2r::log_info!("robot_emulator", "Got request to mount."),
                    "unmount" => r2r::log_info!("robot_emulator", "Got request to unmount."),
                    "check_mounted_tool" => {
                        checked_mounted_tool = vec!("gripper_tool", "suction_tool", "none").choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string();
                        r2r::log_info!("robot_emulator", "Got request to check_mounted_tool.")
                    },
                    _ => {
                        r2r::log_warn!("robot_emulator", "Unknown command");
                        fail = true;
                    },
                };

                let success_info = match request.message.command.as_str() {
                    "move" => format!("Succeeded to move to {}.",
                        request.message.position
                    ),
                    "pick" => "Succeeded to pick.".to_string(),
                    "place" => "Succeeded to place.".to_string(),
                    "mount" => "Succeeded to mount.".to_string(),
                    "unmount" => "Succeeded to unmount.".to_string(),
                    "check_mounted_tool" => "Succeeded to check_mounted_tool.".to_string(),
                    _ => "Failed, unknown command".to_string()
                };

                let failure_info = match request.message.command.as_str() {
                    "move" => format!("Failed to move to {} due to {}.",
                        request.message.position, cause
                    ),
                    "pick" => format!("Failed to pick due to {}.", cause),
                    "place" => format!("Failed to place due to {}.", cause),
                    "mount" => format!("Failed to mount due to {}.", cause),
                    "unmount" => format!("Failed to unmount due to {}.", cause),
                    "check_mounted_tool" => format!("Failed to check_mounted_tool due to {}.", cause),
                    _ => "Failed, unknown command".to_string()
                };

                if !fail {
                    let response = TriggerRobot::Response {
                        success: true,
                        failure_cause: "".to_string(),
                        info: success_info.clone(),
                        checked_mounted_tool
                    };
                    r2r::log_info!("robot_emulator", "{}", success_info);
                    request
                        .respond(response)
                        .expect("Could not send service response.");
                    continue;
                } else {
                    let response = TriggerRobot::Response {
                        success: false,
                        failure_cause: cause,
                        info: failure_info.clone(),
                        checked_mounted_tool
                    };
                    r2r::log_error!("robot_emulator", "{}", failure_info);
                    request
                        .respond(response)
                        .expect("Could not send service response.");
                    continue;
                }
            }

            None => (),
        }
    }
}
