use rand::seq::SliceRandom;
use std::error::Error;
use std::sync::{Arc, Mutex};

use micro_sp::*;
use risk_assessment::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup the node
    let ctx = r2r::Context::create()?;
    let node = r2r::Node::create(ctx, NODE_ID, "")?;
    let arc_node = Arc::new(Mutex::new(node));

    let state = models::minimal::state::state();
    let (model, state) = models::minimal::model::minimal_model(&state);
    let shared_state = Arc::new(Mutex::new(state));

    r2r::log_info!(NODE_ID, "Spawning emulators...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    tokio::task::spawn(async move {
        gantry_emulator::spawn_gantry_emulator_server(
            arc_node_clone,
        )
        .await
        .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    r2r::log_info!(NODE_ID, "Spawning interfaces...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        gantry_client_ticker::spawn_gantry_client_ticker(arc_node_clone, shared_state_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    r2r::log_info!(NODE_ID, "Spawning operation planner...");

    let shared_state_clone = shared_state.clone();
    let model_clone = model.clone();
    tokio::task::spawn(async move {
        planner_ticker(&model_clone.name, &model_clone, &shared_state_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    r2r::log_info!(NODE_ID, "Spawning operation runner...");

    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        simple_operation_runner(&model.name, &model, &shared_state_clone, false)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    r2r::log_info!(NODE_ID, "Spawning test generator...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        perform_test(arc_node_clone, shared_state_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

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
    arc_node: Arc<Mutex<r2r::Node>>,
    shared_state: Arc<Mutex<State>>,
) -> Result<(), Box<dyn Error>> {
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    r2r::log_warn!(NODE_ID, "Starting tests...");
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    r2r::log_warn!(NODE_ID, "Tests started.");

    let shared_state_local = shared_state.lock().unwrap().clone();

    let goal = "var:gantry_position_estimated == b";
    let updated_state = shared_state_local.update("minimal_model_goal", goal.to_spvalue())
        .update("minimal_model_replan_trigger", true.to_spvalue())
        .update("minimal_model_replanned", false.to_spvalue());

    *shared_state.lock().unwrap() = updated_state;
    

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
fn generate_random_initial_state(state: &State) -> State {
    let state = state.update(
        "gantry_command_command",
        vec!["move", "calibrate", "lock", "unlock"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_spvalue(),
    );
    let state = state.update(
        "gantry_command_speed",
        vec![0.0, 1.0, 2.0]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_spvalue(),
    );
    let state = state.update(
        "gantry_position_command",
        vec!["a", "b", "c", "d"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_spvalue(),
    );

    

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
    state
}
