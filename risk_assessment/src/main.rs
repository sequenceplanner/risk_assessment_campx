use std::collections::HashMap;
// use rand::seq::SliceRandom;
use std::error::Error;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

use micro_sp::*;
use risk_assessment::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Logs from extern crates to stdout
    initialize_env_logger();

    // Enable coverability tracking:
    let coverability_tracking = false; 

    // Setup the node
    let ctx = r2r::Context::create()?;
    let node = r2r::Node::create(ctx, NODE_ID, "")?;
    let arc_node = Arc::new(Mutex::new(node));

    let state = models::minimal::state::state();

    // Add the variables that keep track of the runner state
    let runner_vars = generate_runner_state_variables("minimal_model");
    let state = state.extend(runner_vars, true);

    let (model, state) = models::minimal::model::minimal_model(&state);

    let op_vars = generate_operation_state_variables(&model, false);
    let state = state.extend(op_vars, true);

    // Shared state synchronization
    // let version_tracker = HashMap::from(
    //     [
    //         ("global".to_string(), AtomicUsize::new(0)),
    //         ("gantry_interface".to_string(), AtomicUsize::new(0)),
    //         (format!("{}_planner", &model.name), AtomicUsize::new(0)),
    //         (format!("{}_runner", &model.name), AtomicUsize::new(0)),
    //     ]
    // );

    let tracker_1 = AtomicUsize::new(1);
    let tracker_2 = AtomicUsize::new(1);
    let tracker_3 = AtomicUsize::new(1);
    let tracker_4 = AtomicUsize::new(1);

    let version_tracker = vec![tracker_1, tracker_2, tracker_3, tracker_4];

    let shared_state = Arc::new((Mutex::new(state), version_tracker));

    r2r::log_info!(NODE_ID, "Spawning emulators...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    tokio::task::spawn(async move {
        spawn_gantry_emulator_server(arc_node_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    r2r::log_info!(NODE_ID, "Spawning interfaces...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let shared_state_clone = shared_state.clone();
    // let global_version_clone = global_version.clone();
    tokio::task::spawn(async move {
        spawn_gantry_client_ticker(arc_node_clone, &shared_state_clone)
            .await
            .unwrap()
    });

    // let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    // let shared_state_clone = shared_state.clone();
    // // let global_version_clone = global_version.clone();
    // tokio::task::spawn(async move {
    //     spawn_scanner_client_ticker(arc_node_clone, &shared_state_clone)
    //         .await
    //         .unwrap()
    // });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    r2r::log_info!(NODE_ID, "Spawning operation planner...");

    let shared_state_clone = shared_state.clone();
    let model_clone = model.clone();
    tokio::task::spawn(async move {
        planner_ticker(&model_clone, &shared_state_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    r2r::log_info!(NODE_ID, "Spawning auto transition runner...");

    let shared_state_clone = shared_state.clone();
    let model_clone = model.clone();
    tokio::task::spawn(async move {
        auto_transition_runner(&model_clone.name, &model_clone, &shared_state_clone, false)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    r2r::log_info!(NODE_ID, "Spawning operation runner...");

    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        operation_runner(&model, &shared_state_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    r2r::log_info!(NODE_ID, "Spawning test generator...");

    // let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        perform_test(&shared_state_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // keep the node alive
    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let handle = std::thread::spawn(move || loop {
        arc_node_clone
            .lock()
            .unwrap()
            .spin_once(std::time::Duration::from_millis(1000));
    });

    r2r::log_info!(NODE_ID, "Node started.");

    handle.join().unwrap();

    Ok(())
}

async fn perform_test(
    // arc_node: Arc<Mutex<r2r::Node>>,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>, //HashMap<String, AtomicUsize>)>,
) -> Result<(), Box<dyn Error>> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    r2r::log_warn!(NODE_ID, "Starting tests...");
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    r2r::log_warn!(NODE_ID, "Tests started.");

    let shared_state_local = shared_state.0.lock().unwrap().clone();

    let goal = "var:gantry_position_estimated == d";
    let updated_state = shared_state_local
        .update("gantry_emulate_failure_rate", 2.to_spvalue())
        .update("gantry_emulated_failure_rate", 30.to_spvalue())
        .update("minimal_model_goal", goal.to_spvalue())
        .update("minimal_model_replan_trigger", true.to_spvalue())
        .update("minimal_model_replanned", false.to_spvalue());

    *shared_state.0.lock().unwrap() = updated_state;

    // let mut ticker_timer = arc_node
    //     .lock()
    //     .unwrap()
    //     .create_wall_timer(std::time::Duration::from_millis(TEST_TICKER_RATE))?;

    // let mut test_case = 0;

    // let shared_state_local = shared_state.lock().unwrap().clone();
    // while test_case < NUMBER_OF_TEST_CASES {
    //     let random_state = generate_random_initial_state(&shared_state_local);
    //     let goal =

    //     ticker_timer.tick().await?;
    // }

    Ok(())
}

// let gantry_position_command = v!("gantry_position_command");
// let state = state.add(assign!(gantry_command_command, SPValue::UNKNOWN));

// Cannot use choose random from domain because the vars don't have a domain in the informal model
// fn generate_random_initial_state(state: &State) -> State {
//     let state = state.update(
//         "gantry_command_command",
//         vec!["move", "calibrate", "lock", "unlock"]
//             .choose(&mut rand::thread_rng())
//             .unwrap()
//             .to_spvalue(),
//     );
//     let state = state.update(
//         "gantry_command_speed",
//         vec![0.0, 1.0, 2.0]
//             .choose(&mut rand::thread_rng())
//             .unwrap()
//             .to_spvalue(),
//     );
//     let state = state.update(
//         "gantry_position_command",
//         vec!["a", "b", "c", "d"]
//             .choose(&mut rand::thread_rng())
//             .unwrap()
//             .to_spvalue(),
//     );

    // Can't do this, no domains
    // let mut state = state.clone();
    // for v in state.get_all_vars() {
    //     match v.value_type {
    //         SPValueType::Bool => {
    //             let value = vec![false, true]
    //                 .choose(&mut rand::thread_rng())
    //                 .unwrap()
    //                 .clone();
    //             state = state.update(&v.name, value.to_spvalue());
    //         }
    //         SPValueType::Float64 => todo!(),
    //         SPValueType::Int64 => todo!(),
    //         SPValueType::String => todo!(),
    //         SPValueType::Time => todo!(),
    //         SPValueType::Array => todo!(),
    //         SPValueType::UNKNOWN => todo!(),
    //     }
    // }
//     state
// }
