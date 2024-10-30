use micro_sp::{State, SPValue};
use r2r::{std_msgs::msg::String as StringMsg, QosProfile};
use serde_json::Value;
use std::sync::{atomic::AtomicUsize, Arc, Mutex};

use crate::*;

pub async fn spawn_state_publisher(
    arc_node: Arc<Mutex<r2r::Node>>,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>,//HashMap<String, AtomicUsize>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let publisher = arc_node
        .lock()
        .unwrap()
        .create_publisher::<StringMsg>(
            "/state",
            QosProfile::default(),
        )?;

    let timer = arc_node
        .lock()
        .unwrap()
        .create_wall_timer(std::time::Duration::from_millis(PUBLISHER_TICKER_RATE))?;

    let shared_state_clone = shared_state.clone();
    tokio::task::spawn(async move {
        match state_publisher(publisher, timer, &shared_state_clone).await {
            Ok(()) => r2r::log_info!("state_publisher", "Succeeded."),
            Err(e) => r2r::log_error!("state_publisher", "Failed with: '{}'.", e),
        };
    });
    Ok(())
}

pub async fn state_publisher(
    publisher: r2r::Publisher<StringMsg>,
    mut timer: r2r::Timer,
    shared_state: &Arc<(Mutex<State>, Vec<AtomicUsize>)>,//HashMap<String, AtomicUsize>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let shsl = shared_state.0.lock().unwrap().clone();
        let mut map = serde_json::Map::new();
        shsl.state.iter().for_each(|(k, v)| {
            let _ = map.insert(k.to_string(), match &v.val {
                SPValue::Array(_, val) => Value::from(format!("{:?}", val)),
                SPValue::Bool(val) => Value::from(format!("{}", val)),
                SPValue::Float64(val) => Value::from(format!("{}", val)),
                SPValue::String(val) => Value::from(format!("{}", val)),
                SPValue::Int64(val) => Value::from(format!("{}", val)),
                SPValue::Time(val) => Value::from(format!("{:?}", val)),
                SPValue::UNKNOWN => Value::from(format!("UNKNOWN")),
            });
        });

        let state = serde_json::Value::Object(map).to_string();
        let state_msg = StringMsg {
            data: state
        };

        match publisher.publish(&state_msg) {
            Ok(()) => (),
            Err(e) => {
                r2r::log_error!("state_publisher", "Failed to send a message with: '{}'", e);
            }
        };
        timer.tick().await?;
    }
}