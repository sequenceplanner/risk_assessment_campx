use micro_sp::*;
// use crate::*;

pub fn minimal_model(name: &str, state: &State) -> (Model, State) {
    let state = state.clone();
    let mut operations = vec![];
    let mut auto_operations = vec!();
    let mut auto_transitions = vec![];

    operations.push(Operation::new(
        // name
        "op_gantry_lock",
        // deadline
        None,
        Some(3),
        // precondition
        t!(
            // name
            "start_op_gantry_lock",
            // planner guard
            "var:gantry_request_state == initial \
            && var:gantry_request_trigger == false",
            // runner guard
            "true",
            // planner actions
            vec!(
                format!("var:gantry_command_command <- lock").as_str(),
                "var:gantry_request_trigger <- true"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        // postcondition
        t!(
            // name
            "complete_op_gantry_lock",
            // planner guard
            "true",
            // runner guard
            "var:gantry_request_state == succeeded",
            // planner actions
            vec!(
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- true"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        // reset transition
        t!(
            // name
            "fail_op_gantry_lock",
            // planner guard
            "true",
            // runner guard
            "var:gantry_request_state == failed",
            // planner actions
            vec!(
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- UNKNOWN"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        Transition::empty(),
    ));

    operations.push(Operation::new(
        // name
        "op_gantry_unlock",
        // deadline
        None,
        Some(3),
        // precondition
        t!(
            // name
            "start_op_gantry_unlock",
            // planner guard
            "var:gantry_request_state == initial \
            && var:gantry_request_trigger == false",
            // runner guard
            "true",
            // planner actions
            vec!(
                format!("var:gantry_command_command <- unlock").as_str(),
                "var:gantry_request_trigger <- true"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        // postcondition
        t!(
            // name
            "complete_op_gantry_unlock",
            // planner guard
            "true",
            // runner guard
            "var:gantry_request_state == succeeded",
            // planner actions
            vec!(
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- false"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        // reset transition
        t!(
            // name
            "fail_op_gantry_unlock",
            // planner guard
            "true",
            // runner guard
            "var:gantry_request_state == failed",
            // planner actions
            vec!(
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- UNKNOWN"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        Transition::empty(),
    ));

    operations.push(Operation::new(
        // name
        "op_gantry_calibrate",
        // deadline
        None,
        Some(3),
        // precondition
        t!(
            // name
            "start_op_gantry_calibrate",
            // planner guard
            "var:gantry_request_state == initial \
            && var:gantry_request_trigger == false",
            // runner guard
            "true",
            // planner actions
            vec!(
                format!("var:gantry_command_command <- calibrate").as_str(),
                "var:gantry_request_trigger <- true"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        // postcondition
        t!(
            // name
            "complete_op_gantry_calibrate",
            // planner guard
            "true",
            // runner guard
            "var:gantry_request_state == succeeded",
            // planner actions
            vec!(
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_calibrated_estimated <- true"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        // reset transition
        t!(
            // name
            "fail_op_gantry_calibrate",
            // planner guard
            "true",
            // runner guard
            "var:gantry_request_state == failed",
            // planner actions
            vec!(
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_calibrated_estimated <- UNKNOWN"
            ),
            //runner actions
            Vec::<&str>::new(),
            &state
        ),
        Transition::empty(),
    ));

    for pos in vec!["a", "b", "c", "d"] {
        operations.push(Operation::new(
            // name
            &format!("op_gantry_move_to_{}", pos),
            // deadline
            None,
            Some(3),
            // precondition
            t!(
                // name
                &format!("start_op_gantry_move_to_{}", pos).as_str(),
                // planner guard
                "var:gantry_request_state == initial \
                && var:gantry_request_trigger == false \
                && var:gantry_locked_estimated == false \
                && var:gantry_calibrated_estimated == true",
                // runner guard
                "true",
                // planner actions
                vec!(
                    format!("var:gantry_command_command <- move").as_str(),
                    format!("var:gantry_position_command <- {pos}").as_str(),
                    format!("var:gantry_speed_command <- 0.5").as_str(),
                    "var:gantry_request_trigger <- true"
                ),
                //runner actions
                Vec::<&str>::new(),
                &state
            ),
            // postcondition
            t!(
                // name
                &format!("complete_op_gantry_move_to_{}", pos).as_str(),
                // planner guard
                "true",
                // runner guard
                &format!("var:gantry_request_state == succeeded").as_str(),
                // planner actions
                vec!(
                    "var:gantry_request_trigger <- false",
                    "var:gantry_request_state <- initial",
                    &format!("var:gantry_position_estimated <- {pos}")
                ),
                //runner actions
                Vec::<&str>::new(),
                &state
            ),
            // reset transition
            t!(
                // name
                &format!("fail_op_gantry_move_to_{}", pos).as_str(),
                // planner guard
                "true",
                // runner guard
                "var:gantry_request_state == failed",
                // planner actions
                vec!(
                    "var:gantry_request_trigger <- false",
                    "var:gantry_request_state <- initial",
                    "var:gantry_position_estimated <- UNKNOWN"
                ),
                //runner actions
                Vec::<&str>::new(),
                &state
            ),
            Transition::empty(),
        ));
    }

    auto_transitions.push(t!(
        // name
        "replan_if_plan_has_failed",
        // planner guard
        &format!("var:{}_replan_fail_counter == 1", name).as_str(),
        // ruuner guard = none
        "true",
        // planner actions
        Vec::<&str>::new(),
        // runner actions - none
        vec!(
            "var:gantry_request_state <- initial",
            "var:gantry_request_trigger <- false",
            "var:minimal_model_plan <- UNKNOWN",
            "var:minimal_model_plan_current_step <- UNKNOWN",
            "var:minimal_model_replan_trigger <- true" // "var:runner_replan_trigger <- true"
        ),
        &state
    ));

    let model = Model::new(name, auto_transitions, auto_operations, operations);

    (model, state)
}

#[test]
fn test_model() {
    let state = crate::models::minimal::state::state();

    for s in &state.state {
        println!("{:?}", s.1);
    }

    let (model, state) = minimal_model("minimal_model", &state);

    println!("+++++++++++++++++++++++");

    for s in &state.state {
        println!("{:?}", s.1);
    }

    let goal = state.get_value(&format!("{}_goal", model.name));
    let val = state.get_value("gantry_position_estimated");
    println!("Current goal: {:?}", goal);
    println!("Current value: {:?}", val);

    let state = state.update(
        &format!("{}_goal", model.name),
        "var:gantry_position_estimated == b".to_spvalue(),
    );
    let goal = state.get_value(&format!("{}_goal", model.name));
    let val = state.get_value("gantry_position_estimated");
    println!("Current goal: {:?}", goal);
    println!("Current value: {:?}", val);

    // let extracted_goal = extract_goal_from_state(name, state)

    let plan = bfs_operation_planner(
        state.clone(),
        state.extract_goal(&model.name),
        model.operations.clone(),
        30,
    );

    let val = state.get_value("gantry_position_estimated");
    println!("Current goal: {:?}", goal);
    println!("Current value: {:?}", val);

    println!("{:?}", plan);

    assert!(plan.found);
}
