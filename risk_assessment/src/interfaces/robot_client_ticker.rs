use crate::*;
use futures::Future;
use micro_sp::*;
use r2r::{
    risk_assessment_msgs::{msg::Emulation, srv::TriggerRobot},
    Error, QosProfile,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

pub async fn spawn_robot_client_ticker(
    arc_node: Arc<Mutex<r2r::Node>>,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>,//HashMap<String, AtomicUsize>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = arc_node
        .lock()
        .unwrap()
        .create_client::<TriggerRobot::Service>(
            "/robot_emulator_service",
            QosProfile::default(),
        )?;
    let waiting_for_server = r2r::Node::is_available(&client)?;

    let timer = arc_node
        .lock()
        .unwrap()
        .create_wall_timer(std::time::Duration::from_millis(CLIENT_TICKER_RATE))?;

    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        match robot_client_ticker(&client, waiting_for_server, &shared_state_clone, timer).await {
            Ok(()) => r2r::log_info!("robot_interface", "Succeeded."),
            Err(e) => r2r::log_error!("robot_interface", "Failed with: '{}'.", e),
        };
    });
    Ok(())
}

pub async fn robot_client_ticker(
    client: &r2r::Client<TriggerRobot::Service>,
    wait_for_server: impl Future<Output = Result<(), Error>>,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>,//HashMap<String, AtomicUsize>)>,
    mut timer: r2r::Timer,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = "robot_client_ticker";
    r2r::log_warn!("robot_interface", "Waiting for the server...");
    wait_for_server.await?;
    r2r::log_info!("robot_interface", "Server available.");

    r2r::log_info!("robot_interface", "Spawned.");

    // let mut last_known_global_version = match shared_state.1.get("global") {
    //     Some(version) => version.load(Ordering::SeqCst),
    //     None => {
    //         r2r::log_warn!("robot_interface", "Couldn't get 'global' atomic counter.");
    //         0
    //     }
    // };

    // let mut last_known_local_version = match shared_state.1.get("robot_interface") {
    //     Some(version) => version.load(Ordering::SeqCst),
    //     None => {
    //         r2r::log_warn!(
    //             "robot_interface",
    //             "Couldn't get 'robot_interface' atomic counter."
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
    //     let mut current_local_version = match shared_state.1.get("robot_interface") {
    //         Some(version) => version.load(Ordering::SeqCst),
    //         None => {
    //             r2r::log_warn!(
    //                 "robot_interface",
    //                 "Couldn't get 'robot_interface' atomic counter."
    //             );
    //             0
    //         }
    //     };

        // 4 scenarios:
            //

        if current_local_version != last_known_local_version {
            // state has been updated by the "robot_interface" task
            // println!(
            //     "{} - {}",
            //     current_local_version, last_known_local_version
            // );
            // r2r::log_warn!("robot_interface", "state has been updated by the 'robot_interface' task");
            last_known_local_version = current_local_version;
            let state = shared_state.0.lock().unwrap().clone();
            let mut request_trigger = state.get_or_default_bool(target, "robot_request_trigger");
            let mut request_state = state.get_or_default_string(target, "robot_request_state");
            let mut total_fail_counter =
                state.get_or_default_i64(target, "robot_total_fail_counter");
            let mut subsequent_fail_counter =
                state.get_or_default_i64(target, "robot_subsequent_fail_counter");
            let robot_command_command =
                state.get_or_default_string(target, "robot_command_command");
            let robot_speed_command = state.get_or_default_f64(target, "robot_speed_command");
            let robot_position_command =
                state.get_or_default_string(target, "robot_position_command");
            let mut robot_position_estimated =
                state.get_or_default_string(target, "robot_position_estimated");
            let mut robot_calibrated_estimated =
                state.get_or_default_bool(target, "robot_calibrated_estimated");
            let mut robot_locked_estimated = state.get_bool(target, "robot_locked_estimated");
            let mut robot_mounted_one_time_measured = state.get_or_default_string(target, "robot_mounted_one_time_measured");
            let emulate_execution_time =
                state.get_or_default_i64(target, "robot_emulate_execution_time");
            let emulated_execution_time =
                state.get_or_default_i64(target, "robot_emulated_execution_time");
            let emulate_failure_rate =
                state.get_or_default_i64(target, "robot_emulate_failure_rate");
            let emulated_failure_rate =
                state.get_or_default_i64(target, "robot_emulated_failure_rate");
            let emulate_failure_cause =
                state.get_or_default_i64(target, "robot_emulate_failure_cause");
            let emulated_failure_cause =
                state.get_or_default_array_of_strings(target, "robot_emulated_failure_cause");

            if request_trigger {
                request_trigger = false;
                if request_state == ServiceRequestState::Initial.to_string() {
                    r2r::log_info!(
                        "robot_interface",
                        "Requesting to {}.",
                        robot_command_command
                    );
                    let request = TriggerRobot::Request {
                        command: robot_command_command.clone(),
                        speed: robot_speed_command as f32,
                        position: robot_position_command.clone(),
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
                            Ok(response) => match robot_command_command.as_str() {
                                "move" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "robot_interface",
                                            "Requested move to '{}' succeeded.",
                                            robot_position_command
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        robot_position_estimated = robot_position_command;
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "robot_interface",
                                            "Requested move to '{}' failed.",
                                            robot_position_command
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "pick" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "robot_interface",
                                            "Requested pick succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "robot_interface",
                                            "Requested pick failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "place" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "robot_interface",
                                            "Requested place succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "robot_interface",
                                            "Requested place failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "mount" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "robot_interface",
                                            "Requested mount succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "robot_interface",
                                            "Requested mount failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "unmount" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "robot_interface",
                                            "Requested unmount succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "robot_interface",
                                            "Requested unmount failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                "check_mounted_tool" => {
                                    if response.success {
                                        r2r::log_info!(
                                            "robot_interface",
                                            "Requested check_mounted_tool succeeded."
                                        );
                                        request_state = ServiceRequestState::Succeeded.to_string();
                                        robot_mounted_one_time_measured = response.checked_mounted_tool;
                                        subsequent_fail_counter = 0;
                                    } else {
                                        r2r::log_error!(
                                            "robot_interface",
                                            "Requested check_mounted_tool failed."
                                        );
                                        request_state = ServiceRequestState::Failed.to_string();
                                        robot_mounted_one_time_measured = "unknown".to_string();
                                        subsequent_fail_counter = subsequent_fail_counter + 1;
                                        total_fail_counter = total_fail_counter + 1;
                                    }
                                }
                                _ => {
                                    r2r::log_info!(
                                        "robot_interface",
                                        "Requested command '{}' is invalid.",
                                        robot_command_command
                                    );
                                    request_state = ServiceRequestState::Failed.to_string();
                                    subsequent_fail_counter = subsequent_fail_counter + 1;
                                    total_fail_counter = total_fail_counter + 1;
                                }
                            },
                            Err(e) => {
                                r2r::log_info!("robot_interface", "Request failed with: {e}.");
                                request_state = ServiceRequestState::Failed.to_string();
                                subsequent_fail_counter = subsequent_fail_counter + 1;
                                total_fail_counter = total_fail_counter + 1;
                            }
                        },
                        Err(e) => {
                            r2r::log_info!("robot_interface", "Request failed with: {e}.");
                            request_state = ServiceRequestState::Failed.to_string();
                            subsequent_fail_counter = subsequent_fail_counter + 1;
                            total_fail_counter = total_fail_counter + 1;
                        }
                    };
                }
            }
            let updated_state = state
                .update("robot_request_trigger", request_trigger.to_spvalue())
                .update("robot_request_state", request_state.to_spvalue())
                .update("robot_total_fail_counter", total_fail_counter.to_spvalue())
                .update(
                    "robot_subsequent_fail_counter",
                    subsequent_fail_counter.to_spvalue(),
                )
                .update(
                    "robot_position_estimated",
                    robot_position_estimated.to_spvalue(),
                )
                .update(
                    "robot_calibrated_estimated",
                    robot_calibrated_estimated.to_spvalue(),
                )
                .update(
                    "robot_locked_estimated",
                    robot_locked_estimated.to_spvalue(),
                )
                .update(
                    "robot_mounted_one_time_measured",
                    robot_mounted_one_time_measured.to_spvalue(),
                );
            // shared_state
            //     .1
            //     .entry("robot_interface".to_string())
            //     .and_modify(|version| {
            //         version.fetch_add(1, Ordering::SeqCst);
            //     })
            //     .or_default();
            shared_state.1[1].fetch_add(1, Ordering::SeqCst);
            *shared_state.0.lock().unwrap() = updated_state.clone();
        } else {
            r2r::log_warn!("robot_interface", "state has not yet been updated by the 'robot_interface' task");
            // state has not yet been updated by the "robot_interface" task
        }
        timer.tick().await?;
    }
}
