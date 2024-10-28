use micro_sp::*;

fn generate_basic_variables(name: &str, state: &State) -> State {
    let request_trigger = bv!(&&format!("{}_request_trigger", name));
    let request_state = v!(&&format!("{}_request_state", name));
    let total_fail_counter = iv!(&&format!("{}_total_fail_counter", name));
    let subsequent_fail_counter = iv!(&&format!("{}_subsequent_fail_counter", name));
    let ref_counter = iv!(&&format!("{}_ref_counter", name));

    let state = state.add(assign!(request_trigger, false.to_spvalue()));
    let state = state.add(assign!(request_state, "initial".to_spvalue()));
    let state = state.add(assign!(total_fail_counter, 0.to_spvalue()));
    let state = state.add(assign!(subsequent_fail_counter, 0.to_spvalue()));
    let state = state.add(assign!(ref_counter, 1.to_spvalue()));

    state
}

fn generate_emulation_variables(name: &str, state: &State) -> State {
    // -----------------------------------------------------------------------
    // # DONT_EMULATE_EXECUTION_TIME: The action will be executed immediatelly
    // # EMULATE_EXACT_EXECUTION_TIME: The action will always take "emulate_execution_time" amount of time
    // # EMULATE_RANDOM_EXECUTION_TIME: The action will randomly take between 0 and "emulated_execution_time" amount of time
    // uint8 DONT_EMULATE_EXECUTION_TIME = 0
    // uint8 EMULATE_EXACT_EXECUTION_TIME = 1
    // uint8 EMULATE_RANDOM_EXECUTION_TIME = 2
    // uint8 emulate_execution_time
    // int32 emulated_execution_time # milliseconds 

    // # DONT_EMULATE_FAILURE: The action will be execute succesfully every time
    // # EMULATE_FAILURE_ALWAYS: The action will always fail
    // # EMULATE_RANDOM_FAILURE_RATE: The action will randomly fail with a "emulated_failure_rate" rate
    // uint8 DONT_EMULATE_FAILURE = 0
    // uint8 EMULATE_FAILURE_ALWAYS = 1
    // uint8 EMULATE_RANDOM_FAILURE_RATE = 2
    // uint8 emulate_failure_rate
    // int32 emulated_failure_rate # percentage 0..100  

    // # DONT_EMULATE_FAILURE_CAUSE: If the action fails, it wil fail with a generic "fail" cause
    // # EMULATE_EXACT_FAILURE_CAUSE: Specify why the exact reason why the action fails (takes the first from the "emulated_failure_cause" list)
    // # EMULATE_RANDOM_FAILURE_CAUSE: The action will fail and randomly choose a cause from the "emulated_failure_cause" list
    // uint8 DONT_EMULATE_FAILURE_CAUSE = 0
    // uint8 EMULATE_EXACT_FAILURE_CAUSE = 1
    // uint8 EMULATE_RANDOM_FAILURE_CAUSE = 2
    // uint8 emulate_failure_cause
    // string[] emulated_failure_cause # For example: ["violation", "timeout", "collision", etc.]
    // -----------------------------------------------------------------------

    let emulate_execution_time = iv!(&&format!("{}_emulate_execution_time", name));
    let emulate_failure_rate = iv!(&&format!("{}_emulate_failure_rate", name));
    let emulate_failure_cause = iv!(&&format!("{}_emulate_failure_cause", name));

    let state = state.add(assign!(emulate_execution_time, 0.to_spvalue()));
    let state = state.add(assign!(emulate_failure_rate, 0.to_spvalue()));
    let state = state.add(assign!(emulate_failure_cause, 0.to_spvalue()));

    let emulated_execution_time = iv!(&&format!("{}_emulated_execution_time", name));
    let emulated_failure_rate = iv!(&&format!("{}_emulated_failure_rate", name));
    let emulated_failure_cause = av!(&&format!("{}_emulated_failure_cause", name));

    let state = state.add(assign!(emulated_execution_time, 0.to_spvalue()));
    let state = state.add(assign!(emulated_failure_rate, 0.to_spvalue()));
    let state = state.add(assign!(emulated_failure_cause, SPValue::Array(SPValueType::String, vec![])));

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
    let state = state.add(assign!(gantry_speed_command, 0.0.to_spvalue()));
    let state = state.add(assign!(gantry_position_command, SPValue::UNKNOWN));

    // We estimate (memory variables) the following, since we cannot directly measure
    let gantry_speed_measured = fv!("gantry_speed_estimated");
    let gantry_position_estimated = v!("gantry_position_estimated");
    let gantry_calibrated_estimated = bv!("gantry_calibrated_estimated");
    let gantry_locked_estimated = bv!("gantry_locked_estimated");

    let state = state.add(assign!(gantry_calibrated_estimated, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_locked_estimated, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_speed_measured, SPValue::UNKNOWN));
    let state = state.add(assign!(gantry_position_estimated, SPValue::UNKNOWN));

    // Optional: emulate gantry failure and execution time
    let state = generate_emulation_variables("gantry", &state);
    
    state
}
