use std::error::Error;
use std::sync::{Arc, Mutex};
use risk_assessment::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup the node
    let ctx = r2r::Context::create()?;
    let node = r2r::Node::create(ctx, EMULATOR_NODE_ID, "")?;
    let arc_node = Arc::new(Mutex::new(node));

    r2r::log_info!(EMULATOR_NODE_ID, "Spawning tasks...");

    let arc_node_clone: Arc<Mutex<r2r::Node>> = arc_node.clone();
    tokio::task::spawn(async move {
        gantry_emulator::spawn_gantry_emulator_server(
            arc_node_clone,
        )
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

    r2r::log_info!(EMULATOR_NODE_ID, "Node started.");

    handle.join().unwrap();

    Ok(())
}