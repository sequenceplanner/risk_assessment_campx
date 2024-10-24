use crate::*;
use micro_sp::*;
use r2r::{
    risk_assessment_msgs::{msg::Emulation, srv::TriggerGantry},
    QosProfile,
};
use std::sync::{Arc, Mutex};

fn get_or_default_bool(name: &str, state: &State) -> bool {
    match state.get_value(name) {
        micro_sp::SPValue::Bool(value) => value,
        _ => {
            r2r::log_error!(NODE_ID, "Couldn't get '{}' from the shared state.", name);
            false
        }
    }
}

fn get_or_default_i64(name: &str, state: &State) -> i64 {
    match state.get_value(name) {
        micro_sp::SPValue::Int64(value) => value,
        _ => {
            r2r::log_error!(NODE_ID, "Couldn't get '{}' from the shared state.", name);
            0
        }
    }
}

fn get_or_default_f64(name: &str, state: &State) -> f64 {
    match state.get_value(name) {
        micro_sp::SPValue::Float64(value) => value.into_inner(),
        _ => {
            r2r::log_error!(NODE_ID, "Couldn't get '{}' from the shared state.", name);
            0.0
        }
    }
}

fn get_or_default_string(name: &str, state: &State) -> String {
    match state.get_value(name) {
        micro_sp::SPValue::String(value) => value,
        _ => {
            r2r::log_error!(NODE_ID, "Couldn't get '{}' from the shared state.", name);
            "unknown".to_string()
        }
    }
}

fn get_or_default_array_of_strings(name: &str, state: &State) -> Vec<String> {
    match state.get_value(name) {
        micro_sp::SPValue::Array(SPValueType::String, arr) => arr
            .iter()
            .map(|x| match x {
                micro_sp::SPValue::String(value) => value.clone(),
                _ => "unknown".to_string(),
            })
            .collect(),
        _ => {
            r2r::log_error!(NODE_ID, "Couldn't get '{}' from the shared state.", name);
            vec![]
        }
    }
}

