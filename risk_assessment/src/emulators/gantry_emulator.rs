use futures::{Stream, StreamExt};
use r2r::risk_assessment_msgs::srv::TriggerGantry;
use r2r::QosProfile;
use r2r::ServiceRequest;
use rand::Rng;
use std::sync::{Arc, Mutex};

use crate::*;

pub async fn spawn_gantry_emulator_server(
    arc_node: Arc<Mutex<r2r::Node>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = arc_node
        .lock()
        .unwrap()
        .create_service::<TriggerGantry::Service>(
            "gantry_emulator_service",
            QosProfile::default(),
        )?;

    tokio::task::spawn(async move {
        let result = gantry_emlator_server(service).await;
        match result {
            Ok(()) => r2r::log_info!(EMULATOR_NODE_ID, "Gantry service call succeeded."),
            Err(e) => r2r::log_error!(EMULATOR_NODE_ID, "Gantry service call failed with: {}.", e),
        };
    });
    Ok(())
}

async fn gantry_emlator_server(
    mut service: impl Stream<Item = ServiceRequest<TriggerGantry::Service>> + Unpin,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match service.next().await {
            Some(request) => {
                // emulate request execution time
                let delay: u64 = match request.message.emulate_execution_time {
                    0 => 0,
                    1 => request.message.emulated_execution_time as u64,
                    2 => {
                        let mut rng = rand::thread_rng();
                        rng.gen_range(0..request.message.emulated_execution_time) as u64
                    },
                    _ => 0
                };
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;

                // emulate failure rate
                let mut fail = match request.message.emulate_failure_rate {
                    0 => false,
                    1 => true,
                    2 => rand::thread_rng().gen_range(0..=100) <= request.message.emulated_failure_rate as u64,
                    _ => false
                };

                match request.message.command.as_str() {
                    "move" => r2r::log_info!(
                        EMULATOR_NODE_ID,
                        "Gantry: Got request to move to {}.",
                        request.message.position
                    ),
                    "calibrate" => {
                        r2r::log_info!(EMULATOR_NODE_ID, "Gantry: Got request to calibrate.")
                    }
                    "lock" => r2r::log_info!(EMULATOR_NODE_ID, "Gantry: Got request to lock."),
                    "unlock" => r2r::log_info!(EMULATOR_NODE_ID, "Gantry: Got request to unlock."),
                    _ => {
                        r2r::log_warn!(EMULATOR_NODE_ID, "Gantry: Unknown command");
                        fail = true;
                    },
                };

                let success_info = match request.message.command.as_str() {
                    "move" => format!("Gantry: Succeeded to move to {}.",
                        request.message.position
                    ),
                    "calibrate" => "Gantry: Succeeded to calibrate.".to_string(),
                    "lock" => "Gantry: Succeeded to lock.".to_string(),
                    "unlock" => "Gantry: Succeeded to unlock.".to_string(),
                    _ => "Gantry: Failed, unknown command".to_string()
                };

                let failure_info = match request.message.command.as_str() {
                    "move" => format!("Gantry: Failed to move to {}.",
                        request.message.position
                    ),
                    "calibrate" => "Gantry: Failed to calibrate.".to_string(),
                    "lock" => "Gantry: Failed to lock.".to_string(),
                    "unlock" => "Gantry: Failed to unlock.".to_string(),
                    _ => "Gantry: Failed, unknown command".to_string()
                };

                if !fail {
                    let response = TriggerGantry::Response {
                        success: true,
                        info: success_info.clone(),
                    };
                    r2r::log_info!(NODE_ID, "{}", success_info);
                    request
                        .respond(response)
                        .expect("Could not send service response.");
                    continue;
                } else {
                    let response = TriggerGantry::Response {
                        success: false,
                        info: failure_info.clone(),
                    };
                    r2r::log_error!(NODE_ID, "{}", failure_info);
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
