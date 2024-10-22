use micro_sp::*;

pub fn scheduler_model(state: &State, coverability_tracking: bool) -> (Model, State) {
    let state = state.clone();
    let mut operations = vec![];
    let auto_transitions = vec![];

    operations.push(Operation::new(
        // name
        "op_gantry_lock",
        // deadline
        None,
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
        Transition::empty(),
    ));

    operations.push(Operation::new(
        // name
        "op_gantry_unlock",
        // deadline
        None,
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
        Transition::empty(),
    ));

    operations.push(Operation::new(
        // name
        "op_gantry_calibrate",
        // deadline
        None,
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
        Transition::empty(),
    ));

    for pos in vec!["a", "b", "c", "d"] {
        operations.push(Operation::new(
            // name
            &format!("op_gantry_move_to_{}", pos),
            // deadline
            None,
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
            Transition::empty(),
        ));
    }

    let model = Model::new("minimal_model", auto_transitions, operations);

    let state = generate_runner_state_variables(&model, &model.name, true);

    (model, state)
}
