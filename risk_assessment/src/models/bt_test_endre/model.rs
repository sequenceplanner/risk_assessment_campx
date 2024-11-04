use micro_sp::*;
// use crate::*;

// Operations that we need:
// Gantry: move, calibrate, lock, unlock
// Scanner: scan item
// Camera System: update blue boxes
// Robot: move, mount, unmount, pick, place



pub fn bt_test_endre(name: &str, state: &State) -> (Model, State) {
    let state = state.clone();
    let mut auto_transitions = vec![];
    let mut auto_operations = vec![];
    let mut operations = vec![];

    operations.push(Operation::new(
        "op_gantry_lock",
        None,
        Some(3),
        Vec::from([
            Transition::parse(
            "start_op_gantry_lock",
            "var:gantry_request_state == initial \
                && var:gantry_request_trigger == false",
            "true",
            vec![
                &format!("var:gantry_command_command <- lock"),
                "var:gantry_request_trigger <- true",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([Transition::parse(
            "complete_op_gantry_lock",
            "true",
            "var:gantry_request_state == succeeded",
            vec![
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- true",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([Transition::parse(
            "fail_op_gantry_lock",
            "true",
            "var:gantry_request_state == failed",
            vec![
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- UNKNOWN",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([]),
        Vec::from([])
    ));

    operations.push(Operation::new(
        "op_gantry_unlock",
        None,
        Some(3),
        Vec::from([Transition::parse(
            "start_op_gantry_unlock",
            "var:gantry_request_state == initial \
                && var:gantry_request_trigger == false",
            "true",
            vec![
                &format!("var:gantry_command_command <- unlock"),
                "var:gantry_request_trigger <- true",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([Transition::parse(
            "complete_op_gantry_unlock",
            "true",
            "var:gantry_request_state == succeeded",
            vec![
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- false",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([Transition::parse(
            "fail_op_gantry_unlock",
            "true",
            "var:gantry_request_state == failed",
            vec![
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_locked_estimated <- UNKNOWN",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([]),
        Vec::from([])
    ));

    operations.push(Operation::new(
        "op_gantry_calibrate",
        None,
        Some(3),
        Vec::from([Transition::parse(
            "start_op_gantry_calibrate",
            "var:gantry_locked_estimated == false \
                && var:gantry_request_state == initial \
                && var:gantry_request_trigger == false",
            "true",
            vec![
                &format!("var:gantry_command_command <- calibrate"),
                "var:gantry_request_trigger <- true",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([Transition::parse(
            "complete_op_gantry_calibrate",
            "true",
            "var:gantry_request_state == succeeded",
            vec![
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_calibrated_estimated <- true",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([Transition::parse(
            "fail_op_gantry_calibrate",
            "true",
            "var:gantry_request_state == failed",
            vec![
                "var:gantry_request_trigger <- false",
                "var:gantry_request_state <- initial",
                "var:gantry_calibrated_estimated <- UNKNOWN",
            ],
            Vec::<&str>::new(),
            &state,
        )]),
        Vec::from([]),
        Vec::from([])
    ));

    for pos in vec!["a", "b", "c", "d", "pipe_blue_box", "plate_pipe_box"] {
        operations.push(Operation::new(
            &format!("op_gantry_move_to_{}", pos),
            None,
            Some(3),
            Vec::from([Transition::parse(
                &format!("start_op_gantry_move_to_{}", pos),
                "var:gantry_request_state == initial \
                    && var:gantry_request_trigger == false \
                    && var:gantry_locked_estimated == false \
                    && var:gantry_calibrated_estimated == true",
                "true",
                vec![
                    &format!("var:gantry_command_command <- move"),
                    &format!("var:gantry_position_command <- {pos}"),
                    &format!("var:gantry_speed_command <- 0.5"),
                    "var:gantry_request_trigger <- true",
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("complete_op_gantry_move_to_{}", pos),
                "true",
                &format!("var:gantry_request_state == succeeded"),
                vec![
                    "var:gantry_request_trigger <- false",
                    "var:gantry_request_state <- initial",
                    &format!("var:gantry_position_estimated <- {pos}"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("fail_op_gantry_move_to_{}", pos),
                "true",
                "var:gantry_request_state == failed",
                vec![
                    "var:gantry_request_trigger <- false",
                    "var:gantry_request_state <- initial",
                    "var:gantry_position_estimated <- UNKNOWN",
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([]),
            Vec::from([])
        ));
    }

    // for blue_box in vec!["pipe_blue_box", "plate_blue_box"] {
    //     operations.push(Operation::new(
    //         &format!("op_update_position_for_{}", blue_box),
    //         None,
    //         Some(3),
    //         Vec::from([Transition::parse(
    //             &format!("start_op_update_position_for_{}", blue_box),
    //             "var:camera_system_request_state == initial \
    //                 && var:camera_system_request_trigger == false",
    //             "true",
    //             vec![
    //                 &format!("var:camera_system_update_command <- {blue_box}"),
    //                 "var:camera_system_request_trigger <- true",
    //             ],
    //             Vec::<&str>::new(),
    //             &state,
    //         )),
    //         Vec::from([Transition::parse(
    //             &format!("complete_op_update_position_for_{}", blue_box),
    //             "true",
    //             &format!("var:camera_system_request_state == succeeded"),
    //             vec![
    //                 "var:camera_system_request_trigger <- false",
    //                 "var:camera_system_request_state <- initial",
    //                 &format!("var:{blue_box}_position_updated_estimated <- true"),
    //             ],
    //             Vec::<&str>::new(),
    //             &state,
    //         )),
    //         Vec::from([Transition::parse(
    //             &format!("fail_op_update_position_for_{}", blue_box),
    //             "true",
    //             &format!("var:camera_system_request_state == failed"),
    //             vec![
    //                 "var:camera_system_request_trigger <- false",
    //                 "var:camera_system_request_state <- initial",
    //                 &format!("var:{blue_box}_position_updated_estimated <- UNKNOWN"),
    //             ],
    //             Vec::<&str>::new(),
    //             &state,
    //         )),
    //         Transition::empty(),
    //     ));
    // }

    // for item in vec!["pipe", "plate"] {
    //     operations.push(Operation::new(
    //         &format!("op_scan_{}_blue_box", item),
    //         None,
    //         Some(3),
    //         Vec::from([Transition::parse(
    //             &format!("start_op_scan_{}_blue_box", item),
    //             &&format!(
    //                 "var:scanner_request_state == initial \
    //                 && var:scanner_request_trigger == false \
    //                 && var:gantry_position_estimated == {item}_blue_box \
    //                 && var:robot_position_estimated == {item}_blue_box"
    //             ),
    //             "true",
    //             vec![
    //                 &format!("var:scanner_item_command <- {item}"),
    //                 "var:scanner_request_trigger <- true",
    //             ],
    //             Vec::<&str>::new(),
    //             &state,
    //         ),
    //         Vec::from([Transition::parse(
    //             &format!("complete_op_scan_{}_blue_box", item),
    //             "true",
    //             &format!("var:scanner_request_state == succeeded"),
    //             vec![
    //                 "var:scanner_request_trigger <- false",
    //                 "var:scanner_request_state <- initial",
    //                 &format!("var:{item}_blue_box_scanned_estimated <- true"),
    //                 &format!("var:{item}_position_estimated <- {item}_blue_box"),
    //             ],
    //             Vec::<&str>::new(),
    //             &state,
    //         ),
    //         Vec::from([Transition::parse(
    //             &format!("fail_op_scan_{}_blue_box", item),
    //             "true",
    //             &format!("var:scanner_request_state == failed"),
    //             vec![
    //                 "var:scanner_request_trigger <- false",
    //                 "var:scanner_request_state <- initial",
    //                 &format!("var:{item}_blue_box_scanned_estimated <- UNKNOWN"),
    //                 &format!("var:{item}_position_estimated <- UNKNOWN"),
    //             ],
    //             Vec::<&str>::new(),
    //             &state,
    //         ),
    //         Transition::empty(),
    //     ));
    // }

                    //     && var:gantry_locked_estimated == true \
                    // && var:gantry_calibrated_estimated == true",

    for pos in vec![
        "a",
        "b",
        "c",
        "d",
        "pipe_blue_box",
        "plate_pipe_box",
        "gripper_tool_rack",
        "suction_tool_rack",
    ] {
        operations.push(Operation::new(
            &format!("op_robot_move_to_{}", pos),
            None,
            Some(3),
            Vec::from([Transition::parse(
                &format!("start_op_robot_move_to_{}", pos),
                "var:robot_request_state == initial \
                && var:robot_request_trigger == false \
                && var:gantry_locked_estimated == true \
                && var:gantry_calibrated_estimated == true",
                "true",
                vec![
                    &format!("var:robot_command_command <- move"),
                    &format!("var:robot_position_command <- {pos}"),
                    &format!("var:robot_speed_command <- 0.5"),
                    "var:robot_request_trigger <- true",
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("complete_op_robot_move_to_{}", pos),
                "true",
                &format!("var:robot_request_state == succeeded"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_position_estimated <- {pos}"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("fail_op_robot_move_to_{}", pos),
                "true",
                &format!("var:robot_request_state == failed"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_position_estimated <- UNKNOWN"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([]),
            Vec::from([])
        ));
    }

    for tool in vec!["gripper_tool", "suction_tool", "none", "unknown"] {
        operations.push(Operation::new(
            &format!("op_robot_check_for_{tool}_mounted"),
            None,
            Some(3),
            Vec::from([Transition::parse(
                &format!("start_op_robot_check_for_{tool}_mounted"),
                &format!(
                    "(var:robot_mounted_checked == false || var:robot_mounted_checked == UNKNOWN) \
                    && var:robot_request_state == initial \
                    && var:robot_request_trigger == false \
                    && var:robot_mounted_estimated == UNKNOWN"
                ),
                "true",
                vec![
                    &format!("var:robot_command_command <- check_mounted_tool"),
                    "var:robot_request_trigger <- true",
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([
                Transition::parse(
                    &format!("complete_op_robot_check_for_{tool}_mounted"),
                    "true",
                    &format!("var:robot_request_state == succeeded && var:robot_mounted_one_time_measured == {tool}"),
                    vec![
                        "var:robot_request_trigger <- false",
                        "var:robot_request_state <- initial",
                        "var:robot_mounted_checked <- true",
                        &format!("var:robot_mounted_estimated <- {tool}")
                    ],
                    Vec::<&str>::new(),
                    &state,
                ),
                Transition::parse(
                    &format!("complete_op_robot_check_for_{tool}_mounted_2"),
                    "true",
                    &format!("var:robot_request_state == succeeded && var:robot_mounted_one_time_measured != {tool}"),
                    vec![
                        "var:robot_request_trigger <- false",
                        "var:robot_request_state <- initial",
                        "var:robot_mounted_checked <- true",
                        &format!("var:robot_mounted_estimated <- var:robot_mounted_one_time_measured"),
                        &format!("var:{name}_replan_trigger <- true"),
                        &format!("var:{name}_replanned <- false"),
                    ],
                    Vec::<&str>::new(),
                    &state,
                )
            ]),
            Vec::from([Transition::parse(
                &format!("fail_op_robot_check_mounted"),
                "true",
                &format!("var:robot_request_state == failed"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_mounted_estimated <- UNKNOWN"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([]),
            Vec::from([])
        ));
    }

    for tool in vec!["gripper_tool", "suction_tool"] {
        operations.push(Operation::new(
            &format!("op_robot_mount_{}", tool),
            None,
            Some(3),
            Vec::from([Transition::parse(
                &format!("start_op_robot_mount_{}", tool),
                &format!(
                    "var:robot_request_state == initial \
                    && var:robot_request_trigger == false \
                    && var:robot_position_estimated == {tool}_rack \
                    && var:robot_mounted_estimated == none \
                    && var:gantry_locked_estimated == true \
                    && var:robot_mounted_checked == true",
                ),
                "true",
                vec![
                    &format!("var:robot_command_command <- mount"),
                    "var:robot_request_trigger <- true",
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("complete_op_robot_mount_{}", tool),
                "true",
                &format!("var:robot_request_state == succeeded"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_mounted_estimated <- {tool}"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("fail_op_robot_mount_{}", tool),
                "true",
                &format!("var:robot_request_state == failed"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_mounted_estimated <- UNKNOWN"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([]),
            Vec::from([])
        ));
    }

    for tool in vec!["gripper_tool", "suction_tool"] {
        operations.push(Operation::new(
            &format!("op_robot_unmount_{tool}"),
            None,
            Some(3),
            Vec::from([Transition::parse(
                &format!("start_op_robot_unmount_{tool}"),
                &format!(
                    "var:robot_request_state == initial \
                    && var:robot_request_trigger == false \
                    && var:robot_position_estimated == {tool}_rack \
                    && var:robot_mounted_estimated == {tool} \
                    && var:gantry_locked_estimated == true"
                ),
                "true",
                vec![
                    &format!("var:robot_command_command <- unmount"),
                    "var:robot_request_trigger <- true",
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("complete_op_robot_unmount_{tool}"),
                "true",
                &format!("var:robot_request_state == succeeded"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_mounted_estimated <- none"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([Transition::parse(
                &format!("fail_op_robot_unmount_{tool}"),
                "true",
                &format!("var:robot_request_state == failed"),
                vec![
                    "var:robot_request_trigger <- false",
                    "var:robot_request_state <- initial",
                    &format!("var:robot_mounted_estimated <- UNKNOWN"),
                ],
                Vec::<&str>::new(),
                &state,
            )]),
            Vec::from([]),
            Vec::from([])
        ));
    }

    // auto_operations.push(Operation::new(
    //     &format!("op_robot_check_mounted"),
    //     None,
    //     Some(3),
    //     Vec::from([Transition::parse(
    //         &format!("start_op_robot_check_mounted"),
    //         &format!(
    //             "var:robot_request_state == initial \
    //             && var:robot_request_trigger == false \
    //             && var:robot_mounted_estimated == UNKNOWN"
    //         ),
    //         "true",
    //         vec![
    //             &format!("var:robot_command_command <- check_mounted_tool"),
    //             "var:robot_request_trigger <- true",
    //         ],
    //         Vec::<&str>::new(),
    //         &state,
    //     ),
    //     Vec::from([Transition::parse(
    //         &format!("complete_op_robot_check_mounted"),
    //         "true",
    //         &format!("var:robot_request_state == succeeded"),
    //         vec![
    //             "var:robot_request_trigger <- false",
    //             "var:robot_request_state <- initial",
    //             &format!("var:robot_mounted_estimated <- var:robot_mounted_one_time_measured"),
    //         ],
    //         Vec::<&str>::new(),
    //         &state,
    //     ),
    //     Vec::from([Transition::parse(
    //         &format!("fail_op_robot_check_mounted"),
    //         "true",
    //         &format!("var:robot_request_state == failed"),
    //         vec![
    //             "var:robot_request_trigger <- false",
    //             "var:robot_request_state <- initial",
    //             &format!("var:robot_mounted_estimated <- UNKNOWN"),
    //         ],
    //         Vec::<&str>::new(),
    //         &state,
    //     ),
    //     Transition::empty(),
    // ));


    // // reason enough to have automatic operations?
    // auto_transitions.push(Vec::from([Transition::parse(
    //     "start_robot_check_mounted_tool",
    //     "true",
    //     "var:robot_mounted_estimated == UNKNOWN \
    //         && var:robot_request_state == initial \
    //         && var:robot_request_trigger == false",
    //     Vec::<&str>::new(),
    //     Vec::from([
    //         "var:robot_request_trigger <- true",
    //         "var:robot_command_command <- check_mounted_tool", // non-deterministic outcome
    //     ),
    //     &state
    // ));

    // auto_transitions.push(Vec::from([Transition::parse(
    //     "complete_robot_check_mounted_tool",
    //     "true",
    //     "var:robot_mounted_estimated == UNKNOWN \
    //         && var:robot_check_mounted_tool == true \
    //         && var:robot_request_state == initial \
    //         && var:robot_request_trigger == false",
    //     Vec::<&str>::new(),
    //     Vec::from([
    //         "var:robot_request_state <- initial",
    //         "var:robot_request_trigger <- true",
    //         "var:robot_command_command <- check_mounted_tool", // non-deterministic outcome
    //     ),
    //     &state
    // ));

    let model = Model::new(name, auto_transitions, auto_operations, operations);

    (model, state)
}

#[test]
fn test_model() {

    let state = crate::models::bt_test_endre::state::state();

    // Add the variables that keep track of the runner state
    let runner_vars = generate_runner_state_variables("bt_test_endre");
    let state = state.extend(runner_vars, true);

    let (model, state) = crate::models::bt_test_endre::model::bt_test_endre("bt_test_endre", &state);
    let name = model.clone().name;

    let op_vars = generate_operation_state_variables(&model, false);
    let state = state.extend(op_vars, true);

    for s in &state.state {
        println!("{:?}", s.1);
    }

    let goal = state.get_value(&format!("{}_goal", model.name));
    let val = state.get_value("gantry_position_estimated");
    println!("Current goal: {:?}", goal);
    println!("Current value: {:?}", val);

    let state = state.update(
        &format!("{}_goal", model.name),
        "var:robot_position_estimated == b".to_spvalue(),
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
