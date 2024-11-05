use crate::*;
use micro_sp::*;
use r2r::{
    risk_assessment_msgs::{msg::Emulation, srv::TriggerRobot},
    QosProfile,
};
use std::sync::{
    Arc, Mutex,
};
use tokio::sync::{mpsc, oneshot};

pub async fn robot_client_ticker(
    arc_node: Arc<Mutex<r2r::Node>>,
    command_sender: mpsc::Sender<Command>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = arc_node
        .lock()
        .unwrap()
        .create_client::<TriggerRobot::Service>("/robot_emulator_service", QosProfile::default())?;
    let waiting_for_server = r2r::Node::is_available(&client)?;

    let mut timer = arc_node
        .lock()
        .unwrap()
        .create_wall_timer(std::time::Duration::from_millis(CLIENT_TICKER_RATE))?;

    let target = "robot_client_ticker";
    r2r::log_warn!("robot_interface", "Waiting for the server...");
    waiting_for_server.await?;
    r2r::log_info!("robot_interface", "Server available.");

    r2r::log_info!("robot_interface", "Spawned.");

    loop {
        let (response_tx, response_rx) = oneshot::channel();
        command_sender.send(Command::GetState(response_tx)).await?;
        let state = response_rx.await?;

        let mut request_trigger = state.get_or_default_bool(target, "robot_request_trigger");
        let mut request_state = state.get_or_default_string(target, "robot_request_state");
        let mut total_fail_counter = state.get_or_default_i64(target, "robot_total_fail_counter");
        let mut subsequent_fail_counter =
            state.get_or_default_i64(target, "robot_subsequent_fail_counter");
        let robot_command_command = state.get_or_default_string(target, "robot_command_command");
        let robot_speed_command = state.get_or_default_f64(target, "robot_speed_command");
        let robot_position_command = state.get_or_default_string(target, "robot_position_command");
        let mut robot_position_estimated =
            state.get_or_default_string(target, "robot_position_estimated");
        // let mut robot_mounted_estimated =
        //     state.get_or_default_string(target, "robot_mounted_estimated");
        // let mut robot_locked_estimated = state.get_bool(target, "robot_locked_estimated");
        let mut robot_mounted_one_time_measured =
            state.get_or_default_string(target, "robot_mounted_one_time_measured");
        let emulate_execution_time =
            state.get_or_default_i64(target, "robot_emulate_execution_time");
        let emulated_execution_time =
            state.get_or_default_i64(target, "robot_emulated_execution_time");
        let emulate_failure_rate = state.get_or_default_i64(target, "robot_emulate_failure_rate");
        let emulated_failure_rate = state.get_or_default_i64(target, "robot_emulated_failure_rate");
        let emulate_failure_cause = state.get_or_default_i64(target, "robot_emulate_failure_cause");
        let emulated_failure_cause =
            state.get_or_default_array_of_strings(target, "robot_emulated_failure_cause");

        // r2r::log_error!(NODE_ID, "robot_mounted_estimated: {}.", robot_mounted_estimated);
        // r2r::log_error!(NODE_ID, "robot_mounted_one_time_measured: {}.", robot_mounted_one_time_measured);

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
                                    r2r::log_info!("robot_interface", "Requested pick succeeded.");
                                    request_state = ServiceRequestState::Succeeded.to_string();
                                    subsequent_fail_counter = 0;
                                } else {
                                    r2r::log_error!("robot_interface", "Requested pick failed.");
                                    request_state = ServiceRequestState::Failed.to_string();
                                    subsequent_fail_counter = subsequent_fail_counter + 1;
                                    total_fail_counter = total_fail_counter + 1;
                                }
                            }
                            "place" => {
                                if response.success {
                                    r2r::log_info!("robot_interface", "Requested place succeeded.");
                                    request_state = ServiceRequestState::Succeeded.to_string();
                                    subsequent_fail_counter = 0;
                                } else {
                                    r2r::log_error!("robot_interface", "Requested place failed.");
                                    request_state = ServiceRequestState::Failed.to_string();
                                    subsequent_fail_counter = subsequent_fail_counter + 1;
                                    total_fail_counter = total_fail_counter + 1;
                                }
                            }
                            "mount" => {
                                if response.success {
                                    r2r::log_info!("robot_interface", "Requested mount succeeded.");
                                    request_state = ServiceRequestState::Succeeded.to_string();
                                    subsequent_fail_counter = 0;
                                } else {
                                    r2r::log_error!("robot_interface", "Requested mount failed.");
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
                                    r2r::log_error!("robot_interface", "Requested unmount failed.");
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
                                    robot_mounted_one_time_measured = "UNKNOWN".to_string();
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
        let new_state = state
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
                "robot_mounted_one_time_measured",
                robot_mounted_one_time_measured.to_spvalue(),
            );

        let modified_state = state.get_diff_partial_state(&new_state);
        command_sender
            .send(Command::SetPartialState(modified_state))
            .await?;
        timer.tick().await?;
    }
}
