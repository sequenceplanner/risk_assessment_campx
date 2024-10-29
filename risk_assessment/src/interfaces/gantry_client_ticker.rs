use crate::*;
use futures::Future;
use micro_sp::*;
use r2r::{
    risk_assessment_msgs::{msg::Emulation, srv::TriggerGantry},
    Error, QosProfile,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

pub async fn spawn_gantry_client_ticker(
    arc_node: Arc<Mutex<r2r::Node>>,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>,//HashMap<String, AtomicUsize>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = arc_node
        .lock()
        .unwrap()
        .create_client::<TriggerGantry::Service>(
            "/gantry_emulator_service",
            QosProfile::default(),
        )?;
    let waiting_for_server = r2r::Node::is_available(&client)?;

    let timer = arc_node
        .lock()
        .unwrap()
        .create_wall_timer(std::time::Duration::from_millis(CLIENT_TICKER_RATE))?;

    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        match gantry_client_ticker(&client, waiting_for_server, &shared_state_clone, timer).await {
            Ok(()) => r2r::log_info!("gantry_interface", "Succeeded."),
            Err(e) => r2r::log_error!("gantry_interface", "Failed with: '{}'.", e),
        };
    });
    Ok(())
}

pub async fn gantry_client_ticker(
    client: &r2r::Client<TriggerGantry::Service>,
    wait_for_server: impl Future<Output = Result<(), Error>>,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>,//HashMap<String, AtomicUsize>)>,
    mut timer: r2r::Timer,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = "gantry_client_ticker";
    r2r::log_warn!("gantry_interface", "Waiting for the server...");
    wait_for_server.await?;
    r2r::log_info!("gantry_interface", "Server available.");

    r2r::log_info!("gantry_interface", "Spawned.");

    // let mut last_known_global_version = match shared_state.1.get("global") {
    //     Some(version) => version.load(Ordering::SeqCst),
    //     None => {
    //         r2r::log_warn!("gantry_interface", "Couldn't get 'global' atomic counter.");
    //         0
    //     }
    // };

    // let mut last_known_local_version = match shared_state.1.get("gantry_interface") {
    //     Some(version) => version.load(Ordering::SeqCst),
    //     None => {
    //         r2r::log_warn!(
    //             "gantry_interface",
    //             "Couldn't get 'gantry_interface' atomic counter."
    //         );
    //         0
    //     }
    // };

    let mut last_known_global_version = shared_state.1[0].load(Ordering::SeqCst);
    let mut last_known_local_version = 0;

    loop {
        let current_local_version = shared_state.1[1].load(Ordering::SeqCst);
        let runner_version = shared_state.1[2].load(Ordering::SeqCst);
        let planner_version = shared_state.1[3].load(Ordering::SeqCst);
        // println!("runner_version: {}", runner_version);
        // println!("planner_version: {}", planner_version);
        // println!("current_local_version: {}", current_local_version);
    //     let mut current_local_version = match shared_state.1.get("gantry_interface") {
    //         Some(version) => version.load(Ordering::SeqCst),
    //         None => {
    //             r2r::log_warn!(
    //                 "gantry_interface",
    //                 "Couldn't get 'gantry_interface' atomic counter."
    //             );
    //             0
    //         }
    //     };

        // 4 scenarios:
            //

        if current_local_version != last_known_local_version {
            // state has been updated by the "gantry_interface" task
            // println!(
            //     "{} - {}",
            //     current_local_version, last_known_local_version
            // );
            // r2r::log_warn!("gantry_interface", "state has been updated by the 'gantry_interface' task");
            last_known_local_version = current_local_version;
            let state = shared_state.0.lock().unwrap().clone();
            let mut request_trigger = state.get_or_default_bool(target, "gantry_request_trigger");
            let mut request_state = state.get_or_default_string(target, "gantry_request_state");
            let mut total_fail_counter =
                state.get_or_default_i64(target, "gantry_total_fail_counter");
            let mut subsequent_fail_counter =
                state.get_or_default_i64(target, "gantry_subsequent_fail_counter");
            let gantry_command_command =
                state.get_or_default_string(target, "gantry_command_command");
            let gantry_speed_command = state.get_or_default_f64(target, "gantry_speed_command");
            let gantry_position_command =
                state.get_or_default_string(target, "gantry_position_command");
            let mut gantry_position_estimated =
                state.get_or_default_string(target, "gantry_position_estimated");
            let mut gantry_calibrated_estimated =
                state.get_or_default_bool(target, "gantry_calibrated_estimated");
            let mut gantry_locked_estimated = state.get_bool(target, "gantry_locked_estimated");
            let emulate_execution_time =
                state.get_or_default_i64(target, "gantry_emulate_execution_time");
            let emulated_execution_time =
                state.get_or_default_i64(target, "gantry_emulated_execution_time");
            let emulate_failure_rate =
                state.get_or_default_i64(target, "gantry_emulate_failure_rate");
            let emulated_failure_rate =
                state.get_or_default_i64(target, "gantry_emulated_failure_rate");
            let emulate_failure_cause =
                state.get_or_default_i64(target, "gantry_emulate_failure_cause");
            let emulated_failure_cause =
                state.get_or_default_array_of_strings(target, "gantry_emulated_failure_cause");

            if request_trigger {
                request_trigger = false;
                if request_state == ServiceRequestState::Initial.to_string() {
                    r2r::log_info!(
                        "gantry_interface",
                        "Requesting to {}.",
                        gantry_command_command
                    );
                    let request = TriggerGantry::Request {
                        command: gantry_command_command.clone(),
                        speed: gantry_speed_command as f32,
                        position: gantry_position_command.clone(),
                        emulated_response: Emulation {
                            emulate_execution_time: emulate_execution_time as u8,
                            emulated_execution_time: emulated_execution_time as i32,
                            emulate_failure_rate: emulate_failure_rate as u8,
                            emulated_failure_rate: emulated_failure_rate as i32,
                            emulate_failure_cause: emulate_failure_cause as u8,
                            emulated_failure_cause,
                        },
                    };

                    match client.request(&request) {
                        Ok(future) => match future.await {
                            Ok(response) => match gantry_command_command.as_str() {
                                "move" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "gantry_interface",
                                            "Requested move to '{}' succeeded.",
                                            gantry_position_command
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        gantry_position_estimated = gantry_position_command;
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "gantry_interface",
                                            "Requested move to '{}' failed.",
                                            gantry_position_command
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "calibrate" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "gantry_interface",
                                            "Requested calibration succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        gantry_calibrated_estimated = true;
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "gantry_interface",
                                            "Requested calibration failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "lock" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "gantry_interface",
                                            "Requested lock succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        gantry_locked_estimated = Some(true);
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "gantry_interface",
                                            "Requested lock failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "unlock" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "gantry_interface",
                                            "Requested unlock succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        gantry_locked_estimated = Some(false);
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "gantry_interface",
                                            "Requested unlock failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                _ => {
                                    r2r::log_info!(
                                        "gantry_interface",
                                        "Requested command '{}' is invalid.",
                                        gantry_command_command
                                    );
                                    request_state = ServiceRequestState::Failed.to_string();
                                    subsequent_fail_counter = subsequent_fail_counter + 1;
                                    total_fail_counter = total_fail_counter + 1;
                                }
                            },
                            Err(e) => {
                                r2r::log_info!("gantry_interface", "Request failed with: {e}.");
                                request_state = ServiceRequestState::Failed.to_string();
                                subsequent_fail_counter = subsequent_fail_counter + 1;
                                total_fail_counter = total_fail_counter + 1;
                            }
                        },
                        Err(e) => {
                            r2r::log_info!("gantry_interface", "Request failed with: {e}.");
                            request_state = ServiceRequestState::Failed.to_string();
                            subsequent_fail_counter = subsequent_fail_counter + 1;
                            total_fail_counter = total_fail_counter + 1;
                        }
                    };
                }
            }
            let updated_state = state
                .update("gantry_request_trigger", request_trigger.to_spvalue())
                .update("gantry_request_state", request_state.to_spvalue())
                .update("gantry_total_fail_counter", total_fail_counter.to_spvalue())
                .update(
                    "gantry_subsequent_fail_counter",
                    subsequent_fail_counter.to_spvalue(),
                )
                .update(
                    "gantry_position_estimated",
                    gantry_position_estimated.to_spvalue(),
                )
                .update(
                    "gantry_calibrated_estimated",
                    gantry_calibrated_estimated.to_spvalue(),
                )
                .update(
                    "gantry_locked_estimated",
                    gantry_locked_estimated.to_spvalue(),
                );
            // shared_state
            //     .1
            //     .entry("gantry_interface".to_string())
            //     .and_modify(|version| {
            //         version.fetch_add(1, Ordering::SeqCst);
            //     })
            //     .or_default();
            shared_state.1[1].fetch_add(1, Ordering::SeqCst);
            *shared_state.0.lock().unwrap() = updated_state.clone();
        } else {
            r2r::log_warn!("gantry_interface", "state has not yet been updated by the 'gantry_interface' task");
            // state has not yet been updated by the "gantry_interface" task
        }
        timer.tick().await?;
    }
}