pub async fn spawn_gantry_client_ticker(
    arc_node: Arc<Mutex<r2r::Node>>,
    shared_state: Arc<Mutex<State>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = arc_node
        .lock()
        .unwrap()
        .create_client::<TriggerGantry::Service>(
            "/gantry_emulator_service",
            QosProfile::default(),
        )?;
    let waiting_for_server = r2r::Node::is_available(&client)?;

    r2r::log_warn!(NODE_ID, "[gantry_client]: Waiting for the gantry server...");
    waiting_for_server.await?;
    r2r::log_warn!(NODE_ID, "[gantry_client]: Gantry server available.");

    r2r::log_info!(EMULATOR_NODE_ID, "Gantry interface spawned.");

    let mut timer = arc_node
        .lock()
        .unwrap()
        .create_wall_timer(std::time::Duration::from_millis(CLIENT_TICKER_RATE))?;

    loop {
        let shared_state_local = shared_state.lock().unwrap().clone();
        let request_trigger = get_or_default_bool("gantry_request_trigger", &shared_state_local);
        let request_state = get_or_default_string("gantry_request_state", &shared_state_local);
        let total_fail_counter =
            get_or_default_i64("gantry_total_fail_counter", &shared_state_local);
        let subsequent_fail_counter =
            get_or_default_i64("gantry_subsequent_fail_counter", &shared_state_local);
        let gantry_command_command =
            get_or_default_string("gantry_command_command", &shared_state_local);
        let gantry_speed_command = get_or_default_f64("gantry_speed_command", &shared_state_local);
        let gantry_position_command =
            get_or_default_string("gantry_position_command", &shared_state_local);
        let emulate_execution_time =
            get_or_default_i64("emulate_execution_time", &shared_state_local);
        let emulated_execution_time =
            get_or_default_i64("emulated_execution_time", &shared_state_local);
        let emulate_failure_rate = get_or_default_i64("emulate_failure_rate", &shared_state_local);
        let emulated_failure_rate =
            get_or_default_i64("emulated_failure_rate", &shared_state_local);
        let emulate_failure_cause =
            get_or_default_i64("emulate_failure_cause", &shared_state_local);
        let emulated_failure_cause =
            get_or_default_array_of_strings("emulated_failure_cause", &shared_state_local);

        if request_trigger {
            if request_state == ServiceRequestState::Initial.to_string() {
                r2r::log_info!(
                    NODE_ID,
                    "[gantry_client]: Requesting to {}.",
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

                let updated_state = match client.request(&request) {
                    Ok(future) => match future.await {
                        Ok(response) => match gantry_command_command.as_str() {
                            "move" => {
                                if response.success {
                                    r2r::log_info!(
                                        NODE_ID,
                                        "[gantry_client]: Requested move to '{}' succeeded.",
                                        gantry_position_command
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Succeeded.to_string().to_spvalue(),
                                        )
                                        .update(
                                            "gantry_position_estimated",
                                            gantry_position_command.to_spvalue(),
                                        )
                                        .update("gantry_subsequent_fail_counter", 0.to_spvalue())
                                } else {
                                    r2r::log_error!(
                                        NODE_ID,
                                        "[gantry_client]: Requested move to '{}' failed.",
                                        gantry_position_command
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Failed.to_string().to_spvalue(),
                                        )
                                        .update(
                                            "gantry_subsequent_fail_counter",
                                            (subsequent_fail_counter + 1).to_spvalue(),
                                        )
                                        .update(
                                            "gantry_total_fail_counter",
                                            (total_fail_counter + 1).to_spvalue(),
                                        )
                                }
                            }
                            "calibrate" => {
                                if response.success {
                                    r2r::log_info!(
                                        NODE_ID,
                                        "[gantry_client]: Requested calibration succeeded."
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Succeeded.to_string().to_spvalue(),
                                        )
                                        .update("gantry_calibrated_estimated", true.to_spvalue())
                                        .update("gantry_subsequent_fail_counter", 0.to_spvalue())
                                } else {
                                    r2r::log_error!(
                                        NODE_ID,
                                        "[gantry_client]: Requested calibration failed."
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Failed.to_string().to_spvalue(),
                                        )
                                        .update(
                                            "gantry_subsequent_fail_counter",
                                            (subsequent_fail_counter + 1).to_spvalue(),
                                        )
                                        .update(
                                            "gantry_total_fail_counter",
                                            (total_fail_counter + 1).to_spvalue(),
                                        )
                                }
                            }
                            "lock" => {
                                if response.success {
                                    r2r::log_info!(
                                        NODE_ID,
                                        "[gantry_client]: Requested lock succeeded."
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Succeeded.to_string().to_spvalue(),
                                        )
                                        .update("gantry_locked_estimated", true.to_spvalue())
                                        .update("gantry_subsequent_fail_counter", 0.to_spvalue())
                                } else {
                                    r2r::log_error!(
                                        NODE_ID,
                                        "[gantry_client]: Requested lock failed."
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Failed.to_string().to_spvalue(),
                                        )
                                        .update(
                                            "gantry_subsequent_fail_counter",
                                            (subsequent_fail_counter + 1).to_spvalue(),
                                        )
                                        .update(
                                            "gantry_total_fail_counter",
                                            (total_fail_counter + 1).to_spvalue(),
                                        )
                                }
                            }
                            "unlock" => {
                                if response.success {
                                    r2r::log_info!(
                                        NODE_ID,
                                        "[gantry_client]: Requested unlock succeeded."
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Succeeded.to_string().to_spvalue(),
                                        )
                                        .update("gantry_locked_estimated", false.to_spvalue())
                                        .update("gantry_subsequent_fail_counter", 0.to_spvalue())
                                } else {
                                    r2r::log_error!(
                                        NODE_ID,
                                        "[gantry_client]: Requested unlock failed."
                                    );
                                    shared_state_local
                                        .update(
                                            "gantry_request_state",
                                            ServiceRequestState::Failed.to_string().to_spvalue(),
                                        )
                                        .update(
                                            "gantry_subsequent_fail_counter",
                                            (subsequent_fail_counter + 1).to_spvalue(),
                                        )
                                        .update(
                                            "gantry_total_fail_counter",
                                            (total_fail_counter + 1).to_spvalue(),
                                        )
                                }
                            }
                            _ => {
                                r2r::log_info!(
                                    NODE_ID,
                                    "[gantry_client]: Requested command '{}' is invalid.",
                                    gantry_command_command
                                );
                                shared_state_local
                            }
                        },
                        Err(e) => {
                            r2r::log_info!(NODE_ID, "[gantry_client]: Request failed with: {e}.");
                            shared_state_local
                                .update(
                                    "gantry_request_state",
                                    ServiceRequestState::Failed.to_string().to_spvalue(),
                                )
                                .update(
                                    "gantry_subsequent_fail_counter",
                                    (subsequent_fail_counter + 1).to_spvalue(),
                                )
                                .update(
                                    "gantry_total_fail_counter",
                                    (total_fail_counter + 1).to_spvalue(),
                                )
                        }
                    },
                    Err(e) => {
                        r2r::log_info!(NODE_ID, "[gantry_client]: Request failed with: {e}.");
                        shared_state_local
                            .update(
                                "gantry_request_state",
                                ServiceRequestState::Failed.to_string().to_spvalue(),
                            )
                            .update(
                                "gantry_subsequent_fail_counter",
                                (subsequent_fail_counter + 1).to_spvalue(),
                            )
                            .update(
                                "gantry_total_fail_counter",
                                (total_fail_counter + 1).to_spvalue(),
                            )
                    }
                };
                *shared_state.lock().unwrap() = updated_state;
            }
        }

        timer.tick().await?;
    }
}

// pub async fn gantry_client_ticker(
//     // gantry_cient: &r2r::Client<TriggerGantry::Service>,
//     // wait_for_server: impl Future<Output = Result<(), Error>>,
//     // shared_state: &Arc<Mutex<State>>,
//     // mut timer: r2r::Timer,
//     // node_id: &str,
//     arc_node: Arc<Mutex<r2r::Node>>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // r2r::log_warn!(node_id, "Waiting for the gantry server...");
//     // wait_for_server.await?;
//     // r2r::log_warn!(node_id, "Gantry server available.");

//     loop {
//     //     let shsl = shared_state.lock().unwrap().clone();
//     //     let gantry_request_trigger = match shsl.get_value("gantry_request_trigger") {
//     //         SPValue::Bool(value) => value,
//     //         _ => false,
//     //     };
//     //     let gantry_command = match shsl.get_value("gantry_command") {
//     //         SPValue::String(value) => value,
//     //         _ => "unknown".to_string(),
//     //     };
//     //     let gantry_request_state = match shsl.get_value("gantry_request_state") {
//     //         SPValue::String(value) => value,
//     //         _ => "unknown".to_string(),
//     //     };
//     //     if gantry_request_trigger {
//     //         if gantry_request_state == "initial".to_string() {
//     //             let request = TriggerGantry::Request {
//     //                 position: gantry_command.to_string(),
//     //             };

//     //             let response = gantry_cient
//     //                 .request(&request)
//     //                 .expect("Could not send gantry request.")
//     //                 .await
//     //                 .expect("Cancelled.");

//     //             let shsl = shared_state.lock().unwrap().clone();

//     //             *shared_state.lock().unwrap() = match response.success {
//     //                 true => shsl
//     //                     .update("gantry_request_state", "succeeded".to_spvalue())
//     //                     .update("gantry_actual_state", response.state.to_spvalue()),

//     //                 false => {
//     //                     let scanner_fail_counter = match shsl.get_value("fail_counter_gantry") {
//     //                         SPValue::Int32(value) => value,
//     //                         _ => 0,
//     //                     };
//     //                     shsl.update("gantry_request_state", "failed".to_spvalue())
//     //                         .update(
//     //                             "fail_counter_gantry",
//     //                             (scanner_fail_counter + 1).to_spvalue(),
//     //                         )
//     //                 }
//     //             };
//     //         }
//     //     }

//         // timer.tick().await?;
//     }
// }
