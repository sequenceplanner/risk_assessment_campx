use micro_sp::*;
// use crate::*;

pub fn minimal_model(state: &State) -> (Model, State) {
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

    let runner_state = generate_runner_state_variables(&model, &model.name, true);

    (model, runner_state.extend(state, true))
}

#[test]
    fn test_model() {
        let state = crate::models::minimal::state::state();

        for s in &state.state {
            println!("{:?}", s.1);
        }

        let (model, state) = minimal_model(&state);

        println!("+++++++++++++++++++++++");

        for s in &state.state {
            println!("{:?}", s.1);
        }

        let goal = state.get_value(&format!("{}_goal", model.name));
        let val = state.get_value("gantry_position_estimated");
        println!("Current goal: {:?}", goal);
        println!("Current value: {:?}", val);

        let state = state.update(&format!("{}_goal", model.name), "var:gantry_position_estimated == b".to_spvalue());
        let goal = state.get_value(&format!("{}_goal", model.name));
        let val = state.get_value("gantry_position_estimated");
        println!("Current goal: {:?}", goal);
        println!("Current value: {:?}", val);

        // let extracted_goal = extract_goal_from_state(name, state)

        let plan = bfs_operation_planner(
            state.clone(),
            extract_goal_from_state(model.name, &state.clone()),
            model.operations.clone(),
            30,
        );

        let val = state.get_value("gantry_position_estimated");
        println!("Current goal: {:?}", goal);
        println!("Current value: {:?}", val);

        println!("{:?}", plan);

        assert!(plan.found);
    }

// #[cfg(test)]
// mod tests {

//     use minimal::model::minimal_model;
//     use micro_sp::*;
//     use crate::*;

//     #[test]
//     fn test_model() {
//         let state = models::minimal::state::state();

//         let (model, state) = minimal_model(&state);

//         let plan = bfs_operation_planner(
//             state.clone(),
//             extract_goal_from_state(model.name, &state.clone()),
//             model.operations.clone(),
//             30,
//         );
//         println!("ASDFASDFASDF");
//         log::error!("asdf");
//         for p in plan.plan {
//             println!("{}", p);
//             // println!("{}", p);
//         }
//         // log::info!("This is an info message.");

//         assert!(plan.found);
//     }
// }
