use std::collections::HashMap;
// use rand::seq::SliceRandom;
use std::error::Error;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{interval, Duration};

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

    let (model, state) = models::minimal::model::minimal_model("minimal_model", &state);
    let name = model.clone().name;

    let op_vars = generate_operation_state_variables(&model, false);
    let state = state.extend(op_vars, true);

    let (tx, rx) = mpsc::channel(100); // Experiment with buffer size
    tokio::spawn(state_manager(rx, state));

    r2r::log_info!(NODE_ID, "Spawning emulators...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    tokio::task::spawn(async move { spawn_gantry_emulator_server(arc_node_clone).await.unwrap() });

    // // tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // // let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    // // tokio::task::spawn(async move { spawn_robot_emulator_server(arc_node_clone).await.unwrap() });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    r2r::log_info!(NODE_ID, "Spawning interfaces...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let tx_clone = tx.clone();
    tokio::task::spawn(async move {
        gantry_client_ticker(arc_node_clone, tx_clone)
            .await
            .unwrap()
    });

    // tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    // let shared_state_clone = shared_state.clone();
    // // let global_version_clone = global_version.clone();
    // tokio::task::spawn(async move {
    //     spawn_robot_client_ticker(arc_node_clone, &shared_state_clone)
    //         .await
    //         .unwrap()
    // });

    // let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    // let shared_state_clone = shared_state.clone();
    // // let global_version_clone = global_version.clone();
    // tokio::task::spawn(async move {
    //     spawn_state_publisher(arc_node_clone, &shared_state_clone)
    //         .await
    //         .unwrap()
    // });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    r2r::log_info!(NODE_ID, "Spawning operation planner...");

    let model_clone = model.clone();
    let tx_clone = tx.clone();
    tokio::task::spawn(async move {
        planner_ticker(&model_clone, tx_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    r2r::log_info!(NODE_ID, "Spawning auto transition runner...");

    let model_clone = model.clone();
    let tx_clone = tx.clone();
    tokio::task::spawn(async move {
        auto_transition_runner(&model_clone.name, &model_clone, tx_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    r2r::log_info!(NODE_ID, "Spawning auto operation runner...");

    let model_clone = model.clone();
    let tx_clone = tx.clone();
    tokio::task::spawn(async move {
        auto_operation_runner(&model_clone.name, &model_clone, tx_clone)
            .await
            .unwrap()
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    r2r::log_info!(NODE_ID, "Spawning operation runner...");

    let tx_clone = tx.clone();
    tokio::task::spawn(async move { operation_runner(&model, tx_clone).await.unwrap() });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    r2r::log_info!(NODE_ID, "Spawning test generator...");

    // let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    let tx_clone = tx.clone();
    tokio::task::spawn(async move { perform_test(&name, tx_clone).await.unwrap() });

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
    name: &str,
    command_sender: mpsc::Sender<Command>,
) -> Result<(), Box<dyn Error>> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    r2r::log_warn!(NODE_ID, "Starting tests...");
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    r2r::log_warn!(NODE_ID, "Tests started.");
    let mut interval = interval(Duration::from_millis(TEST_TICKER_RATE));
    let mut test_nr = 0;
    let mut goals = vec!["var:gantry_position_estimated == b"];

    'test_loop: loop {
        let (response_tx, response_rx) = oneshot::channel();
        command_sender.send(Command::GetState(response_tx)).await?; // TODO: maybe we can just ask for values from the guard
        let state = response_rx.await?;

        let plan_state = state
            .get_or_default_string(&format!("{}_tester", name), &format!("{}_plan_state", name));

        if goals.len() != 0 {
            // println!("{:?}", goals);
            
            if PlanState::from_str(&plan_state) == PlanState::Failed
                || PlanState::from_str(&plan_state) == PlanState::Completed
                || PlanState::from_str(&plan_state) == PlanState::UNKNOWN
            {
                test_nr = test_nr + 1;
                r2r::log_warn!(NODE_ID, "Starting test {}.", test_nr);
                let new_state = state
                    // .update("gantry_emulate_execution_time", 2.to_spvalue())
                    // .update("gantry_emulated_execution_time", 3000.to_spvalue())
                    // .update("gantry_emulate_failure_rate", 2.to_spvalue())
                    // .update("gantry_emulated_failure_rate", 30.to_spvalue())
                    // .update("gantry_emulate_failure_cause", 2.to_spvalue())
                    // .update(
                    //     "gantry_emulated_failure_cause",
                    //     vec!["violation", "collision", "detected_drift"].to_spvalue(),
                    // )
                    // .update(
                    //     "robot_emulated_failure_cause",
                    //     vec!["move_outside_work_area", "collision_with_operator"].to_spvalue(),
                    // )
                    .update("minimal_model_goal", goals.remove(0).to_spvalue())
                    .update("minimal_model_replan_trigger", true.to_spvalue())
                    .update("minimal_model_replanned", false.to_spvalue());

                    let modified_state = state.get_diff_partial_state(&new_state);
                    command_sender
                        .send(Command::SetPartialState(modified_state))
                        .await?;
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            }
        } else {
            break 'test_loop;
        }
        interval.tick().await;
    }

    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    r2r::log_warn!(NODE_ID, "All tests are finished. Generating report...");

    // Measure operation and plan execution times, and measure total failure rates...
    // Print out plan done or plan failed when done or failed.

    Ok(())
}
