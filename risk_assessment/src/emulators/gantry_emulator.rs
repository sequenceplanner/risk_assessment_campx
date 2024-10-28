use futures::{Stream, StreamExt};
use r2r::risk_assessment_msgs::srv::TriggerGantry;
use rand::prelude::SliceRandom;
use r2r::QosProfile;
use r2r::ServiceRequest;
use rand::Rng;
use std::sync::{Arc, Mutex};

pub async fn spawn_gantry_emulator_server(
    arc_node: Arc<Mutex<r2r::Node>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = arc_node
        .lock()
        .unwrap()
        .create_service::<TriggerGantry::Service>(
            "/gantry_emulator_service",
            QosProfile::default(),
        )?;

    tokio::task::spawn(async move {
        let result = gantry_emlator_server(service).await;
        match result {
            Ok(()) => r2r::log_info!("gantry_emulator", "Service call succeeded."),
            Err(e) => r2r::log_error!("gantry_emulator", "Service call failed with: {}.", e),
        };
    });
    Ok(())
}

async fn gantry_emlator_server(
    mut service: impl Stream<Item = ServiceRequest<TriggerGantry::Service>> + Unpin,
) -> Result<(), Box<dyn std::error::Error>> {
    r2r::log_info!("gantry_emulator", "Spawned.");
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

                match request.message.command.as_str() {
                    "move" => r2r::log_info!(
                        "gantry_emulator",
                        "Got request to move to {}.",
                        request.message.position
                    ),
                    "calibrate" => {
                        r2r::log_info!("gantry_emulator", "Got request to calibrate.")
                    }
                    "lock" => r2r::log_info!("gantry_emulator", "Got request to lock."),
                    "unlock" => r2r::log_info!("gantry_emulator", "Got request to unlock."),
                    _ => {
                        r2r::log_warn!("gantry_emulator", "Unknown command");
                        fail = true;
                    },
                };

                let success_info = match request.message.command.as_str() {
                    "move" => format!("Succeeded to move to {}.",
                        request.message.position
                    ),
                    "calibrate" => "Succeeded to calibrate.".to_string(),
                    "lock" => "Succeeded to lock.".to_string(),
                    "unlock" => "Succeeded to unlock.".to_string(),
                    _ => "Failed, unknown command".to_string()
                };

                let failure_info = match request.message.command.as_str() {
                    "move" => format!("Failed to move to {} due to {}.",
                        request.message.position, cause
                    ),
                    "calibrate" => format!("Failed to calibrate due to {}.", cause),
                    "lock" => format!("Failed to lock due to {}.", cause),
                    "unlock" => format!("Failed to unlock due to {}.", cause),
                    _ => "Failed, unknown command".to_string()
                };

                if !fail {
                    let response = TriggerGantry::Response {
                        success: true,
                        failure_cause: "".to_string(),
                        info: success_info.clone(),
                    };
                    r2r::log_info!("gantry_emulator", "{}", success_info);
                    request
                        .respond(response)
                        .expect("Could not send service response.");
                    continue;
                } else {
                    let response = TriggerGantry::Response {
                        success: false,
                        failure_cause: cause,
                        info: failure_info.clone(),
                    };
                    r2r::log_error!("gantry_emulator", "{}", failure_info);
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
