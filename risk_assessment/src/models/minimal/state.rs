use micro_sp::*;

fn generate_basic_variables(name: &str, state: &State) -> State {
    let request_trigger = bv!(&&format!("{}_request_trigger", name));
    let request_state = v!(&&format!("{}_request_state", name));
    let fail_counter = iv!(&&format!("{}_fail_counter", name));

    let state = state.add(assign!(request_trigger, false.to_spvalue()));
    let state = state.add(assign!(request_state, "initial".to_spvalue()));
    let state = state.add(assign!(fail_counter, 0.to_spvalue()));

    state
}

pub fn state() -> State {
    let state = State::new();

    // -----------------------------------------------------------------------
    // Gantry:
    // string command # move, calibrate, lock, unlock
    // float32 speed
    // string position
    // -----------------------------------------------------------------------

    let state = generate_basic_variables("gantry", &state);

    let gantry_command_command = v!("gantry_command_command");
    let gantry_speed_command = fv!("gantry_speed_command");
    let gantry_position_command = v!("gantry_position_command");

    let state = state.add(assign!(gantry_command_command, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_speed_command, 0.to_spvalue()));
    let state = state.add(assign!(gantry_position_command, SPValue::UNKNOWN));

    // In this emulation, we estimate (memory variables) the following, since we cannot directly measure
    let gantry_speed_measured = fv!("gantry_speed_estimated");
    let gantry_position_measured = v!("gantry_position_estimated");
    let gantry_calibrated_estimated = bv!("gantry_calibrated_estimated");
    let gantry_locked_estimated = bv!("gantry_locked_estimated");

    let state = state.add(assign!(gantry_calibrated_estimated, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_locked_estimated, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_speed_measured, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_position_measured, SPValue::UNKNOWN));

    state
}
