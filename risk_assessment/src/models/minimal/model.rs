use micro_sp::*;
// use crate::*;

pub fn minimal_model(state: &State) -> (Model, State) {
    let state = state.clone();
    let mut operations = vec![];
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
        &state),
        Transition::empty()
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
        &state),
        Transition::empty()
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
        &state),
        Transition::empty()
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
            &state),
            Transition::empty()
        ));
    }

    // let taken_auto_replan_if_gantry_failed = iv!("taken_auto_replan_if_gantry_failed");
    // let state = state.add(assign!(taken_auto_replan_if_gantry_failed, 0.to_spvalue()));
    // auto_transitions.push(t!(
    //     // name
    //     "replan_if_gantry_failed",
    //     // planner guard
    //     "var:gantry_request_state == failed",
    //     // ruuner guard = none
    //     "true",
    //     // planner actions
    //     Vec::<&str>::new(),
    //     // runner actions - none
    //     vec!(
    //         "var:gantry_request_state <- initial",
    //         "var:gantry_request_trigger <- false",
    //         "var:minimal_model_plan <- UNKNOWN",
    //         "var:minimal_model_plan_current_step <- UNKNOWN",
    //         "var:minimal_model_replan_trigger <- true" // "var:runner_replan_trigger <- true"
    //     ),
    //     &state
    // ));

    // let taken_auto_replan_if_scanner_failed = iv_runner!("taken_auto_replan_if_scanner_failed");
    // let state = state.add(assign!(taken_auto_replan_if_scanner_failed, 0.to_spvalue()));
    // auto_transitions.push(t!(
    //     // name
    //     "replan_if_scanner_failed",
    //     // planner guard
    //     "var:scanner_request_state == failed",
    //     // ruuner guard = none
    //     "true",
    //     // planner actions
    //     Vec::<&str>::new(),
    //     // runner actions - none
    //     vec!(
    //         "var:scanner_request_state <- initial",
    //         "var:scanner_request_trigger <- false",
    //         "var:runner_plan <- [unknown]",
    //         "var:runner_plan_current_step <- [unknown]",
    //         "var:runner_plan_info <- Waiting_for_the_re_plan",
    //         "var:runner_replan <- true" // "var:runner_replan_trigger <- true"
    //     ),
    //     &state
    // ));

    // let taken_auto_replan_if_gripper_failed = iv_runner!("taken_auto_replan_if_gripper_failed");
    // let state = state.add(assign!(taken_auto_replan_if_gripper_failed, 0.to_spvalue()));
    // auto_transitions.push(t!(
    //     // name
    //     "replan_if_gripper_failed",
    //     // planner guard
    //     "var:gripper_request_state == failed",
    //     // ruuner guard = none
    //     "true",
    //     // planner actions
    //     Vec::<&str>::new(),
    //     // runner actions - none
    //     vec!(
    //         "var:gripper_request_state <- initial",
    //         "var:gripper_request_trigger <- false",
    //         "var:runner_plan <- [unknown]",
    //         "var:runner_plan_current_step <- [unknown]",
    //         "var:runner_plan_info <- Waiting_for_the_re_plan",
    //         "var:runner_replan <- true" // "var:runner_replan_trigger <- true"
    //     ),
    //     &state
    // ));

    // let taken_auto_replan_if_gripper_cant_completely_close =
    //     iv_runner!("taken_auto_replan_if_gripper_cant_completely_close");
    // let state = state.add(assign!(
    //     taken_auto_replan_if_gripper_cant_completely_close,
    //     0.to_spvalue()
    // ));
    // auto_transitions.push(t!(
    //     // name
    //     "replan_if_gripper_cant_completely_close",
    //     // planner guard
    //     "var:op_close_gripper == executing && var:gripper_actual_state == gripping",
    //     // ruuner guard = none
    //     "true",
    //     // planner actions
    //     Vec::<&str>::new(),
    //     // runner actions - none
    //     vec!(
    //         "var:gripper_request_state <- initial",
    //         "var:gripper_request_trigger <- false",
    //         "var:runner_plan <- [unknown]",
    //         "var:runner_plan_current_step <- [unknown]",
    //         "var:runner_plan_info <- Waiting_for_the_re_plan",
    //         "var:runner_replan <- true" // "var:runner_replan_trigger <- true"
    //     ),
    //     &state
    // ));

    // let taken_auto_abort_planning_if_scanning_timedout_5_times =
    //     iv_runner!("taken_auto_abort_if_scanning_timedout_5_times");
    // let state = state.add(assign!(
    //     taken_auto_abort_planning_if_scanning_timedout_5_times,
    //     0.to_spvalue()
    // ));
    // auto_transitions.push(t!(
    //     // name
    //     "abort_if_scanning_timedout_5_times",
    //     // planner guard
    //     "var:timedout_op_scan_box_a == 5 && var:runner_plan_state != aborted",
    //     // ruuner guard = none
    //     "true",
    //     // planner actions
    //     Vec::<&str>::new(),
    //     // runner actions - none
    //     vec!(
    //         "var:runner_plan <- [unknown]",
    //         "var:runner_plan_current_step <- [unknown]",
    //         "var:runner_plan_info <- Aborted_due_to_timeout",
    //         "var:runner_plan_state <- aborted",
    //         "var:runner_replan <- false",
    //         "var:runner_replanned <- false",
    //         "var:timedout_op_scan_box_a <- 1" // "var:runner_replan_trigger <- true"
    //     ),
    //     &state
    // ));

    

    // let taken_auto_replan_if_robot_failed = iv_runner!("taken_auto_replan_if_robot_failed");
    // let state = state.add(assign!(taken_auto_replan_if_robot_failed, 0.to_spvalue()));
    // auto_transitions.push(t!(
    //     // name
    //     "replan_if_robot_failed",
    //     // planner guard
    //     "var:robot_request_state == failed",
    //     // ruuner guard = none
    //     "true",
    //     // planner actions
    //     Vec::<&str>::new(),
    //     // runner actions - none
    //     vec!(
    //         "var:robot_request_state <- initial",
    //         "var:robot_request_trigger <- false",
    //         "var:runner_plan <- [unknown]",
    //         "var:runner_plan_current_step <- [unknown]",
    //         "var:runner_plan_info <- Waiting_for_the_re_plan",
    //         "var:runner_replan <- true" // "var:runner_replan_trigger <- true"
    //     ),
    //     &state

    let model = Model::new("minimal_model", auto_transitions, operations);

    // let runner_state = generate_runner_state_variables(&model, &model.name, true);

    (model, state)
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
