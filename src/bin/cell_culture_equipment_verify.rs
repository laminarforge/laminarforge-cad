use std::fs;
use std::path::Path;

#[derive(Debug)]
struct ExpectedOutput {
    generator: &'static str,
    path: &'static str,
    min_triangles: u32,
    min_size_mm: [f32; 3],
}

#[derive(Debug)]
struct StlStats {
    triangles: u32,
    bbox_min: [f32; 3],
    bbox_max: [f32; 3],
}

const OUTPUTS: &[ExpectedOutput] = &[
    ExpectedOutput {
        generator: "co2_incubator",
        path: "output/co2_incubator_chamber.stl",
        min_triangles: 100,
        min_size_mm: [250.0, 200.0, 200.0],
    },
    ExpectedOutput {
        generator: "co2_incubator",
        path: "output/co2_incubator_service_manifold.stl",
        min_triangles: 50,
        min_size_mm: [120.0, 5.0, 50.0],
    },
    ExpectedOutput {
        generator: "cell_culture_logger_enclosure",
        path: "output/cell_culture_logger_enclosure_body.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 70.0, 35.0],
    },
    ExpectedOutput {
        generator: "cell_culture_logger_enclosure",
        path: "output/cell_culture_logger_enclosure_lid.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 70.0, 3.0],
    },
    ExpectedOutput {
        generator: "co2_sensor_service_module",
        path: "output/co2_sensor_service_module_base.stl",
        min_triangles: 80,
        min_size_mm: [160.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "co2_sensor_service_module",
        path: "output/co2_sensor_service_module_cover.stl",
        min_triangles: 80,
        min_size_mm: [160.0, 80.0, 35.0],
    },
    ExpectedOutput {
        generator: "water_bath",
        path: "output/water_bath_basin.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 100.0, 40.0],
    },
    ExpectedOutput {
        generator: "water_bath_safety_kit",
        path: "output/water_bath_safety_bottle_rack.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 60.0, 45.0],
    },
    ExpectedOutput {
        generator: "water_bath_safety_kit",
        path: "output/water_bath_safety_spill_tray.stl",
        min_triangles: 20,
        min_size_mm: [200.0, 150.0, 10.0],
    },
    ExpectedOutput {
        generator: "heating_block",
        path: "output/heating_block.stl",
        min_triangles: 50,
        min_size_mm: [50.0, 20.0, 10.0],
    },
    ExpectedOutput {
        generator: "orbital_shaker",
        path: "output/orbital_shaker_base.stl",
        min_triangles: 50,
        min_size_mm: [100.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "rack_rocker",
        path: "output/rack_rocker_base.stl",
        min_triangles: 50,
        min_size_mm: [100.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "media_reservoir",
        path: "output/media_reservoir_body.stl",
        min_triangles: 100,
        min_size_mm: [150.0, 150.0, 200.0],
    },
    ExpectedOutput {
        generator: "chip_priming_tubing_fixture",
        path: "output/chip_priming_fixture_base.stl",
        min_triangles: 80,
        min_size_mm: [160.0, 130.0, 10.0],
    },
    ExpectedOutput {
        generator: "chip_priming_tubing_fixture",
        path: "output/chip_priming_fixture_tubing_comb.stl",
        min_triangles: 40,
        min_size_mm: [110.0, 100.0, 10.0],
    },
    ExpectedOutput {
        generator: "automated_media_exchange_cassette",
        path: "output/automated_media_exchange_cassette_base.stl",
        min_triangles: 80,
        min_size_mm: [550.0, 470.0, 10.0],
    },
    ExpectedOutput {
        generator: "automated_media_exchange_cassette",
        path: "output/automated_media_exchange_cassette_assembly.stl",
        min_triangles: 120,
        min_size_mm: [610.0, 470.0, 30.0],
    },
    ExpectedOutput {
        generator: "automated_seeding_coating_station",
        path: "output/automated_seeding_coating_station_baseplate.stl",
        min_triangles: 300,
        min_size_mm: [1200.0, 850.0, 15.0],
    },
    ExpectedOutput {
        generator: "automated_seeding_coating_station",
        path: "output/automated_seeding_coating_station_cassette_nest.stl",
        min_triangles: 400,
        min_size_mm: [700.0, 600.0, 10.0],
    },
    ExpectedOutput {
        generator: "automated_seeding_coating_station",
        path: "output/automated_seeding_coating_station_valve_manifold.stl",
        min_triangles: 600,
        min_size_mm: [400.0, 140.0, 40.0],
    },
    ExpectedOutput {
        generator: "automated_seeding_coating_station",
        path: "output/automated_seeding_coating_station_assembly.stl",
        min_triangles: 1500,
        min_size_mm: [1200.0, 850.0, 150.0],
    },
    ExpectedOutput {
        generator: "automated_ecm_coating_qc_station",
        path: "output/automated_ecm_coating_qc_station_baseplate.stl",
        min_triangles: 300,
        min_size_mm: [1160.0, 740.0, 15.0],
    },
    ExpectedOutput {
        generator: "automated_ecm_coating_qc_station",
        path: "output/automated_ecm_coating_qc_station_cassette_datum_nest.stl",
        min_triangles: 300,
        min_size_mm: [650.0, 600.0, 15.0],
    },
    ExpectedOutput {
        generator: "automated_ecm_coating_qc_station",
        path: "output/automated_ecm_coating_qc_station_dispense_recirculation_lanes.stl",
        min_triangles: 500,
        min_size_mm: [350.0, 300.0, 35.0],
    },
    ExpectedOutput {
        generator: "automated_ecm_coating_qc_station",
        path: "output/automated_ecm_coating_qc_station_timed_wetness_witness_pockets.stl",
        min_triangles: 800,
        min_size_mm: [600.0, 520.0, 8.0],
    },
    ExpectedOutput {
        generator: "automated_ecm_coating_qc_station",
        path: "output/automated_ecm_coating_qc_station_assembly.stl",
        min_triangles: 2200,
        min_size_mm: [1160.0, 740.0, 140.0],
    },
    ExpectedOutput {
        generator: "cell_suspension_prep_qc_module",
        path: "output/cell_suspension_prep_qc_module_baseplate.stl",
        min_triangles: 300,
        min_size_mm: [750.0, 420.0, 40.0],
    },
    ExpectedOutput {
        generator: "cell_suspension_prep_qc_module",
        path: "output/cell_suspension_prep_qc_module_bag_holder.stl",
        min_triangles: 250,
        min_size_mm: [300.0, 200.0, 150.0],
    },
    ExpectedOutput {
        generator: "cell_suspension_prep_qc_module",
        path: "output/cell_suspension_prep_qc_module_qc_loop_cartridge.stl",
        min_triangles: 450,
        min_size_mm: [280.0, 140.0, 80.0],
    },
    ExpectedOutput {
        generator: "cell_suspension_prep_qc_module",
        path: "output/cell_suspension_prep_qc_module_handoff_manifold.stl",
        min_triangles: 450,
        min_size_mm: [430.0, 130.0, 40.0],
    },
    ExpectedOutput {
        generator: "cell_suspension_prep_qc_module",
        path: "output/cell_suspension_prep_qc_module_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [830.0, 430.0, 210.0],
    },
    ExpectedOutput {
        generator: "media_sampling_analyzer_interface",
        path: "output/media_sampling_analyzer_interface_baseplate.stl",
        min_triangles: 250,
        min_size_mm: [750.0, 430.0, 45.0],
    },
    ExpectedOutput {
        generator: "media_sampling_analyzer_interface",
        path: "output/media_sampling_analyzer_interface_selector_manifold.stl",
        min_triangles: 600,
        min_size_mm: [610.0, 80.0, 50.0],
    },
    ExpectedOutput {
        generator: "media_sampling_analyzer_interface",
        path: "output/media_sampling_analyzer_interface_sterile_bulkhead_panel.stl",
        min_triangles: 800,
        min_size_mm: [690.0, 40.0, 280.0],
    },
    ExpectedOutput {
        generator: "media_sampling_analyzer_interface",
        path: "output/media_sampling_analyzer_interface_bubble_dead_volume_control.stl",
        min_triangles: 800,
        min_size_mm: [590.0, 85.0, 80.0],
    },
    ExpectedOutput {
        generator: "media_sampling_analyzer_interface",
        path: "output/media_sampling_analyzer_interface_assembly.stl",
        min_triangles: 2200,
        min_size_mm: [750.0, 440.0, 300.0],
    },
    ExpectedOutput {
        generator: "run_record_material_scan_station",
        path: "output/run_record_material_scan_station_cleanable_deck.stl",
        min_triangles: 300,
        min_size_mm: [1020.0, 700.0, 15.0],
    },
    ExpectedOutput {
        generator: "run_record_material_scan_station",
        path: "output/run_record_material_scan_station_barcode_rfid_scanner_bridge.stl",
        min_triangles: 300,
        min_size_mm: [600.0, 70.0, 120.0],
    },
    ExpectedOutput {
        generator: "run_record_material_scan_station",
        path: "output/run_record_material_scan_station_lot_staging_pockets.stl",
        min_triangles: 1000,
        min_size_mm: [500.0, 450.0, 25.0],
    },
    ExpectedOutput {
        generator: "run_record_material_scan_station",
        path: "output/run_record_material_scan_station_weigh_scale_load_cell_placeholder.stl",
        min_triangles: 300,
        min_size_mm: [170.0, 140.0, 40.0],
    },
    ExpectedOutput {
        generator: "run_record_material_scan_station",
        path: "output/run_record_material_scan_station_assembly.stl",
        min_triangles: 2200,
        min_size_mm: [1020.0, 700.0, 170.0],
    },
    ExpectedOutput {
        generator: "connector_topology_scan_station",
        path: "output/connector_topology_scan_station_deck.stl",
        min_triangles: 300,
        min_size_mm: [1100.0, 800.0, 40.0],
    },
    ExpectedOutput {
        generator: "connector_topology_scan_station",
        path: "output/connector_topology_scan_station_twenty_chip_cassette_datum.stl",
        min_triangles: 800,
        min_size_mm: [600.0, 530.0, 25.0],
    },
    ExpectedOutput {
        generator: "connector_topology_scan_station",
        path: "output/connector_topology_scan_station_connector_id_scan_comb.stl",
        min_triangles: 1000,
        min_size_mm: [170.0, 500.0, 20.0],
    },
    ExpectedOutput {
        generator: "connector_topology_scan_station",
        path: "output/connector_topology_scan_station_camera_illumination_scan_bridge.stl",
        min_triangles: 500,
        min_size_mm: [700.0, 50.0, 110.0],
    },
    ExpectedOutput {
        generator: "connector_topology_scan_station",
        path: "output/connector_topology_scan_station_assembly.stl",
        min_triangles: 2200,
        min_size_mm: [1100.0, 800.0, 130.0],
    },
    ExpectedOutput {
        generator: "incubator_cassette_shuttle_airlock",
        path: "output/incubator_cassette_shuttle_airlock_shell.stl",
        min_triangles: 300,
        min_size_mm: [880.0, 840.0, 350.0],
    },
    ExpectedOutput {
        generator: "incubator_cassette_shuttle_airlock",
        path: "output/incubator_cassette_shuttle_airlock_sealed_tray.stl",
        min_triangles: 500,
        min_size_mm: [680.0, 600.0, 35.0],
    },
    ExpectedOutput {
        generator: "incubator_cassette_shuttle_airlock",
        path: "output/incubator_cassette_shuttle_airlock_dual_door_interlock.stl",
        min_triangles: 600,
        min_size_mm: [800.0, 850.0, 250.0],
    },
    ExpectedOutput {
        generator: "incubator_cassette_shuttle_airlock",
        path: "output/incubator_cassette_shuttle_airlock_assembly.stl",
        min_triangles: 2000,
        min_size_mm: [880.0, 850.0, 360.0],
    },
    ExpectedOutput {
        generator: "robotic_cassette_gripper_end_effector",
        path: "output/robotic_cassette_gripper_end_effector_wrist_plate.stl",
        min_triangles: 300,
        min_size_mm: [120.0, 120.0, 10.0],
    },
    ExpectedOutput {
        generator: "robotic_cassette_gripper_end_effector",
        path: "output/robotic_cassette_gripper_end_effector_finger_bodies.stl",
        min_triangles: 500,
        min_size_mm: [600.0, 520.0, 80.0],
    },
    ExpectedOutput {
        generator: "robotic_cassette_gripper_end_effector",
        path: "output/robotic_cassette_gripper_end_effector_collision_keepout.stl",
        min_triangles: 100,
        min_size_mm: [840.0, 630.0, 230.0],
    },
    ExpectedOutput {
        generator: "robotic_cassette_gripper_end_effector",
        path: "output/robotic_cassette_gripper_end_effector_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [840.0, 630.0, 230.0],
    },
    ExpectedOutput {
        generator: "chip_cassette_position_randomization_tray",
        path: "output/chip_cassette_position_randomization_tray_base_tray.stl",
        min_triangles: 200,
        min_size_mm: [960.0, 660.0, 20.0],
    },
    ExpectedOutput {
        generator: "chip_cassette_position_randomization_tray",
        path: "output/chip_cassette_position_randomization_tray_cassette_datum.stl",
        min_triangles: 300,
        min_size_mm: [560.0, 480.0, 25.0],
    },
    ExpectedOutput {
        generator: "chip_cassette_position_randomization_tray",
        path: "output/chip_cassette_position_randomization_tray_clean_chip_staging_pockets.stl",
        min_triangles: 300,
        min_size_mm: [280.0, 230.0, 25.0],
    },
    ExpectedOutput {
        generator: "chip_cassette_position_randomization_tray",
        path: "output/chip_cassette_position_randomization_tray_robot_pick_clearances.stl",
        min_triangles: 100,
        min_size_mm: [560.0, 480.0, 90.0],
    },
    ExpectedOutput {
        generator: "chip_cassette_position_randomization_tray",
        path: "output/chip_cassette_position_randomization_tray_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [960.0, 660.0, 90.0],
    },
    ExpectedOutput {
        generator: "environmental_mapping_cassette_surrogate",
        path: "output/environmental_mapping_cassette_surrogate_body_frame.stl",
        min_triangles: 400,
        min_size_mm: [580.0, 500.0, 15.0],
    },
    ExpectedOutput {
        generator: "environmental_mapping_cassette_surrogate",
        path: "output/environmental_mapping_cassette_surrogate_sensor_pockets.stl",
        min_triangles: 1000,
        min_size_mm: [500.0, 400.0, 6.0],
    },
    ExpectedOutput {
        generator: "environmental_mapping_cassette_surrogate",
        path: "output/environmental_mapping_cassette_surrogate_flow_dummy_channels.stl",
        min_triangles: 300,
        min_size_mm: [500.0, 380.0, 3.0],
    },
    ExpectedOutput {
        generator: "environmental_mapping_cassette_surrogate",
        path: "output/environmental_mapping_cassette_surrogate_cable_strain_relief.stl",
        min_triangles: 300,
        min_size_mm: [250.0, 25.0, 20.0],
    },
    ExpectedOutput {
        generator: "environmental_mapping_cassette_surrogate",
        path: "output/environmental_mapping_cassette_surrogate_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [580.0, 500.0, 35.0],
    },
    ExpectedOutput {
        generator: "environmental_sensor_calibration_station",
        path: "output/environmental_sensor_calibration_station_base_tray.stl",
        min_triangles: 300,
        min_size_mm: [800.0, 540.0, 15.0],
    },
    ExpectedOutput {
        generator: "environmental_sensor_calibration_station",
        path: "output/environmental_sensor_calibration_station_gas_reference_manifold.stl",
        min_triangles: 300,
        min_size_mm: [300.0, 80.0, 40.0],
    },
    ExpectedOutput {
        generator: "environmental_sensor_calibration_station",
        path: "output/environmental_sensor_calibration_station_humidity_standard_block.stl",
        min_triangles: 300,
        min_size_mm: [240.0, 100.0, 35.0],
    },
    ExpectedOutput {
        generator: "environmental_sensor_calibration_station",
        path: "output/environmental_sensor_calibration_station_leak_capture_tray.stl",
        min_triangles: 100,
        min_size_mm: [700.0, 45.0, 15.0],
    },
    ExpectedOutput {
        generator: "environmental_sensor_calibration_station",
        path: "output/environmental_sensor_calibration_station_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [800.0, 540.0, 100.0],
    },
    ExpectedOutput {
        generator: "cassette_storage_recovery_incubator_rack",
        path: "output/cassette_storage_recovery_incubator_rack_leak_tray_base.stl",
        min_triangles: 300,
        min_size_mm: [700.0, 720.0, 25.0],
    },
    ExpectedOutput {
        generator: "cassette_storage_recovery_incubator_rack",
        path: "output/cassette_storage_recovery_incubator_rack_multi_cassette_slot_rails.stl",
        min_triangles: 500,
        min_size_mm: [600.0, 500.0, 350.0],
    },
    ExpectedOutput {
        generator: "cassette_storage_recovery_incubator_rack",
        path: "output/cassette_storage_recovery_incubator_rack_airflow_thermal_spacing.stl",
        min_triangles: 300,
        min_size_mm: [640.0, 550.0, 300.0],
    },
    ExpectedOutput {
        generator: "cassette_storage_recovery_incubator_rack",
        path: "output/cassette_storage_recovery_incubator_rack_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [720.0, 740.0, 540.0],
    },
    ExpectedOutput {
        generator: "closed_cell_harvest_passaging_module",
        path: "output/closed_cell_harvest_passaging_module_base_deck.stl",
        min_triangles: 200,
        min_size_mm: [1300.0, 800.0, 25.0],
    },
    ExpectedOutput {
        generator: "closed_cell_harvest_passaging_module",
        path: "output/closed_cell_harvest_passaging_module_vessel_docking_bay.stl",
        min_triangles: 300,
        min_size_mm: [400.0, 300.0, 100.0],
    },
    ExpectedOutput {
        generator: "closed_cell_harvest_passaging_module",
        path: "output/closed_cell_harvest_passaging_module_pump_valve_bank.stl",
        min_triangles: 300,
        min_size_mm: [600.0, 150.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_cell_harvest_passaging_module",
        path: "output/closed_cell_harvest_passaging_module_assembly.stl",
        min_triangles: 1500,
        min_size_mm: [1300.0, 800.0, 180.0],
    },
    ExpectedOutput {
        generator: "perfusion_bubble_management_module",
        path: "output/perfusion_bubble_management_module_base_leak_tray.stl",
        min_triangles: 100,
        min_size_mm: [620.0, 320.0, 15.0],
    },
    ExpectedOutput {
        generator: "perfusion_bubble_management_module",
        path: "output/perfusion_bubble_management_module_upstream_debubbler.stl",
        min_triangles: 300,
        min_size_mm: [500.0, 80.0, 70.0],
    },
    ExpectedOutput {
        generator: "perfusion_bubble_management_module",
        path: "output/perfusion_bubble_management_module_optical_sensor_blocks.stl",
        min_triangles: 300,
        min_size_mm: [350.0, 80.0, 50.0],
    },
    ExpectedOutput {
        generator: "perfusion_bubble_management_module",
        path: "output/perfusion_bubble_management_module_assembly.stl",
        min_triangles: 1200,
        min_size_mm: [620.0, 300.0, 110.0],
    },
    ExpectedOutput {
        generator: "automated_cell_seeding_distribution_manifold",
        path: "output/automated_cell_seeding_distribution_manifold_datum_plate.stl",
        min_triangles: 300,
        min_size_mm: [820.0, 720.0, 15.0],
    },
    ExpectedOutput {
        generator: "automated_cell_seeding_distribution_manifold",
        path: "output/automated_cell_seeding_distribution_manifold_equalized_20_way_channels.stl",
        min_triangles: 500,
        min_size_mm: [500.0, 360.0, 10.0],
    },
    ExpectedOutput {
        generator: "automated_cell_seeding_distribution_manifold",
        path:
            "output/automated_cell_seeding_distribution_manifold_pressure_shear_sensor_pockets.stl",
        min_triangles: 500,
        min_size_mm: [420.0, 400.0, 12.0],
    },
    ExpectedOutput {
        generator: "automated_cell_seeding_distribution_manifold",
        path: "output/automated_cell_seeding_distribution_manifold_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [820.0, 720.0, 140.0],
    },
    ExpectedOutput {
        generator: "inline_media_conditioning_qc_module",
        path: "output/inline_media_conditioning_qc_module_baseplate.stl",
        min_triangles: 200,
        min_size_mm: [840.0, 500.0, 15.0],
    },
    ExpectedOutput {
        generator: "inline_media_conditioning_qc_module",
        path: "output/inline_media_conditioning_qc_module_water_jacket_block.stl",
        min_triangles: 300,
        min_size_mm: [300.0, 100.0, 35.0],
    },
    ExpectedOutput {
        generator: "inline_media_conditioning_qc_module",
        path: "output/inline_media_conditioning_qc_module_sensor_qc_manifold.stl",
        min_triangles: 300,
        min_size_mm: [260.0, 80.0, 35.0],
    },
    ExpectedOutput {
        generator: "inline_media_conditioning_qc_module",
        path: "output/inline_media_conditioning_qc_module_assembly.stl",
        min_triangles: 1600,
        min_size_mm: [840.0, 500.0, 80.0],
    },
    ExpectedOutput {
        generator: "gas_humidity_service_panel",
        path: "output/gas_humidity_service_panel_baseplate.stl",
        min_triangles: 100,
        min_size_mm: [740.0, 340.0, 15.0],
    },
    ExpectedOutput {
        generator: "gas_humidity_service_panel",
        path: "output/gas_humidity_service_panel_bulkhead_panel.stl",
        min_triangles: 100,
        min_size_mm: [680.0, 20.0, 440.0],
    },
    ExpectedOutput {
        generator: "gas_humidity_service_panel",
        path: "output/gas_humidity_service_panel_gas_control_bank.stl",
        min_triangles: 500,
        min_size_mm: [400.0, 70.0, 150.0],
    },
    ExpectedOutput {
        generator: "gas_humidity_service_panel",
        path: "output/gas_humidity_service_panel_assembly.stl",
        min_triangles: 1400,
        min_size_mm: [740.0, 340.0, 440.0],
    },
    ExpectedOutput {
        generator: "robot_tool_change_and_end_effector_rack",
        path: "output/robot_tool_change_and_end_effector_rack_drip_tray.stl",
        min_triangles: 100,
        min_size_mm: [880.0, 440.0, 40.0],
    },
    ExpectedOutput {
        generator: "robot_tool_change_and_end_effector_rack",
        path: "output/robot_tool_change_and_end_effector_rack_clean_pocket_bank.stl",
        min_triangles: 300,
        min_size_mm: [700.0, 150.0, 60.0],
    },
    ExpectedOutput {
        generator: "robot_tool_change_and_end_effector_rack",
        path: "output/robot_tool_change_and_end_effector_rack_collision_service_keepouts.stl",
        min_triangles: 100,
        min_size_mm: [850.0, 420.0, 260.0],
    },
    ExpectedOutput {
        generator: "robot_tool_change_and_end_effector_rack",
        path: "output/robot_tool_change_and_end_effector_rack_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [880.0, 440.0, 300.0],
    },
    ExpectedOutput {
        generator: "sterile_fluid_path_integrity_tester",
        path: "output/sterile_fluid_path_integrity_tester_baseplate.stl",
        min_triangles: 200,
        min_size_mm: [960.0, 740.0, 15.0],
    },
    ExpectedOutput {
        generator: "sterile_fluid_path_integrity_tester",
        path: "output/sterile_fluid_path_integrity_tester_cassette_datum_nest.stl",
        min_triangles: 300,
        min_size_mm: [600.0, 520.0, 20.0],
    },
    ExpectedOutput {
        generator: "sterile_fluid_path_integrity_tester",
        path: "output/sterile_fluid_path_integrity_tester_pressure_decay_sensor_matrix.stl",
        min_triangles: 500,
        min_size_mm: [600.0, 150.0, 20.0],
    },
    ExpectedOutput {
        generator: "sterile_fluid_path_integrity_tester",
        path: "output/sterile_fluid_path_integrity_tester_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [960.0, 740.0, 100.0],
    },
    ExpectedOutput {
        generator: "aseptic_tubing_weld_seal_station",
        path: "output/aseptic_tubing_weld_seal_station_deck.stl",
        min_triangles: 200,
        min_size_mm: [1260.0, 800.0, 15.0],
    },
    ExpectedOutput {
        generator: "aseptic_tubing_weld_seal_station",
        path: "output/aseptic_tubing_weld_seal_station_equipment_envelopes.stl",
        min_triangles: 200,
        min_size_mm: [600.0, 250.0, 250.0],
    },
    ExpectedOutput {
        generator: "aseptic_tubing_weld_seal_station",
        path: "output/aseptic_tubing_weld_seal_station_cut_weld_seal_lanes.stl",
        min_triangles: 300,
        min_size_mm: [850.0, 200.0, 25.0],
    },
    ExpectedOutput {
        generator: "aseptic_tubing_weld_seal_station",
        path: "output/aseptic_tubing_weld_seal_station_assembly.stl",
        min_triangles: 1600,
        min_size_mm: [1260.0, 800.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_thaw_equilibration_station",
        path: "output/closed_reagent_thaw_equilibration_station_base_leak_tray.stl",
        min_triangles: 300,
        min_size_mm: [1200.0, 740.0, 25.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_thaw_equilibration_station",
        path: "output/closed_reagent_thaw_equilibration_station_controlled_thaw_block.stl",
        min_triangles: 500,
        min_size_mm: [500.0, 300.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_thaw_equilibration_station",
        path: "output/closed_reagent_thaw_equilibration_station_sterile_connector_bulkhead.stl",
        min_triangles: 700,
        min_size_mm: [720.0, 30.0, 200.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_thaw_equilibration_station",
        path: "output/closed_reagent_thaw_equilibration_station_assembly.stl",
        min_triangles: 2000,
        min_size_mm: [1200.0, 740.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_sample_fraction_archive_module",
        path: "output/closed_sample_fraction_archive_module_leak_tray_base.stl",
        min_triangles: 300,
        min_size_mm: [700.0, 480.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_sample_fraction_archive_module",
        path: "output/closed_sample_fraction_archive_module_fraction_tube_nest.stl",
        min_triangles: 1000,
        min_size_mm: [180.0, 130.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_sample_fraction_archive_module",
        path: "output/closed_sample_fraction_archive_module_sterile_sample_loop_handoff.stl",
        min_triangles: 700,
        min_size_mm: [400.0, 90.0, 55.0],
    },
    ExpectedOutput {
        generator: "closed_sample_fraction_archive_module",
        path: "output/closed_sample_fraction_archive_module_assembly.stl",
        min_triangles: 2200,
        min_size_mm: [700.0, 480.0, 200.0],
    },
    ExpectedOutput {
        generator: "closed_cleaning_sanitization_validation_cart",
        path: "output/closed_cleaning_sanitization_validation_cart_frame.stl",
        min_triangles: 400,
        min_size_mm: [950.0, 450.0, 750.0],
    },
    ExpectedOutput {
        generator: "closed_cleaning_sanitization_validation_cart",
        path: "output/closed_cleaning_sanitization_validation_cart_leak_tray.stl",
        min_triangles: 100,
        min_size_mm: [900.0, 450.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_cleaning_sanitization_validation_cart",
        path: "output/closed_cleaning_sanitization_validation_cart_flush_ports.stl",
        min_triangles: 700,
        min_size_mm: [600.0, 30.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_cleaning_sanitization_validation_cart",
        path: "output/closed_cleaning_sanitization_validation_cart_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1100.0, 600.0, 1100.0],
    },
    ExpectedOutput {
        generator: "closed_fluid_path_packaging_kitting_station",
        path: "output/closed_fluid_path_packaging_kitting_station_deck_tray.stl",
        min_triangles: 200,
        min_size_mm: [1200.0, 800.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_fluid_path_packaging_kitting_station",
        path: "output/closed_fluid_path_packaging_kitting_station_sterile_tubing_harness_kit_trays.stl",
        min_triangles: 600,
        min_size_mm: [350.0, 230.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_fluid_path_packaging_kitting_station",
        path: "output/closed_fluid_path_packaging_kitting_station_leak_test_handoff_ports.stl",
        min_triangles: 600,
        min_size_mm: [550.0, 65.0, 25.0],
    },
    ExpectedOutput {
        generator: "closed_fluid_path_packaging_kitting_station",
        path: "output/closed_fluid_path_packaging_kitting_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1200.0, 820.0, 170.0],
    },
    ExpectedOutput {
        generator: "cell_lot_release_qc_panel",
        path: "output/cell_lot_release_qc_panel_base_tray.stl",
        min_triangles: 200,
        min_size_mm: [1150.0, 730.0, 15.0],
    },
    ExpectedOutput {
        generator: "cell_lot_release_qc_panel",
        path: "output/cell_lot_release_qc_panel_sealed_sample_receiving_nest.stl",
        min_triangles: 500,
        min_size_mm: [300.0, 200.0, 40.0],
    },
    ExpectedOutput {
        generator: "cell_lot_release_qc_panel",
        path: "output/cell_lot_release_qc_panel_count_viability_analyzer_dock.stl",
        min_triangles: 200,
        min_size_mm: [430.0, 280.0, 250.0],
    },
    ExpectedOutput {
        generator: "cell_lot_release_qc_panel",
        path: "output/cell_lot_release_qc_panel_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1150.0, 740.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_module_cip_sip_service_station",
        path: "output/closed_module_cip_sip_service_station_base_deck.stl",
        min_triangles: 200,
        min_size_mm: [1300.0, 850.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_module_cip_sip_service_station",
        path: "output/closed_module_cip_sip_service_station_sealed_module_dock.stl",
        min_triangles: 600,
        min_size_mm: [800.0, 650.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_module_cip_sip_service_station",
        path: "output/closed_module_cip_sip_service_station_flush_inlet_bulkhead.stl",
        min_triangles: 600,
        min_size_mm: [450.0, 25.0, 220.0],
    },
    ExpectedOutput {
        generator: "closed_module_cip_sip_service_station",
        path: "output/closed_module_cip_sip_service_station_assembly.stl",
        min_triangles: 2600,
        min_size_mm: [1300.0, 870.0, 250.0],
    },
    ExpectedOutput {
        generator: "cassette_deviation_quarantine_station",
        path: "output/cassette_deviation_quarantine_station_leak_tray_base.stl",
        min_triangles: 200,
        min_size_mm: [2200.0, 1800.0, 18.0],
    },
    ExpectedOutput {
        generator: "cassette_deviation_quarantine_station",
        path: "output/cassette_deviation_quarantine_station_status_bay_array.stl",
        min_triangles: 1000,
        min_size_mm: [1700.0, 850.0, 35.0],
    },
    ExpectedOutput {
        generator: "cassette_deviation_quarantine_station",
        path: "output/cassette_deviation_quarantine_station_isolation_cover_envelope.stl",
        min_triangles: 160,
        min_size_mm: [1800.0, 1000.0, 220.0],
    },
    ExpectedOutput {
        generator: "cassette_deviation_quarantine_station",
        path: "output/cassette_deviation_quarantine_station_assembly.stl",
        min_triangles: 3200,
        min_size_mm: [2200.0, 1800.0, 230.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_excursion_response_station",
        path: "output/closed_environmental_excursion_response_station_leak_condensate_tray.stl",
        min_triangles: 200,
        min_size_mm: [1280.0, 800.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_excursion_response_station",
        path: "output/closed_environmental_excursion_response_station_suspect_cassette_tote_dock.stl",
        min_triangles: 600,
        min_size_mm: [720.0, 640.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_excursion_response_station",
        path: "output/closed_environmental_excursion_response_station_quarantine_cover_envelope.stl",
        min_triangles: 400,
        min_size_mm: [720.0, 640.0, 230.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_excursion_response_station",
        path: "output/closed_environmental_excursion_response_station_assembly.stl",
        min_triangles: 2600,
        min_size_mm: [1300.0, 820.0, 250.0],
    },
    ExpectedOutput {
        generator: "sterile_tote_docking_and_transfer_station",
        path: "output/sterile_tote_docking_and_transfer_station_base_leak_condensate_tray.stl",
        min_triangles: 200,
        min_size_mm: [1500.0, 950.0, 20.0],
    },
    ExpectedOutput {
        generator: "sterile_tote_docking_and_transfer_station",
        path: "output/sterile_tote_docking_and_transfer_station_sealed_tote_receiver_datum.stl",
        min_triangles: 500,
        min_size_mm: [780.0, 680.0, 35.0],
    },
    ExpectedOutput {
        generator: "sterile_tote_docking_and_transfer_station",
        path: "output/sterile_tote_docking_and_transfer_station_transfer_tongue_rail_interface.stl",
        min_triangles: 300,
        min_size_mm: [700.0, 400.0, 25.0],
    },
    ExpectedOutput {
        generator: "sterile_tote_docking_and_transfer_station",
        path: "output/sterile_tote_docking_and_transfer_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1500.0, 950.0, 250.0],
    },
    ExpectedOutput {
        generator: "bioreactor_feed_harvest_bag_hotel",
        path: "output/bioreactor_feed_harvest_bag_hotel_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1280.0, 720.0, 20.0],
    },
    ExpectedOutput {
        generator: "bioreactor_feed_harvest_bag_hotel",
        path: "output/bioreactor_feed_harvest_bag_hotel_fresh_feed_bag_positions.stl",
        min_triangles: 400,
        min_size_mm: [480.0, 390.0, 15.0],
    },
    ExpectedOutput {
        generator: "bioreactor_feed_harvest_bag_hotel",
        path: "output/bioreactor_feed_harvest_bag_hotel_sterile_connector_bulkhead.stl",
        min_triangles: 600,
        min_size_mm: [650.0, 35.0, 150.0],
    },
    ExpectedOutput {
        generator: "bioreactor_feed_harvest_bag_hotel",
        path: "output/bioreactor_feed_harvest_bag_hotel_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1300.0, 740.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_cell_bank_recovery_thaw_station",
        path: "output/closed_cell_bank_recovery_thaw_station_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1380.0, 860.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_cell_bank_recovery_thaw_station",
        path: "output/closed_cell_bank_recovery_thaw_station_cryovial_bag_receiving_nest.stl",
        min_triangles: 800,
        min_size_mm: [420.0, 260.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_cell_bank_recovery_thaw_station",
        path: "output/closed_cell_bank_recovery_thaw_station_controlled_thaw_block.stl",
        min_triangles: 600,
        min_size_mm: [380.0, 220.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_cell_bank_recovery_thaw_station",
        path: "output/closed_cell_bank_recovery_thaw_station_assembly.stl",
        min_triangles: 3000,
        min_size_mm: [1400.0, 880.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_consumable_pre_use_inspection_station",
        path: "output/closed_consumable_pre_use_inspection_station_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1320.0, 820.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_consumable_pre_use_inspection_station",
        path: "output/closed_consumable_pre_use_inspection_station_clean_incoming_kit_datum.stl",
        min_triangles: 1200,
        min_size_mm: [440.0, 420.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_consumable_pre_use_inspection_station",
        path: "output/closed_consumable_pre_use_inspection_station_optical_inspection_window_camera_bridge.stl",
        min_triangles: 500,
        min_size_mm: [650.0, 250.0, 150.0],
    },
    ExpectedOutput {
        generator: "closed_consumable_pre_use_inspection_station",
        path: "output/closed_consumable_pre_use_inspection_station_assembly.stl",
        min_triangles: 3000,
        min_size_mm: [1320.0, 840.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_deviation_sample_triage_station",
        path: "output/closed_deviation_sample_triage_station_leak_tray_base.stl",
        min_triangles: 200,
        min_size_mm: [1080.0, 700.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_deviation_sample_triage_station",
        path: "output/closed_deviation_sample_triage_station_sealed_incoming_sample_receiver.stl",
        min_triangles: 800,
        min_size_mm: [300.0, 180.0, 35.0],
    },
    ExpectedOutput {
        generator: "closed_deviation_sample_triage_station",
        path: "output/closed_deviation_sample_triage_station_cold_block_interface.stl",
        min_triangles: 600,
        min_size_mm: [200.0, 130.0, 35.0],
    },
    ExpectedOutput {
        generator: "closed_deviation_sample_triage_station",
        path: "output/closed_deviation_sample_triage_station_assembly.stl",
        min_triangles: 3000,
        min_size_mm: [1100.0, 720.0, 220.0],
    },
    ExpectedOutput {
        generator: "closed_module_residual_rinse_sampling_station",
        path: "output/closed_module_residual_rinse_sampling_station_baseplate.stl",
        min_triangles: 200,
        min_size_mm: [1080.0, 740.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_module_residual_rinse_sampling_station",
        path: "output/closed_module_residual_rinse_sampling_station_sealed_module_dock.stl",
        min_triangles: 600,
        min_size_mm: [580.0, 340.0, 100.0],
    },
    ExpectedOutput {
        generator: "closed_module_residual_rinse_sampling_station",
        path: "output/closed_module_residual_rinse_sampling_station_sample_vial_carousel.stl",
        min_triangles: 600,
        min_size_mm: [250.0, 250.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_module_residual_rinse_sampling_station",
        path: "output/closed_module_residual_rinse_sampling_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1100.0, 760.0, 120.0],
    },
    ExpectedOutput {
        generator: "sterile_gas_changeover_regulator_qualification_panel",
        path: "output/sterile_gas_changeover_regulator_qualification_panel_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1600.0, 880.0, 20.0],
    },
    ExpectedOutput {
        generator: "sterile_gas_changeover_regulator_qualification_panel",
        path: "output/sterile_gas_changeover_regulator_qualification_panel_regulator_mfc_envelopes.stl",
        min_triangles: 600,
        min_size_mm: [900.0, 80.0, 120.0],
    },
    ExpectedOutput {
        generator: "sterile_gas_changeover_regulator_qualification_panel",
        path: "output/sterile_gas_changeover_regulator_qualification_panel_sterile_filter_check_valve_bank.stl",
        min_triangles: 600,
        min_size_mm: [900.0, 50.0, 80.0],
    },
    ExpectedOutput {
        generator: "sterile_gas_changeover_regulator_qualification_panel",
        path: "output/sterile_gas_changeover_regulator_qualification_panel_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1600.0, 900.0, 300.0],
    },
    ExpectedOutput {
        generator: "closed_material_passthrough_debagging_station",
        path: "output/closed_material_passthrough_debagging_station_base_leak_debris_tray.stl",
        min_triangles: 200,
        min_size_mm: [1480.0, 920.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_material_passthrough_debagging_station",
        path: "output/closed_material_passthrough_debagging_station_sealed_tote_transfer_hatch_receiver.stl",
        min_triangles: 600,
        min_size_mm: [650.0, 100.0, 60.0],
    },
    ExpectedOutput {
        generator: "closed_material_passthrough_debagging_station",
        path: "output/closed_material_passthrough_debagging_station_evidence_photo_inspection_bridge.stl",
        min_triangles: 600,
        min_size_mm: [600.0, 200.0, 150.0],
    },
    ExpectedOutput {
        generator: "closed_material_passthrough_debagging_station",
        path: "output/closed_material_passthrough_debagging_station_assembly.stl",
        min_triangles: 3000,
        min_size_mm: [1500.0, 940.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_monitoring_plate_coupon_station",
        path: "output/closed_environmental_monitoring_plate_coupon_station_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1450.0, 900.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_monitoring_plate_coupon_station",
        path: "output/closed_environmental_monitoring_plate_coupon_station_settle_plate_exposure_grid.stl",
        min_triangles: 600,
        min_size_mm: [500.0, 250.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_monitoring_plate_coupon_station",
        path: "output/closed_environmental_monitoring_plate_coupon_station_evidence_photo_bridge.stl",
        min_triangles: 500,
        min_size_mm: [600.0, 180.0, 150.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_monitoring_plate_coupon_station",
        path: "output/closed_environmental_monitoring_plate_coupon_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1450.0, 900.0, 220.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_cartridge_hydration_calibration_station",
        path: "output/inline_sensor_cartridge_hydration_calibration_station_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1240.0, 780.0, 18.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_cartridge_hydration_calibration_station",
        path: "output/inline_sensor_cartridge_hydration_calibration_station_clean_cartridge_rack.stl",
        min_triangles: 800,
        min_size_mm: [400.0, 180.0, 30.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_cartridge_hydration_calibration_station",
        path: "output/inline_sensor_cartridge_hydration_calibration_station_electrical_pogo_check_fixture.stl",
        min_triangles: 500,
        min_size_mm: [300.0, 80.0, 30.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_cartridge_hydration_calibration_station",
        path: "output/inline_sensor_cartridge_hydration_calibration_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1240.0, 780.0, 200.0],
    },
    ExpectedOutput {
        generator: "robotic_spill_response_decon_kit_station",
        path: "output/robotic_spill_response_decon_kit_station_drip_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [1240.0, 780.0, 18.0],
    },
    ExpectedOutput {
        generator: "robotic_spill_response_decon_kit_station",
        path: "output/robotic_spill_response_decon_kit_station_sealed_spill_pad_cassette.stl",
        min_triangles: 600,
        min_size_mm: [340.0, 240.0, 80.0],
    },
    ExpectedOutput {
        generator: "robotic_spill_response_decon_kit_station",
        path: "output/robotic_spill_response_decon_kit_station_used_material_quarantine_bin.stl",
        min_triangles: 600,
        min_size_mm: [340.0, 260.0, 180.0],
    },
    ExpectedOutput {
        generator: "robotic_spill_response_decon_kit_station",
        path: "output/robotic_spill_response_decon_kit_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1240.0, 800.0, 220.0],
    },
    ExpectedOutput {
        generator: "closed_run_start_readiness_gate_station",
        path: "output/closed_run_start_readiness_gate_station_cleanable_interlock_deck.stl",
        min_triangles: 200,
        min_size_mm: [1740.0, 1200.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_run_start_readiness_gate_station",
        path: "output/closed_run_start_readiness_gate_station_cassette_id_dock.stl",
        min_triangles: 600,
        min_size_mm: [620.0, 560.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_run_start_readiness_gate_station",
        path: "output/closed_run_start_readiness_gate_station_camera_evidence_bridge.stl",
        min_triangles: 500,
        min_size_mm: [1300.0, 80.0, 180.0],
    },
    ExpectedOutput {
        generator: "closed_run_start_readiness_gate_station",
        path: "output/closed_run_start_readiness_gate_station_assembly.stl",
        min_triangles: 3000,
        min_size_mm: [1740.0, 1200.0, 220.0],
    },
    ExpectedOutput {
        generator: "closed_pump_valve_manifold_calibration_station",
        path: "output/closed_pump_valve_manifold_calibration_station_base_leak_tray.stl",
        min_triangles: 200,
        min_size_mm: [940.0, 600.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_pump_valve_manifold_calibration_station",
        path: "output/closed_pump_valve_manifold_calibration_station_valve_actuation_map_plate.stl",
        min_triangles: 600,
        min_size_mm: [760.0, 80.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_pump_valve_manifold_calibration_station",
        path: "output/closed_pump_valve_manifold_calibration_station_pressure_flow_sensor_pockets.stl",
        min_triangles: 600,
        min_size_mm: [760.0, 70.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_pump_valve_manifold_calibration_station",
        path: "output/closed_pump_valve_manifold_calibration_station_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [950.0, 620.0, 100.0],
    },
    ExpectedOutput {
        generator: "closed_calibration_standard_custody_gate",
        path: "output/closed_calibration_standard_custody_gate_base_tray.stl",
        min_triangles: 200,
        min_size_mm: [940.0, 620.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_calibration_standard_custody_gate",
        path: "output/closed_calibration_standard_custody_gate_temperature_pocket_block.stl",
        min_triangles: 500,
        min_size_mm: [380.0, 130.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_calibration_standard_custody_gate",
        path: "output/closed_calibration_standard_custody_gate_reader_dock_panel.stl",
        min_triangles: 500,
        min_size_mm: [300.0, 120.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_calibration_standard_custody_gate",
        path: "output/closed_calibration_standard_custody_gate_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [940.0, 620.0, 120.0],
    },
    ExpectedOutput {
        generator: "sterile_tubing_harness",
        path: "output/sterile_tubing_harness_manifold_insert.stl",
        min_triangles: 160,
        min_size_mm: [620.0, 85.0, 15.0],
    },
    ExpectedOutput {
        generator: "sterile_tubing_harness",
        path: "output/sterile_tubing_harness_branch_comb.stl",
        min_triangles: 160,
        min_size_mm: [590.0, 470.0, 10.0],
    },
    ExpectedOutput {
        generator: "sterile_tubing_harness",
        path: "output/sterile_tubing_harness_assembly.stl",
        min_triangles: 240,
        min_size_mm: [680.0, 560.0, 20.0],
    },
    ExpectedOutput {
        generator: "cassette_bench_nest",
        path: "output/cassette_bench_nest_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [720.0, 620.0, 15.0],
    },
    ExpectedOutput {
        generator: "cassette_bench_nest",
        path: "output/cassette_bench_nest_assembly.stl",
        min_triangles: 160,
        min_size_mm: [720.0, 620.0, 35.0],
    },
    ExpectedOutput {
        generator: "sealed_culture_module",
        path: "output/sealed_culture_module_base.stl",
        min_triangles: 120,
        min_size_mm: [850.0, 750.0, 60.0],
    },
    ExpectedOutput {
        generator: "sealed_culture_module",
        path: "output/sealed_culture_module_assembly.stl",
        min_triangles: 240,
        min_size_mm: [850.0, 780.0, 85.0],
    },
    ExpectedOutput {
        generator: "sealed_module_docking_bay",
        path: "output/sealed_module_docking_bay_tray.stl",
        min_triangles: 100,
        min_size_mm: [950.0, 900.0, 45.0],
    },
    ExpectedOutput {
        generator: "sealed_module_docking_bay",
        path: "output/sealed_module_docking_bay_service_receiver.stl",
        min_triangles: 180,
        min_size_mm: [800.0, 50.0, 120.0],
    },
    ExpectedOutput {
        generator: "sealed_module_docking_bay",
        path: "output/sealed_module_docking_bay_assembly.stl",
        min_triangles: 300,
        min_size_mm: [950.0, 940.0, 150.0],
    },
    ExpectedOutput {
        generator: "culture_module_service_skid",
        path: "output/culture_module_service_skid_deck.stl",
        min_triangles: 180,
        min_size_mm: [1100.0, 900.0, 55.0],
    },
    ExpectedOutput {
        generator: "culture_module_service_skid",
        path: "output/culture_module_service_skid_utility_panel.stl",
        min_triangles: 180,
        min_size_mm: [980.0, 15.0, 120.0],
    },
    ExpectedOutput {
        generator: "culture_module_service_skid",
        path: "output/culture_module_service_skid_assembly.stl",
        min_triangles: 300,
        min_size_mm: [1100.0, 900.0, 140.0],
    },
    ExpectedOutput {
        generator: "media_conditioning_perfusion_rack",
        path: "output/media_conditioning_perfusion_rack_frame.stl",
        min_triangles: 300,
        min_size_mm: [800.0, 450.0, 700.0],
    },
    ExpectedOutput {
        generator: "media_conditioning_perfusion_rack",
        path: "output/media_conditioning_perfusion_rack_conditioning_block.stl",
        min_triangles: 300,
        min_size_mm: [500.0, 90.0, 45.0],
    },
    ExpectedOutput {
        generator: "media_conditioning_perfusion_rack",
        path: "output/media_conditioning_perfusion_rack_valve_filter_manifold.stl",
        min_triangles: 1000,
        min_size_mm: [680.0, 100.0, 200.0],
    },
    ExpectedOutput {
        generator: "media_conditioning_perfusion_rack",
        path: "output/media_conditioning_perfusion_rack_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [800.0, 450.0, 700.0],
    },
    ExpectedOutput {
        generator: "pressure_driven_perfusion_panel",
        path: "output/pressure_driven_perfusion_panel_baseplate.stl",
        min_triangles: 400,
        min_size_mm: [840.0, 360.0, 35.0],
    },
    ExpectedOutput {
        generator: "pressure_driven_perfusion_panel",
        path: "output/pressure_driven_perfusion_panel_instrument_panel.stl",
        min_triangles: 700,
        min_size_mm: [780.0, 25.0, 420.0],
    },
    ExpectedOutput {
        generator: "pressure_driven_perfusion_panel",
        path: "output/pressure_driven_perfusion_panel_reservoir_bulkhead_manifold.stl",
        min_triangles: 800,
        min_size_mm: [680.0, 50.0, 70.0],
    },
    ExpectedOutput {
        generator: "pressure_driven_perfusion_panel",
        path: "output/pressure_driven_perfusion_panel_assembly.stl",
        min_triangles: 2200,
        min_size_mm: [840.0, 360.0, 430.0],
    },
    ExpectedOutput {
        generator: "flow_pressure_validation_fixture",
        path: "output/flow_pressure_validation_fixture_baseplate.stl",
        min_triangles: 400,
        min_size_mm: [880.0, 700.0, 40.0],
    },
    ExpectedOutput {
        generator: "flow_pressure_validation_fixture",
        path: "output/flow_pressure_validation_fixture_restrictor_coupon_carrier.stl",
        min_triangles: 1000,
        min_size_mm: [650.0, 550.0, 30.0],
    },
    ExpectedOutput {
        generator: "flow_pressure_validation_fixture",
        path: "output/flow_pressure_validation_fixture_row_manifold_tree.stl",
        min_triangles: 500,
        min_size_mm: [180.0, 600.0, 75.0],
    },
    ExpectedOutput {
        generator: "flow_pressure_validation_fixture",
        path: "output/flow_pressure_validation_fixture_assembly.stl",
        min_triangles: 2000,
        min_size_mm: [900.0, 700.0, 110.0],
    },
    ExpectedOutput {
        generator: "workcell_calibration_drawer",
        path: "output/workcell_calibration_drawer_base_tray.stl",
        min_triangles: 200,
        min_size_mm: [620.0, 400.0, 45.0],
    },
    ExpectedOutput {
        generator: "workcell_calibration_drawer",
        path: "output/workcell_calibration_drawer_flow_restrictor_caddy.stl",
        min_triangles: 300,
        min_size_mm: [240.0, 65.0, 20.0],
    },
    ExpectedOutput {
        generator: "workcell_calibration_drawer",
        path: "output/workcell_calibration_drawer_pressure_leak_adapter_panel.stl",
        min_triangles: 300,
        min_size_mm: [210.0, 90.0, 25.0],
    },
    ExpectedOutput {
        generator: "workcell_calibration_drawer",
        path: "output/workcell_calibration_drawer_chemistry_standard_block.stl",
        min_triangles: 500,
        min_size_mm: [240.0, 110.0, 40.0],
    },
    ExpectedOutput {
        generator: "workcell_calibration_drawer",
        path: "output/workcell_calibration_drawer_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [620.0, 400.0, 60.0],
    },
    ExpectedOutput {
        generator: "media_reagent_quarantine_pod",
        path: "output/media_reagent_quarantine_pod_shell.stl",
        min_triangles: 300,
        min_size_mm: [1200.0, 730.0, 1350.0],
    },
    ExpectedOutput {
        generator: "media_reagent_quarantine_pod",
        path: "output/media_reagent_quarantine_pod_segregation_bays.stl",
        min_triangles: 600,
        min_size_mm: [950.0, 400.0, 300.0],
    },
    ExpectedOutput {
        generator: "media_reagent_quarantine_pod",
        path: "output/media_reagent_quarantine_pod_sampling_drawers.stl",
        min_triangles: 600,
        min_size_mm: [390.0, 280.0, 110.0],
    },
    ExpectedOutput {
        generator: "media_reagent_quarantine_pod",
        path: "output/media_reagent_quarantine_pod_spill_waste_capture.stl",
        min_triangles: 500,
        min_size_mm: [1050.0, 600.0, 50.0],
    },
    ExpectedOutput {
        generator: "media_reagent_quarantine_pod",
        path: "output/media_reagent_quarantine_pod_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [1200.0, 730.0, 1350.0],
    },
    ExpectedOutput {
        generator: "closed_isolator_workcell",
        path: "output/closed_isolator_workcell_shell.stl",
        min_triangles: 200,
        min_size_mm: [2300.0, 1050.0, 1700.0],
    },
    ExpectedOutput {
        generator: "closed_isolator_workcell",
        path: "output/closed_isolator_workcell_hepa_plenum.stl",
        min_triangles: 80,
        min_size_mm: [2200.0, 950.0, 250.0],
    },
    ExpectedOutput {
        generator: "closed_isolator_workcell",
        path: "output/closed_isolator_workcell_assembly.stl",
        min_triangles: 500,
        min_size_mm: [2450.0, 1200.0, 1750.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_service_module",
        path: "output/inline_sensor_service_module_baseplate.stl",
        min_triangles: 180,
        min_size_mm: [400.0, 190.0, 35.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_service_module",
        path: "output/inline_sensor_service_module_manifold.stl",
        min_triangles: 220,
        min_size_mm: [320.0, 45.0, 35.0],
    },
    ExpectedOutput {
        generator: "inline_sensor_service_module",
        path: "output/inline_sensor_service_module_assembly.stl",
        min_triangles: 500,
        min_size_mm: [400.0, 190.0, 90.0],
    },
    ExpectedOutput {
        generator: "cassette_sensor_backplane",
        path: "output/cassette_sensor_backplane_substrate.stl",
        min_triangles: 500,
        min_size_mm: [620.0, 540.0, 5.0],
    },
    ExpectedOutput {
        generator: "cassette_sensor_backplane",
        path: "output/cassette_sensor_backplane_spring_pin_carrier.stl",
        min_triangles: 1000,
        min_size_mm: [600.0, 520.0, 20.0],
    },
    ExpectedOutput {
        generator: "cassette_sensor_backplane",
        path: "output/cassette_sensor_backplane_assembly.stl",
        min_triangles: 2000,
        min_size_mm: [620.0, 600.0, 35.0],
    },
    ExpectedOutput {
        generator: "automated_culture_imaging_module",
        path: "output/automated_culture_imaging_module_enclosure.stl",
        min_triangles: 300,
        min_size_mm: [1100.0, 800.0, 500.0],
    },
    ExpectedOutput {
        generator: "automated_culture_imaging_module",
        path: "output/automated_culture_imaging_module_datum_nest.stl",
        min_triangles: 500,
        min_size_mm: [700.0, 630.0, 45.0],
    },
    ExpectedOutput {
        generator: "automated_culture_imaging_module",
        path: "output/automated_culture_imaging_module_motion_gantry.stl",
        min_triangles: 400,
        min_size_mm: [850.0, 680.0, 80.0],
    },
    ExpectedOutput {
        generator: "automated_culture_imaging_module",
        path: "output/automated_culture_imaging_module_assembly.stl",
        min_triangles: 1500,
        min_size_mm: [1100.0, 800.0, 530.0],
    },
    ExpectedOutput {
        generator: "environmental_utility_skid",
        path: "output/environmental_utility_skid_frame.stl",
        min_triangles: 120,
        min_size_mm: [1200.0, 500.0, 1600.0],
    },
    ExpectedOutput {
        generator: "environmental_utility_skid",
        path: "output/environmental_utility_skid_gas_panel.stl",
        min_triangles: 300,
        min_size_mm: [500.0, 30.0, 600.0],
    },
    ExpectedOutput {
        generator: "environmental_utility_skid",
        path: "output/environmental_utility_skid_assembly.stl",
        min_triangles: 800,
        min_size_mm: [1200.0, 600.0, 1600.0],
    },
    ExpectedOutput {
        generator: "aseptic_transfer_hatch",
        path: "output/aseptic_transfer_hatch_body.stl",
        min_triangles: 150,
        min_size_mm: [700.0, 350.0, 450.0],
    },
    ExpectedOutput {
        generator: "aseptic_transfer_hatch",
        path: "output/aseptic_transfer_hatch_rtp_alpha_placeholders.stl",
        min_triangles: 400,
        min_size_mm: [550.0, 25.0, 270.0],
    },
    ExpectedOutput {
        generator: "aseptic_transfer_hatch",
        path: "output/aseptic_transfer_hatch_assembly.stl",
        min_triangles: 700,
        min_size_mm: [700.0, 400.0, 500.0],
    },
    ExpectedOutput {
        generator: "clean_support_pod_shell",
        path: "output/clean_support_pod_shell_floor_and_zones.stl",
        min_triangles: 40,
        min_size_mm: [3500.0, 2100.0, 10.0],
    },
    ExpectedOutput {
        generator: "clean_support_pod_shell",
        path: "output/clean_support_pod_shell_modular_panels.stl",
        min_triangles: 80,
        min_size_mm: [3500.0, 2000.0, 1800.0],
    },
    ExpectedOutput {
        generator: "clean_support_pod_shell",
        path: "output/clean_support_pod_shell_assembly.stl",
        min_triangles: 500,
        min_size_mm: [3500.0, 2100.0, 1900.0],
    },
    ExpectedOutput {
        generator: "sterility_validation_challenge_rack",
        path: "output/sterility_validation_challenge_rack_base.stl",
        min_triangles: 160,
        min_size_mm: [700.0, 500.0, 45.0],
    },
    ExpectedOutput {
        generator: "sterility_validation_challenge_rack",
        path: "output/sterility_validation_challenge_rack_coupon_carrier.stl",
        min_triangles: 180,
        min_size_mm: [280.0, 60.0, 200.0],
    },
    ExpectedOutput {
        generator: "sterility_validation_challenge_rack",
        path: "output/sterility_validation_challenge_rack_media_fill_tray.stl",
        min_triangles: 250,
        min_size_mm: [480.0, 190.0, 45.0],
    },
    ExpectedOutput {
        generator: "sterility_validation_challenge_rack",
        path: "output/sterility_validation_challenge_rack_assembly.stl",
        min_triangles: 700,
        min_size_mm: [700.0, 530.0, 220.0],
    },
    ExpectedOutput {
        generator: "sterile_consumable_cartridge_hotel",
        path: "output/sterile_consumable_cartridge_hotel_base_tray.stl",
        min_triangles: 300,
        min_size_mm: [710.0, 520.0, 50.0],
    },
    ExpectedOutput {
        generator: "sterile_consumable_cartridge_hotel",
        path: "output/sterile_consumable_cartridge_hotel_clean_shelf_stack.stl",
        min_triangles: 600,
        min_size_mm: [710.0, 490.0, 520.0],
    },
    ExpectedOutput {
        generator: "sterile_consumable_cartridge_hotel",
        path: "output/sterile_consumable_cartridge_hotel_used_return_shelf.stl",
        min_triangles: 400,
        min_size_mm: [300.0, 360.0, 120.0],
    },
    ExpectedOutput {
        generator: "sterile_consumable_cartridge_hotel",
        path: "output/sterile_consumable_cartridge_hotel_assembly.stl",
        min_triangles: 1800,
        min_size_mm: [1000.0, 1000.0, 550.0],
    },
    ExpectedOutput {
        generator: "waste_decon_service_pod",
        path: "output/waste_decon_service_pod_frame.stl",
        min_triangles: 400,
        min_size_mm: [900.0, 540.0, 880.0],
    },
    ExpectedOutput {
        generator: "waste_decon_service_pod",
        path: "output/waste_decon_service_pod_secondary_containment.stl",
        min_triangles: 300,
        min_size_mm: [800.0, 450.0, 55.0],
    },
    ExpectedOutput {
        generator: "waste_decon_service_pod",
        path: "output/waste_decon_service_pod_liquid_waste_cassette.stl",
        min_triangles: 700,
        min_size_mm: [540.0, 250.0, 300.0],
    },
    ExpectedOutput {
        generator: "waste_decon_service_pod",
        path: "output/waste_decon_service_pod_filtered_vent_stack.stl",
        min_triangles: 600,
        min_size_mm: [450.0, 70.0, 150.0],
    },
    ExpectedOutput {
        generator: "waste_decon_service_pod",
        path: "output/waste_decon_service_pod_assembly.stl",
        min_triangles: 2500,
        min_size_mm: [900.0, 540.0, 890.0],
    },
    ExpectedOutput {
        generator: "closed_media_fill_run_simulation_fixture",
        path: "output/closed_media_fill_run_simulation_fixture_baseplate.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_fill_run_simulation_fixture",
        path: "output/closed_media_fill_run_simulation_fixture_sterile_connector_loop_routing.stl",
        min_triangles: 80,
        min_size_mm: [180.0, 80.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_media_fill_run_simulation_fixture",
        path: "output/closed_media_fill_run_simulation_fixture_assembly.stl",
        min_triangles: 400,
        min_size_mm: [300.0, 200.0, 60.0],
    },
    ExpectedOutput {
        generator: "closed_incubation_slot_map_verification_station",
        path: "output/closed_incubation_slot_map_verification_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubation_slot_map_verification_station",
        path: "output/closed_incubation_slot_map_verification_station_six_slot_rack_comb.stl",
        min_triangles: 80,
        min_size_mm: [180.0, 120.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_incubation_slot_map_verification_station",
        path: "output/closed_incubation_slot_map_verification_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_sterile_connector_lot_inspection_station",
        path: "output/closed_sterile_connector_lot_inspection_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sterile_connector_lot_inspection_station",
        path: "output/closed_sterile_connector_lot_inspection_station_gonogo_gauge_pockets.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 60.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sterile_connector_lot_inspection_station",
        path: "output/closed_sterile_connector_lot_inspection_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_gasket_lot_incoming_inspection_station",
        path: "output/closed_gasket_lot_incoming_inspection_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_gasket_lot_incoming_inspection_station",
        path: "output/closed_gasket_lot_incoming_inspection_station_defect_imaging_bridge.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 30.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_gasket_lot_incoming_inspection_station",
        path: "output/closed_gasket_lot_incoming_inspection_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_waste_bag_custody_weigh_seal_station",
        path: "output/closed_waste_bag_custody_weigh_seal_station_base_leak_drip_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_waste_bag_custody_weigh_seal_station",
        path: "output/closed_waste_bag_custody_weigh_seal_station_gravimetric_pad.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_waste_bag_custody_weigh_seal_station",
        path: "output/closed_waste_bag_custody_weigh_seal_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "sterile_tubing_weld_seal_coupon_test_station",
        path: "output/sterile_tubing_weld_seal_coupon_test_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "sterile_tubing_weld_seal_coupon_test_station",
        path: "output/sterile_tubing_weld_seal_coupon_test_station_pressure_decay_ports.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 30.0, 10.0],
    },
    ExpectedOutput {
        generator: "sterile_tubing_weld_seal_coupon_test_station",
        path: "output/sterile_tubing_weld_seal_coupon_test_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_robot_end_effector_sterility_parking_station",
        path: "output/closed_robot_end_effector_sterility_parking_station_base_lane_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_robot_end_effector_sterility_parking_station",
        path: "output/closed_robot_end_effector_sterility_parking_station_covered_tool_nests.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_robot_end_effector_sterility_parking_station",
        path: "output/closed_robot_end_effector_sterility_parking_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 180.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_hepa_filter_scan_adapter_fixture",
        path: "output/closed_hepa_filter_scan_adapter_fixture_cleanable_base_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_hepa_filter_scan_adapter_fixture",
        path: "output/closed_hepa_filter_scan_adapter_fixture_scan_wand_rails.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_hepa_filter_scan_adapter_fixture",
        path: "output/closed_hepa_filter_scan_adapter_fixture_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_culture_module_power_data_quickconnect_validation_station",
        path: "output/closed_culture_module_power_data_quickconnect_validation_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_culture_module_power_data_quickconnect_validation_station",
        path: "output/closed_culture_module_power_data_quickconnect_validation_station_continuity_pogo_fixture.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 50.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_culture_module_power_data_quickconnect_validation_station",
        path: "output/closed_culture_module_power_data_quickconnect_validation_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_aseptic_tool_surface_bioburden_sampling_station",
        path: "output/closed_aseptic_tool_surface_bioburden_sampling_station_base_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_aseptic_tool_surface_bioburden_sampling_station",
        path: "output/closed_aseptic_tool_surface_bioburden_sampling_station_swab_contact_plate_holders.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 60.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_aseptic_tool_surface_bioburden_sampling_station",
        path: "output/closed_aseptic_tool_surface_bioburden_sampling_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_condensate_drain_validation_fixture",
        path: "output/closed_incubator_condensate_drain_validation_fixture_base_spill_pan.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_condensate_drain_validation_fixture",
        path: "output/closed_incubator_condensate_drain_validation_fixture_sloped_drain_coupon_tray.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 60.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_condensate_drain_validation_fixture",
        path: "output/closed_incubator_condensate_drain_validation_fixture_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_sample_label_print_apply_verify_station",
        path: "output/closed_sample_label_print_apply_verify_station_cleanable_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sample_label_print_apply_verify_station",
        path: "output/closed_sample_label_print_apply_verify_station_barcode_rfid_verifier_camera_bridge.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 30.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_sample_label_print_apply_verify_station",
        path: "output/closed_sample_label_print_apply_verify_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_evaporation_mass_loss_mapping_station",
        path: "output/closed_cassette_evaporation_mass_loss_mapping_station_base_pan.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_evaporation_mass_loss_mapping_station",
        path: "output/closed_cassette_evaporation_mass_loss_mapping_station_weighed_surrogate_cassette_nests.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 80.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_evaporation_mass_loss_mapping_station",
        path: "output/closed_cassette_evaporation_mass_loss_mapping_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_shear_stress_surrogate_chip_station",
        path: "output/closed_perfusion_shear_stress_surrogate_chip_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_shear_stress_surrogate_chip_station",
        path: "output/closed_perfusion_shear_stress_surrogate_chip_station_dummy_restriction_chip_carrier.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_shear_stress_surrogate_chip_station",
        path: "output/closed_perfusion_shear_stress_surrogate_chip_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_chain_of_custody_sample_tote_station",
        path: "output/closed_chain_of_custody_sample_tote_station_leak_drip_tray_base.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_chain_of_custody_sample_tote_station",
        path: "output/closed_chain_of_custody_sample_tote_station_sealed_tote_receiver.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 80.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_chain_of_custody_sample_tote_station",
        path: "output/closed_chain_of_custody_sample_tote_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_cell_density_viability_sampling_station",
        path: "output/closed_cell_density_viability_sampling_station_base_enclosure.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cell_density_viability_sampling_station",
        path: "output/closed_cell_density_viability_sampling_station_closed_sample_loop_manifold.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_cell_density_viability_sampling_station",
        path: "output/closed_cell_density_viability_sampling_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_microfluidic_chip_preflush_debubble_station",
        path: "output/closed_microfluidic_chip_preflush_debubble_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_microfluidic_chip_preflush_debubble_station",
        path: "output/closed_microfluidic_chip_preflush_debubble_station_vertical_bubble_trap_tower.stl",
        min_triangles: 80,
        min_size_mm: [40.0, 40.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_microfluidic_chip_preflush_debubble_station",
        path: "output/closed_microfluidic_chip_preflush_debubble_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_media_osmolality_conductivity_qc_station",
        path: "output/closed_media_osmolality_conductivity_qc_station_base_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_osmolality_conductivity_qc_station",
        path: "output/closed_media_osmolality_conductivity_qc_station_sterile_sample_loop_manifold.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_media_osmolality_conductivity_qc_station",
        path: "output/closed_media_osmolality_conductivity_qc_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_media_buffer_hold_time_stability_station",
        path: "output/closed_media_buffer_hold_time_stability_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_buffer_hold_time_stability_station",
        path: "output/closed_media_buffer_hold_time_stability_station_timepoint_sample_loop.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_buffer_hold_time_stability_station",
        path: "output/closed_media_buffer_hold_time_stability_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_inline_filter_preuse_integrity_test_station",
        path: "output/closed_inline_filter_preuse_integrity_test_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_inline_filter_preuse_integrity_test_station",
        path: "output/closed_inline_filter_preuse_integrity_test_station_filter_cartridge_nests.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 80.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_inline_filter_preuse_integrity_test_station",
        path: "output/closed_inline_filter_preuse_integrity_test_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_growth_factor_cytokine_addition_qc_station",
        path: "output/closed_growth_factor_cytokine_addition_qc_station_chilled_spill_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_growth_factor_cytokine_addition_qc_station",
        path: "output/closed_growth_factor_cytokine_addition_qc_station_chilled_additive_nests.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 80.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_growth_factor_cytokine_addition_qc_station",
        path: "output/closed_growth_factor_cytokine_addition_qc_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_chip_inlet_outlet_dead_volume_dye_recovery_station",
        path: "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_chip_inlet_outlet_dead_volume_dye_recovery_station",
        path: "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_chip_cassette_surrogate.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_chip_inlet_outlet_dead_volume_dye_recovery_station",
        path: "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_teer_impedance_phantom_verification_station",
        path: "output/closed_teer_impedance_phantom_verification_station_base_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_teer_impedance_phantom_verification_station",
        path: "output/closed_teer_impedance_phantom_verification_station_phantom_cartridge_nests.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 80.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_teer_impedance_phantom_verification_station",
        path: "output/closed_teer_impedance_phantom_verification_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_reference_particle_flow_calibration_station",
        path: "output/closed_reference_particle_flow_calibration_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reference_particle_flow_calibration_station",
        path: "output/closed_reference_particle_flow_calibration_station_sterile_injection_loop.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_reference_particle_flow_calibration_station",
        path: "output/closed_reference_particle_flow_calibration_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_pump_tubing_occlusion_endurance_station",
        path: "output/closed_pump_tubing_occlusion_endurance_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_pump_tubing_occlusion_endurance_station",
        path: "output/closed_pump_tubing_occlusion_endurance_station_tubing_loop_strain_relief_combs.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 40.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_pump_tubing_occlusion_endurance_station",
        path: "output/closed_pump_tubing_occlusion_endurance_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_sampling_carryover_flush_validation_station",
        path: "output/closed_sampling_carryover_flush_validation_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sampling_carryover_flush_validation_station",
        path: "output/closed_sampling_carryover_flush_validation_station_closed_sample_loop_cartridge_nests.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_sampling_carryover_flush_validation_station",
        path: "output/closed_sampling_carryover_flush_validation_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_low_volume_dose_accuracy_recovery_station",
        path: "output/closed_low_volume_dose_accuracy_recovery_station_base_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_low_volume_dose_accuracy_recovery_station",
        path: "output/closed_low_volume_dose_accuracy_recovery_station_micro_dose_collection_nests.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 40.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_low_volume_dose_accuracy_recovery_station",
        path: "output/closed_low_volume_dose_accuracy_recovery_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_additive_trace_dispense_verification_station",
        path: "output/closed_reagent_additive_trace_dispense_verification_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_additive_trace_dispense_verification_station",
        path: "output/closed_reagent_additive_trace_dispense_verification_station_small_volume_dispense_wells.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 40.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_additive_trace_dispense_verification_station",
        path: "output/closed_reagent_additive_trace_dispense_verification_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_optical_focus_drift_calibration_station",
        path: "output/closed_optical_focus_drift_calibration_station_base_enclosure.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_optical_focus_drift_calibration_station",
        path: "output/closed_optical_focus_drift_calibration_station_focus_step_phantom_blocks.stl",
        min_triangles: 80,
        min_size_mm: [80.0, 30.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_optical_focus_drift_calibration_station",
        path: "output/closed_optical_focus_drift_calibration_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_reference_viscosity_media_rheology_station",
        path: "output/closed_reference_viscosity_media_rheology_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reference_viscosity_media_rheology_station",
        path: "output/closed_reference_viscosity_media_rheology_station_capillary_restriction_coupon_holder.stl",
        min_triangles: 80,
        min_size_mm: [80.0, 40.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reference_viscosity_media_rheology_station",
        path: "output/closed_reference_viscosity_media_rheology_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [300.0, 200.0, 50.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_particle_counter_probe_calibration_station",
        path: "output/closed_environmental_particle_counter_probe_calibration_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [900.0, 500.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_particle_counter_probe_calibration_station",
        path: "output/closed_environmental_particle_counter_probe_calibration_station_clean_probe_parking_cradle.stl",
        min_triangles: 80,
        min_size_mm: [200.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_environmental_particle_counter_probe_calibration_station",
        path: "output/closed_environmental_particle_counter_probe_calibration_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [900.0, 500.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_dewpoint_reference_calibration_station",
        path: "output/closed_humidity_dewpoint_reference_calibration_station_base_tray.stl",
        min_triangles: 80,
        min_size_mm: [650.0, 400.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_dewpoint_reference_calibration_station",
        path: "output/closed_humidity_dewpoint_reference_calibration_station_probe_nest_plate.stl",
        min_triangles: 80,
        min_size_mm: [250.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_dewpoint_reference_calibration_station",
        path: "output/closed_humidity_dewpoint_reference_calibration_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [650.0, 400.0, 60.0],
    },
    ExpectedOutput {
        generator: "closed_module_latch_force_alignment_verification_station",
        path: "output/closed_module_latch_force_alignment_verification_station_base_deck.stl",
        min_triangles: 80,
        min_size_mm: [800.0, 600.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_module_latch_force_alignment_verification_station",
        path: "output/closed_module_latch_force_alignment_verification_station_datum_nest.stl",
        min_triangles: 80,
        min_size_mm: [450.0, 300.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_module_latch_force_alignment_verification_station",
        path: "output/closed_module_latch_force_alignment_verification_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [800.0, 600.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_waste_line_backflow_siphon_validation_station",
        path: "output/closed_waste_line_backflow_siphon_validation_station_leak_tray_base.stl",
        min_triangles: 80,
        min_size_mm: [900.0, 500.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_waste_line_backflow_siphon_validation_station",
        path: "output/closed_waste_line_backflow_siphon_validation_station_waste_bag_tube_nest.stl",
        min_triangles: 80,
        min_size_mm: [250.0, 180.0, 40.0],
    },
    ExpectedOutput {
        generator: "closed_waste_line_backflow_siphon_validation_station",
        path: "output/closed_waste_line_backflow_siphon_validation_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [900.0, 500.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_gas_filter_integrity_leak_test_station",
        path: "output/closed_gas_filter_integrity_leak_test_station_base_containment_tray.stl",
        min_triangles: 80,
        min_size_mm: [800.0, 450.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_gas_filter_integrity_leak_test_station",
        path: "output/closed_gas_filter_integrity_leak_test_station_filter_cartridge_nests.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 40.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_gas_filter_integrity_leak_test_station",
        path: "output/closed_gas_filter_integrity_leak_test_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [800.0, 450.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_gasket_compression_set_aging_station",
        path: "output/closed_cassette_gasket_compression_set_aging_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [900.0, 600.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_gasket_compression_set_aging_station",
        path: "output/closed_cassette_gasket_compression_set_aging_station_gasket_sample_nests.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_gasket_compression_set_aging_station",
        path: "output/closed_cassette_gasket_compression_set_aging_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [900.0, 600.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_module_port_dry_break_cycle_life_station",
        path: "output/closed_module_port_dry_break_cycle_life_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [1100.0, 700.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_module_port_dry_break_cycle_life_station",
        path: "output/closed_module_port_dry_break_cycle_life_station_male_coupler_nests.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 120.0, 25.0],
    },
    ExpectedOutput {
        generator: "closed_module_port_dry_break_cycle_life_station",
        path: "output/closed_module_port_dry_break_cycle_life_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1200.0, 750.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_gas_mfc_zero_span_crosscheck_station",
        path: "output/closed_gas_mfc_zero_span_crosscheck_station_base_tray.stl",
        min_triangles: 80,
        min_size_mm: [1000.0, 740.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_gas_mfc_zero_span_crosscheck_station",
        path: "output/closed_gas_mfc_zero_span_crosscheck_station_mfc_pocket_array.stl",
        min_triangles: 80,
        min_size_mm: [480.0, 120.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_gas_mfc_zero_span_crosscheck_station",
        path: "output/closed_gas_mfc_zero_span_crosscheck_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1050.0, 760.0, 150.0],
    },
    ExpectedOutput {
        generator: "closed_uv_h2o2_decon_shadow_mapping_coupon_station",
        path: "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_base_deck.stl",
        min_triangles: 80,
        min_size_mm: [1200.0, 700.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_uv_h2o2_decon_shadow_mapping_coupon_station",
        path: "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_coupon_grid_carrier.stl",
        min_triangles: 80,
        min_size_mm: [400.0, 250.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_uv_h2o2_decon_shadow_mapping_coupon_station",
        path: "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1200.0, 700.0, 180.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_position_environmental_equivalence_station",
        path: "output/closed_cassette_position_environmental_equivalence_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [1200.0, 760.0, 18.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_position_environmental_equivalence_station",
        path: "output/closed_cassette_position_environmental_equivalence_station_cassette_surrogate_nest.stl",
        min_triangles: 80,
        min_size_mm: [300.0, 200.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_position_environmental_equivalence_station",
        path: "output/closed_cassette_position_environmental_equivalence_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1200.0, 760.0, 120.0],
    },
    ExpectedOutput {
        generator: "closed_module_thermal_contact_resistance_mapping_station",
        path: "output/closed_module_thermal_contact_resistance_mapping_station_base_deck.stl",
        min_triangles: 80,
        min_size_mm: [1050.0, 680.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_module_thermal_contact_resistance_mapping_station",
        path: "output/closed_module_thermal_contact_resistance_mapping_station_cassette_module_datum_nest.stl",
        min_triangles: 80,
        min_size_mm: [250.0, 180.0, 25.0],
    },
    ExpectedOutput {
        generator: "closed_module_thermal_contact_resistance_mapping_station",
        path: "output/closed_module_thermal_contact_resistance_mapping_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1050.0, 680.0, 120.0],
    },
    ExpectedOutput {
        generator: "closed_waste_container_neutralization_contact_time_station",
        path: "output/closed_waste_container_neutralization_contact_time_station_secondary_containment_tray.stl",
        min_triangles: 80,
        min_size_mm: [1200.0, 800.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_waste_container_neutralization_contact_time_station",
        path: "output/closed_waste_container_neutralization_contact_time_station_waste_container_cradle.stl",
        min_triangles: 80,
        min_size_mm: [450.0, 250.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_waste_container_neutralization_contact_time_station",
        path: "output/closed_waste_container_neutralization_contact_time_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1200.0, 820.0, 200.0],
    },
    ExpectedOutput {
        generator: "closed_supply_cylinder_lot_changeover_custody_station",
        path: "output/closed_supply_cylinder_lot_changeover_custody_station_base_leak_tray.stl",
        min_triangles: 80,
        min_size_mm: [1350.0, 800.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_supply_cylinder_lot_changeover_custody_station",
        path: "output/closed_supply_cylinder_lot_changeover_custody_station_cylinder_restraints.stl",
        min_triangles: 80,
        min_size_mm: [1000.0, 250.0, 120.0],
    },
    ExpectedOutput {
        generator: "closed_supply_cylinder_lot_changeover_custody_station",
        path: "output/closed_supply_cylinder_lot_changeover_custody_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1350.0, 800.0, 350.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_rack_vibration_tilt_mapping_station",
        path: "output/closed_incubator_rack_vibration_tilt_mapping_station_deck.stl",
        min_triangles: 80,
        min_size_mm: [1000.0, 700.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_rack_vibration_tilt_mapping_station",
        path: "output/closed_incubator_rack_vibration_tilt_mapping_station_rack_datum_plate.stl",
        min_triangles: 80,
        min_size_mm: [700.0, 380.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_rack_vibration_tilt_mapping_station",
        path: "output/closed_incubator_rack_vibration_tilt_mapping_station_assembly.stl",
        min_triangles: 350,
        min_size_mm: [1000.0, 700.0, 120.0],
    },
    ExpectedOutput {
        generator: "closed_media_bag_load_cell_drift_reference_station",
        path: "output/closed_media_bag_load_cell_drift_reference_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [200.0, 150.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_bag_load_cell_drift_reference_station",
        path: "output/closed_media_bag_load_cell_drift_reference_station_load_cell_bridge.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 30.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_media_bag_load_cell_drift_reference_station",
        path: "output/closed_media_bag_load_cell_drift_reference_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_robot_tool_tip_runout_repeatability_station",
        path: "output/closed_robot_tool_tip_runout_repeatability_station_base_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_robot_tool_tip_runout_repeatability_station",
        path: "output/closed_robot_tool_tip_runout_repeatability_station_kinematic_tool_nest.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 40.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_robot_tool_tip_runout_repeatability_station",
        path: "output/closed_robot_tool_tip_runout_repeatability_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 60.0],
    },
    ExpectedOutput {
        generator: "closed_liquid_waste_aerosol_trap_integrity_station",
        path: "output/closed_liquid_waste_aerosol_trap_integrity_station_leak_basin_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_liquid_waste_aerosol_trap_integrity_station",
        path: "output/closed_liquid_waste_aerosol_trap_integrity_station_closed_waste_bottle_nest.stl",
        min_triangles: 40,
        min_size_mm: [60.0, 60.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_liquid_waste_aerosol_trap_integrity_station",
        path: "output/closed_liquid_waste_aerosol_trap_integrity_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_service_bulkhead_misconnection_gauge_station",
        path: "output/closed_service_bulkhead_misconnection_gauge_station_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_service_bulkhead_misconnection_gauge_station",
        path: "output/closed_service_bulkhead_misconnection_gauge_station_keyed_bulkhead_mockup.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 30.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_service_bulkhead_misconnection_gauge_station",
        path: "output/closed_service_bulkhead_misconnection_gauge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_door_recovery_challenge_station",
        path: "output/closed_incubator_door_recovery_challenge_station_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_door_recovery_challenge_station",
        path: "output/closed_incubator_door_recovery_challenge_station_sealed_cassette_surrogate_rack.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_door_recovery_challenge_station",
        path: "output/closed_incubator_door_recovery_challenge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_internal_surface_cleanability_coupon_station",
        path: "output/closed_incubator_internal_surface_cleanability_coupon_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_internal_surface_cleanability_coupon_station",
        path: "output/closed_incubator_internal_surface_cleanability_coupon_station_removable_wall_coupon_panel.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 30.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_internal_surface_cleanability_coupon_station",
        path: "output/closed_incubator_internal_surface_cleanability_coupon_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_sterile_connector_actuation_force_life_station",
        path: "output/closed_sterile_connector_actuation_force_life_station_base_leak_tray_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sterile_connector_actuation_force_life_station",
        path: "output/closed_sterile_connector_actuation_force_life_station_connector_nest_arrays.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 30.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_sterile_connector_actuation_force_life_station",
        path: "output/closed_sterile_connector_actuation_force_life_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_cable_feedthrough_leak_test_station",
        path: "output/closed_sensor_cable_feedthrough_leak_test_station_base_containment_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_cable_feedthrough_leak_test_station",
        path: "output/closed_sensor_cable_feedthrough_leak_test_station_wall_feedthrough_coupon_panel.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 30.0, 15.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_cable_feedthrough_leak_test_station",
        path: "output/closed_sensor_cable_feedthrough_leak_test_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_o2_co2_stratification_mapping_station",
        path: "output/closed_incubator_o2_co2_stratification_mapping_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_o2_co2_stratification_mapping_station",
        path: "output/closed_incubator_o2_co2_stratification_mapping_station_rack_slot_surrogate_fixture.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_o2_co2_stratification_mapping_station",
        path: "output/closed_incubator_o2_co2_stratification_mapping_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_module_thermal_runaway_fault_injection_station",
        path: "output/closed_module_thermal_runaway_fault_injection_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_module_thermal_runaway_fault_injection_station",
        path: "output/closed_module_thermal_runaway_fault_injection_station_sealed_module_dummy_nest.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_module_thermal_runaway_fault_injection_station",
        path: "output/closed_module_thermal_runaway_fault_injection_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 80.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_transport_drop_shock_inspection_station",
        path: "output/closed_cassette_transport_drop_shock_inspection_station_base_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_transport_drop_shock_inspection_station",
        path: "output/closed_cassette_transport_drop_shock_inspection_station_cassette_receiving_nest.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_transport_drop_shock_inspection_station",
        path: "output/closed_cassette_transport_drop_shock_inspection_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_condensate_material_compatibility_station",
        path: "output/closed_humidity_condensate_material_compatibility_station_base_containment_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_condensate_material_compatibility_station",
        path: "output/closed_humidity_condensate_material_compatibility_station_coupon_rack_cassettes.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_condensate_material_compatibility_station",
        path: "output/closed_humidity_condensate_material_compatibility_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_culture_module_service_line_kink_occlusion_station",
        path: "output/closed_culture_module_service_line_kink_occlusion_station_base_leak_tray_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_culture_module_service_line_kink_occlusion_station",
        path: "output/closed_culture_module_service_line_kink_occlusion_station_service_bulkhead_coupon_nest.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_culture_module_service_line_kink_occlusion_station",
        path: "output/closed_culture_module_service_line_kink_occlusion_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_inline_sensor_calibration_drift_challenge_station",
        path: "output/closed_inline_sensor_calibration_drift_challenge_station_base_leak_tray_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_inline_sensor_calibration_drift_challenge_station",
        path: "output/closed_inline_sensor_calibration_drift_challenge_station_sensor_cartridge_nest_array.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_inline_sensor_calibration_drift_challenge_station",
        path: "output/closed_inline_sensor_calibration_drift_challenge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_multichannel_pressure_decay_leak_reference_station",
        path: "output/closed_multichannel_pressure_decay_leak_reference_station_base_containment_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_multichannel_pressure_decay_leak_reference_station",
        path: "output/closed_multichannel_pressure_decay_leak_reference_station_20_channel_reference_manifold_coupon.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_multichannel_pressure_decay_leak_reference_station",
        path: "output/closed_multichannel_pressure_decay_leak_reference_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_media_reservoir_mixing_homogeneity_validation_station",
        path: "output/closed_media_reservoir_mixing_homogeneity_validation_station_base_leak_tray_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_reservoir_mixing_homogeneity_validation_station",
        path: "output/closed_media_reservoir_mixing_homogeneity_validation_station_sealed_reservoir_bag_cradle.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_media_reservoir_mixing_homogeneity_validation_station",
        path: "output/closed_media_reservoir_mixing_homogeneity_validation_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_pump_cross_channel_crosstalk_station",
        path: "output/closed_perfusion_pump_cross_channel_crosstalk_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_pump_cross_channel_crosstalk_station",
        path: "output/closed_perfusion_pump_cross_channel_crosstalk_station_multi_channel_pump_mounts.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_pump_cross_channel_crosstalk_station",
        path: "output/closed_perfusion_pump_cross_channel_crosstalk_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_robot_collision_force_limit_validation_station",
        path: "output/closed_robot_collision_force_limit_validation_station_base_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_robot_collision_force_limit_validation_station",
        path: "output/closed_robot_collision_force_limit_validation_station_force_gauge_targets.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_robot_collision_force_limit_validation_station",
        path: "output/closed_robot_collision_force_limit_validation_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_humidity_recovery_load_challenge_station",
        path: "output/closed_incubator_humidity_recovery_load_challenge_station_base_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_humidity_recovery_load_challenge_station",
        path: "output/closed_incubator_humidity_recovery_load_challenge_station_cassette_surrogate_load_rack.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_humidity_recovery_load_challenge_station",
        path: "output/closed_incubator_humidity_recovery_load_challenge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_airlock_pressure_decay_interlock_challenge_station",
        path: "output/closed_airlock_pressure_decay_interlock_challenge_station_base_validation_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_airlock_pressure_decay_interlock_challenge_station",
        path: "output/closed_airlock_pressure_decay_interlock_challenge_station_door_surrogate_plates.stl",
        min_triangles: 40,
        min_size_mm: [80.0, 60.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_airlock_pressure_decay_interlock_challenge_station",
        path: "output/closed_airlock_pressure_decay_interlock_challenge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_barcode_rfid_mismatch_reconciliation_station",
        path: "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_cleanable_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_barcode_rfid_mismatch_reconciliation_station",
        path: "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_sixteen_slot_cassette_tray.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_barcode_rfid_mismatch_reconciliation_station",
        path: "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_cleanroom_gowning_material_shedding_surrogate_station",
        path: "output/closed_cleanroom_gowning_material_shedding_surrogate_station_base_particle_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cleanroom_gowning_material_shedding_surrogate_station",
        path: "output/closed_cleanroom_gowning_material_shedding_surrogate_station_sample_clamp_rails.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 60.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_cleanroom_gowning_material_shedding_surrogate_station",
        path: "output/closed_cleanroom_gowning_material_shedding_surrogate_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_media_reagent_cold_chain_excursion_unpack_station",
        path: "output/closed_media_reagent_cold_chain_excursion_unpack_station_base_drain_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_reagent_cold_chain_excursion_unpack_station",
        path: "output/closed_media_reagent_cold_chain_excursion_unpack_station_sealed_tote_receiver.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_media_reagent_cold_chain_excursion_unpack_station",
        path: "output/closed_media_reagent_cold_chain_excursion_unpack_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_manifold_residual_volume_drainability_station",
        path: "output/closed_perfusion_manifold_residual_volume_drainability_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_manifold_residual_volume_drainability_station",
        path: "output/closed_perfusion_manifold_residual_volume_drainability_station_manifold_nest.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_manifold_residual_volume_drainability_station",
        path: "output/closed_perfusion_manifold_residual_volume_drainability_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_module_external_surface_disinfectant_contact_time_station",
        path: "output/closed_module_external_surface_disinfectant_contact_time_station_base_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_module_external_surface_disinfectant_contact_time_station",
        path: "output/closed_module_external_surface_disinfectant_contact_time_station_module_cradle.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_module_external_surface_disinfectant_contact_time_station",
        path: "output/closed_module_external_surface_disinfectant_contact_time_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_chip_cassette_condensate_ingress_witness_station",
        path: "output/closed_chip_cassette_condensate_ingress_witness_station_base_containment_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_chip_cassette_condensate_ingress_witness_station",
        path: "output/closed_chip_cassette_condensate_ingress_witness_station_sealed_cassette_nest.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_chip_cassette_condensate_ingress_witness_station",
        path: "output/closed_chip_cassette_condensate_ingress_witness_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_inline_bubble_sensor_false_positive_negative_challenge_station",
        path: "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_base_validation_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_inline_bubble_sensor_false_positive_negative_challenge_station",
        path: "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_transparent_surrogate_channel_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 60.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_inline_bubble_sensor_false_positive_negative_challenge_station",
        path: "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_pass_through_tote_pressure_equalization_flow_balance_station",
        path: "output/closed_pass_through_tote_pressure_equalization_flow_balance_station_base_service_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_pass_through_tote_pressure_equalization_flow_balance_station",
        path: "output/closed_pass_through_tote_pressure_equalization_flow_balance_station_sealed_tote_receiver.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 30.0],
    },
    ExpectedOutput {
        generator: "closed_pass_through_tote_pressure_equalization_flow_balance_station",
        path: "output/closed_pass_through_tote_pressure_equalization_flow_balance_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_bag_pressure_hold_creep_station",
        path: "output/closed_reagent_bag_pressure_hold_creep_station_secondary_containment_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_bag_pressure_hold_creep_station",
        path: "output/closed_reagent_bag_pressure_hold_creep_station_guarded_bag_tray.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_reagent_bag_pressure_hold_creep_station",
        path: "output/closed_reagent_bag_pressure_hold_creep_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_chip_cassette_static_charge_particle_attraction_station",
        path: "output/closed_chip_cassette_static_charge_particle_attraction_station_secondary_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_chip_cassette_static_charge_particle_attraction_station",
        path: "output/closed_chip_cassette_static_charge_particle_attraction_station_cassette_nest.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_chip_cassette_static_charge_particle_attraction_station",
        path: "output/closed_chip_cassette_static_charge_particle_attraction_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_line_air_ingress_microleak_challenge_station",
        path: "output/closed_perfusion_line_air_ingress_microleak_challenge_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_line_air_ingress_microleak_challenge_station",
        path: "output/closed_perfusion_line_air_ingress_microleak_challenge_station_closed_tubing_lane_combs.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 60.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_line_air_ingress_microleak_challenge_station",
        path: "output/closed_perfusion_line_air_ingress_microleak_challenge_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_sampling_valve_carryover_deadleg_station",
        path: "output/closed_sampling_valve_carryover_deadleg_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sampling_valve_carryover_deadleg_station",
        path: "output/closed_sampling_valve_carryover_deadleg_station_valve_coupon_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_sampling_valve_carryover_deadleg_station",
        path: "output/closed_sampling_valve_carryover_deadleg_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_robot_gripper_pad_wear_particle_shedding_station",
        path: "output/closed_robot_gripper_pad_wear_particle_shedding_station_base_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_robot_gripper_pad_wear_particle_shedding_station",
        path: "output/closed_robot_gripper_pad_wear_particle_shedding_station_gripper_pad_coupon_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_robot_gripper_pad_wear_particle_shedding_station",
        path: "output/closed_robot_gripper_pad_wear_particle_shedding_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_condensate_cross_contamination_witness_station",
        path: "output/closed_incubator_condensate_cross_contamination_witness_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_condensate_cross_contamination_witness_station",
        path: "output/closed_incubator_condensate_cross_contamination_witness_station_sealed_cassette_surrogate_grid.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_condensate_cross_contamination_witness_station",
        path: "output/closed_incubator_condensate_cross_contamination_witness_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_reservoir_cap_septum_puncture_leak_life_station",
        path: "output/closed_reservoir_cap_septum_puncture_leak_life_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_reservoir_cap_septum_puncture_leak_life_station",
        path: "output/closed_reservoir_cap_septum_puncture_leak_life_station_reservoir_cap_coupon_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_reservoir_cap_septum_puncture_leak_life_station",
        path: "output/closed_reservoir_cap_septum_puncture_leak_life_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_fan_flow_shadowing_mapping_station",
        path: "output/closed_incubator_fan_flow_shadowing_mapping_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_fan_flow_shadowing_mapping_station",
        path: "output/closed_incubator_fan_flow_shadowing_mapping_station_rack_slot_surrogate_grid.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_fan_flow_shadowing_mapping_station",
        path: "output/closed_incubator_fan_flow_shadowing_mapping_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_media_additive_light_exposure_witness_station",
        path: "output/closed_media_additive_light_exposure_witness_station_base_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_additive_light_exposure_witness_station",
        path: "output/closed_media_additive_light_exposure_witness_station_amber_clear_vial_surrogate_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_media_additive_light_exposure_witness_station",
        path: "output/closed_media_additive_light_exposure_witness_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_shipping_vibration_logger_dock_station",
        path: "output/closed_cassette_shipping_vibration_logger_dock_station_base_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_shipping_vibration_logger_dock_station",
        path: "output/closed_cassette_shipping_vibration_logger_dock_station_cassette_dock.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_cassette_shipping_vibration_logger_dock_station",
        path: "output/closed_cassette_shipping_vibration_logger_dock_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_media_osmolality_evaporation_correlation_station",
        path: "output/closed_media_osmolality_evaporation_correlation_station_secondary_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_media_osmolality_evaporation_correlation_station",
        path: "output/closed_media_osmolality_evaporation_correlation_station_sealed_reservoir_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_media_osmolality_evaporation_correlation_station",
        path: "output/closed_media_osmolality_evaporation_correlation_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_cell_source_vial_thaw_timing_stagger_station",
        path: "output/closed_cell_source_vial_thaw_timing_stagger_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_cell_source_vial_thaw_timing_stagger_station",
        path: "output/closed_cell_source_vial_thaw_timing_stagger_station_timed_vial_nest_ladder.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_cell_source_vial_thaw_timing_stagger_station",
        path: "output/closed_cell_source_vial_thaw_timing_stagger_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_module_luer_lock_misassembly_prevention_station",
        path: "output/closed_module_luer_lock_misassembly_prevention_station_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_module_luer_lock_misassembly_prevention_station",
        path: "output/closed_module_luer_lock_misassembly_prevention_station_keyed_connector_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_module_luer_lock_misassembly_prevention_station",
        path: "output/closed_module_luer_lock_misassembly_prevention_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_probe_cleanability_residue_station",
        path: "output/closed_sensor_probe_cleanability_residue_station_base_leak_safe_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_probe_cleanability_residue_station",
        path: "output/closed_sensor_probe_cleanability_residue_station_probe_nest_array.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_probe_cleanability_residue_station",
        path: "output/closed_sensor_probe_cleanability_residue_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_tubing_set_barcode_length_route_verification_station",
        path: "output/closed_tubing_set_barcode_length_route_verification_station_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_tubing_set_barcode_length_route_verification_station",
        path: "output/closed_tubing_set_barcode_length_route_verification_station_length_gauge_channels.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 60.0, 10.0],
    },
    ExpectedOutput {
        generator: "closed_tubing_set_barcode_length_route_verification_station",
        path: "output/closed_tubing_set_barcode_length_route_verification_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_co2_supply_regulator_drift_changeover_station",
        path: "output/closed_co2_supply_regulator_drift_changeover_station_base_closed_cabinet_leak_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_co2_supply_regulator_drift_changeover_station",
        path: "output/closed_co2_supply_regulator_drift_changeover_station_cylinder_regulator_surrogate_restraints.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_co2_supply_regulator_drift_changeover_station",
        path: "output/closed_co2_supply_regulator_drift_changeover_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_probe_storage_hydration_drift_station",
        path: "output/closed_sensor_probe_storage_hydration_drift_station_base_leak_tray_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_probe_storage_hydration_drift_station",
        path: "output/closed_sensor_probe_storage_hydration_drift_station_hydrated_probe_nest_bank.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_sensor_probe_storage_hydration_drift_station",
        path: "output/closed_sensor_probe_storage_hydration_drift_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_dewpoint_condensation_boundary_map_station",
        path: "output/closed_incubator_dewpoint_condensation_boundary_map_station_base_boundary_map_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_dewpoint_condensation_boundary_map_station",
        path: "output/closed_incubator_dewpoint_condensation_boundary_map_station_thermal_gradient_coupon_rack.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_incubator_dewpoint_condensation_boundary_map_station",
        path: "output/closed_incubator_dewpoint_condensation_boundary_map_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_filter_bypass_relief_validation_station",
        path: "output/closed_perfusion_filter_bypass_relief_validation_station_base_containment_tray.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_filter_bypass_relief_validation_station",
        path: "output/closed_perfusion_filter_bypass_relief_validation_station_filter_cartridge_nests.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_perfusion_filter_bypass_relief_validation_station",
        path: "output/closed_perfusion_filter_bypass_relief_validation_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_water_reservoir_biofilm_witness_station",
        path: "output/closed_humidity_water_reservoir_biofilm_witness_station_base_containment_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_water_reservoir_biofilm_witness_station",
        path: "output/closed_humidity_water_reservoir_biofilm_witness_station_removable_reservoir_surrogate.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_humidity_water_reservoir_biofilm_witness_station",
        path: "output/closed_humidity_water_reservoir_biofilm_witness_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "closed_airlock_vhp_residue_aeration_witness_station",
        path: "output/closed_airlock_vhp_residue_aeration_witness_station_base_validation_deck.stl",
        min_triangles: 40,
        min_size_mm: [250.0, 180.0, 5.0],
    },
    ExpectedOutput {
        generator: "closed_airlock_vhp_residue_aeration_witness_station",
        path: "output/closed_airlock_vhp_residue_aeration_witness_station_closed_transfer_airlock_reference.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 80.0, 20.0],
    },
    ExpectedOutput {
        generator: "closed_airlock_vhp_residue_aeration_witness_station",
        path: "output/closed_airlock_vhp_residue_aeration_witness_station_assembly.stl",
        min_triangles: 120,
        min_size_mm: [250.0, 180.0, 70.0],
    },
    ExpectedOutput {
        generator: "pipette_tip_organizer",
        path: "output/pipette_tip_organizer_pipette_stand.stl",
        min_triangles: 80,
        min_size_mm: [240.0, 80.0, 100.0],
    },
    ExpectedOutput {
        generator: "pipette_tip_organizer",
        path: "output/pipette_tip_organizer_tip_box_tray.stl",
        min_triangles: 40,
        min_size_mm: [280.0, 115.0, 15.0],
    },
    ExpectedOutput {
        generator: "aspirator_waste_trap_holder",
        path: "output/aspirator_waste_trap_base_tray.stl",
        min_triangles: 20,
        min_size_mm: [300.0, 140.0, 10.0],
    },
    ExpectedOutput {
        generator: "aspirator_waste_trap_holder",
        path: "output/aspirator_waste_trap_bottle_cradle.stl",
        min_triangles: 80,
        min_size_mm: [200.0, 80.0, 50.0],
    },
    ExpectedOutput {
        generator: "sample_cold_block",
        path: "output/sample_cold_block.stl",
        min_triangles: 50,
        min_size_mm: [50.0, 30.0, 10.0],
    },
    ExpectedOutput {
        generator: "peltier_reservoir_block",
        path: "output/peltier_reservoir_block.stl",
        min_triangles: 50,
        min_size_mm: [40.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "still_air_box_corner",
        path: "output/still_air_box_corner.stl",
        min_triangles: 20,
        min_size_mm: [20.0, 20.0, 20.0],
    },
    ExpectedOutput {
        generator: "still_air_box_rail",
        path: "output/still_air_box_rail_400.stl",
        min_triangles: 20,
        min_size_mm: [350.0, 5.0, 5.0],
    },
    ExpectedOutput {
        generator: "still_air_box_accessories",
        path: "output/still_air_box_tube_rack.stl",
        min_triangles: 20,
        min_size_mm: [50.0, 20.0, 5.0],
    },
    ExpectedOutput {
        generator: "workstation_enclosure",
        path: "output/workstation_rail_segment.stl",
        min_triangles: 20,
        min_size_mm: [200.0, 5.0, 5.0],
    },
    ExpectedOutput {
        generator: "centrifuge_adapter",
        path: "output/centrifuge_adapter_15ml.stl",
        min_triangles: 30,
        min_size_mm: [13.0, 13.0, 15.0],
    },
];

fn main() {
    let mut failures = Vec::new();

    println!("Cell culture equipment STL verification");
    println!("---------------------------------------");

    for output in OUTPUTS {
        match read_binary_stl(output.path) {
            Ok(stats) => {
                let size = [
                    stats.bbox_max[0] - stats.bbox_min[0],
                    stats.bbox_max[1] - stats.bbox_min[1],
                    stats.bbox_max[2] - stats.bbox_min[2],
                ];

                println!(
                    "{} ({}) triangles={} size=[{:.2}, {:.2}, {:.2}]",
                    output.path, output.generator, stats.triangles, size[0], size[1], size[2],
                );

                if stats.triangles < output.min_triangles {
                    failures.push(format!(
                        "{} has {} triangles, expected at least {}",
                        output.path, stats.triangles, output.min_triangles
                    ));
                }

                for (axis, (&actual, &minimum)) in
                    size.iter().zip(output.min_size_mm.iter()).enumerate()
                {
                    if actual < minimum {
                        failures.push(format!(
                            "{} axis {} is {:.2}mm, expected at least {:.2}mm",
                            output.path, axis, actual, minimum
                        ));
                    }
                }
            }
            Err(error) => failures.push(format!(
                "{} missing or invalid; run `{}` first: {error}",
                output.path, output.generator
            )),
        }
    }

    if !failures.is_empty() {
        eprintln!();
        eprintln!("Verification failed:");
        for failure in failures {
            eprintln!("- {failure}");
        }
        std::process::exit(1);
    }

    println!("All starter cell-culture equipment STL outputs are present and sane.");
}

fn read_binary_stl(path: &str) -> Result<StlStats, String> {
    let bytes = fs::read(Path::new(path)).map_err(|error| format!("failed to read: {error}"))?;
    parse_binary_stl(&bytes)
}

fn parse_binary_stl(bytes: &[u8]) -> Result<StlStats, String> {
    if bytes.len() < 84 {
        return Err(format!(
            "file is too small for binary STL: {} bytes",
            bytes.len()
        ));
    }

    let triangles = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
    if triangles == 0 {
        return Err("binary STL has zero triangles".to_string());
    }

    let expected_len = 84usize + triangles as usize * 50usize;
    if bytes.len() != expected_len {
        return Err(format!(
            "binary STL size mismatch: got {} bytes, expected {} bytes for {} triangles",
            bytes.len(),
            expected_len,
            triangles
        ));
    }

    let mut bbox_min = [f32::INFINITY; 3];
    let mut bbox_max = [f32::NEG_INFINITY; 3];

    for tri in 0..triangles as usize {
        let tri_offset = 84 + tri * 50;
        for vertex_idx in 0..3 {
            let vertex_offset = tri_offset + 12 + vertex_idx * 12;
            let vertex = [
                read_f32(bytes, vertex_offset)?,
                read_f32(bytes, vertex_offset + 4)?,
                read_f32(bytes, vertex_offset + 8)?,
            ];
            for axis in 0..3 {
                if !vertex[axis].is_finite() {
                    return Err(format!("non-finite vertex coordinate in triangle {tri}"));
                }
                bbox_min[axis] = bbox_min[axis].min(vertex[axis]);
                bbox_max[axis] = bbox_max[axis].max(vertex[axis]);
            }
        }
    }

    Ok(StlStats {
        triangles,
        bbox_min,
        bbox_max,
    })
}

fn read_f32(bytes: &[u8], offset: usize) -> Result<f32, String> {
    let raw: [u8; 4] = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| format!("unexpected EOF reading f32 at offset {offset}"))?
        .try_into()
        .unwrap();
    Ok(f32::from_le_bytes(raw))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_too_small_stl() {
        let result = parse_binary_stl(&[0u8; 12]);
        assert!(result.is_err());
    }

    #[test]
    fn required_outputs_include_new_validation_modules() {
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cell_culture_logger_enclosure"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "co2_sensor_service_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "co2_incubator"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "aspirator_waste_trap_holder"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "chip_priming_tubing_fixture"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "water_bath_safety_kit"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "pipette_tip_organizer"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "automated_media_exchange_cassette"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "automated_seeding_coating_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "automated_ecm_coating_qc_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cell_suspension_prep_qc_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "media_sampling_analyzer_interface"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "run_record_material_scan_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "connector_topology_scan_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "incubator_cassette_shuttle_airlock"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "robotic_cassette_gripper_end_effector"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "chip_cassette_position_randomization_tray"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "environmental_mapping_cassette_surrogate"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "environmental_sensor_calibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cassette_storage_recovery_incubator_rack"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_cell_harvest_passaging_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "perfusion_bubble_management_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "automated_cell_seeding_distribution_manifold"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "inline_media_conditioning_qc_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "gas_humidity_service_panel"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "robot_tool_change_and_end_effector_rack"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sterile_fluid_path_integrity_tester"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "aseptic_tubing_weld_seal_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_reagent_thaw_equilibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_sample_fraction_archive_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_cleaning_sanitization_validation_cart"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_fluid_path_packaging_kitting_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cell_lot_release_qc_panel"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_module_cip_sip_service_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cassette_deviation_quarantine_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_environmental_excursion_response_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sterile_tote_docking_and_transfer_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "bioreactor_feed_harvest_bag_hotel"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_cell_bank_recovery_thaw_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_consumable_pre_use_inspection_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_deviation_sample_triage_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_module_residual_rinse_sampling_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "sterile_gas_changeover_regulator_qualification_panel"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_material_passthrough_debagging_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_environmental_monitoring_plate_coupon_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "inline_sensor_cartridge_hydration_calibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "robotic_spill_response_decon_kit_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_run_start_readiness_gate_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_pump_valve_manifold_calibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_calibration_standard_custody_gate"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_media_fill_run_simulation_fixture"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_incubation_slot_map_verification_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_sterile_connector_lot_inspection_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_gasket_lot_incoming_inspection_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_waste_bag_custody_weigh_seal_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sterile_tubing_weld_seal_coupon_test_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_robot_end_effector_sterility_parking_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_hepa_filter_scan_adapter_fixture"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_culture_module_power_data_quickconnect_validation_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_aseptic_tool_surface_bioburden_sampling_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_incubator_condensate_drain_validation_fixture"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_sample_label_print_apply_verify_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_cassette_evaporation_mass_loss_mapping_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_perfusion_shear_stress_surrogate_chip_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_chain_of_custody_sample_tote_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_cell_density_viability_sampling_station"));
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_microfluidic_chip_preflush_debubble_station")
        );
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_media_osmolality_conductivity_qc_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_media_buffer_hold_time_stability_station"));
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_inline_filter_preuse_integrity_test_station")
        );
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_growth_factor_cytokine_addition_qc_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_chip_inlet_outlet_dead_volume_dye_recovery_station"));
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_teer_impedance_phantom_verification_station")
        );
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_reference_particle_flow_calibration_station")
        );
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_pump_tubing_occlusion_endurance_station"));
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_sampling_carryover_flush_validation_station")
        );
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_low_volume_dose_accuracy_recovery_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_reagent_additive_trace_dispense_verification_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_optical_focus_drift_calibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_reference_viscosity_media_rheology_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_environmental_particle_counter_probe_calibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_humidity_dewpoint_reference_calibration_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_module_latch_force_alignment_verification_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_waste_line_backflow_siphon_validation_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_gas_filter_integrity_leak_test_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_cassette_gasket_compression_set_aging_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_module_port_dry_break_cycle_life_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_gas_mfc_zero_span_crosscheck_station"));
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_uv_h2o2_decon_shadow_mapping_coupon_station")
        );
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_cassette_position_environmental_equivalence_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_module_thermal_contact_resistance_mapping_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_waste_container_neutralization_contact_time_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_supply_cylinder_lot_changeover_custody_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_incubator_rack_vibration_tilt_mapping_station"
        ));
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_media_bag_load_cell_drift_reference_station")
        );
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_robot_tool_tip_runout_repeatability_station")
        );
        assert!(
            OUTPUTS
                .iter()
                .any(|output| output.generator
                    == "closed_liquid_waste_aerosol_trap_integrity_station")
        );
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_service_bulkhead_misconnection_gauge_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_incubator_door_recovery_challenge_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_incubator_internal_surface_cleanability_coupon_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_sterile_connector_actuation_force_life_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_sensor_cable_feedthrough_leak_test_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_incubator_o2_co2_stratification_mapping_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_module_thermal_runaway_fault_injection_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_cassette_transport_drop_shock_inspection_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_humidity_condensate_material_compatibility_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_culture_module_service_line_kink_occlusion_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_inline_sensor_calibration_drift_challenge_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_multichannel_pressure_decay_leak_reference_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_media_reservoir_mixing_homogeneity_validation_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_perfusion_pump_cross_channel_crosstalk_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_robot_collision_force_limit_validation_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_incubator_humidity_recovery_load_challenge_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_airlock_pressure_decay_interlock_challenge_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_cassette_barcode_rfid_mismatch_reconciliation_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_cleanroom_gowning_material_shedding_surrogate_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_media_reagent_cold_chain_excursion_unpack_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_perfusion_manifold_residual_volume_drainability_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_module_external_surface_disinfectant_contact_time_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_chip_cassette_condensate_ingress_witness_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_inline_bubble_sensor_false_positive_negative_challenge_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_pass_through_tote_pressure_equalization_flow_balance_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_reagent_bag_pressure_hold_creep_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_chip_cassette_static_charge_particle_attraction_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_perfusion_line_air_ingress_microleak_challenge_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_sampling_valve_carryover_deadleg_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_robot_gripper_pad_wear_particle_shedding_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_incubator_condensate_cross_contamination_witness_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_reservoir_cap_septum_puncture_leak_life_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_incubator_fan_flow_shadowing_mapping_station"
        ));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_media_additive_light_exposure_witness_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_cassette_shipping_vibration_logger_dock_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_media_osmolality_evaporation_correlation_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_cell_source_vial_thaw_timing_stagger_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_module_luer_lock_misassembly_prevention_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_sensor_probe_cleanability_residue_station"));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_tubing_set_barcode_length_route_verification_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_co2_supply_regulator_drift_changeover_station"
        ));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_sensor_probe_storage_hydration_drift_station"
        ));
        assert!(OUTPUTS.iter().any(|output| output.generator
            == "closed_incubator_dewpoint_condensation_boundary_map_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_perfusion_filter_bypass_relief_validation_station"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator
                == "closed_humidity_water_reservoir_biofilm_witness_station"));
        assert!(OUTPUTS.iter().any(
            |output| output.generator == "closed_airlock_vhp_residue_aeration_witness_station"
        ));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sterile_tubing_harness"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cassette_bench_nest"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sealed_culture_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sealed_module_docking_bay"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "culture_module_service_skid"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "media_conditioning_perfusion_rack"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "pressure_driven_perfusion_panel"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "flow_pressure_validation_fixture"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "workcell_calibration_drawer"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "media_reagent_quarantine_pod"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "closed_isolator_workcell"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "inline_sensor_service_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cassette_sensor_backplane"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "automated_culture_imaging_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "environmental_utility_skid"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "aseptic_transfer_hatch"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "clean_support_pod_shell"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sterility_validation_challenge_rack"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "sterile_consumable_cartridge_hotel"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "waste_decon_service_pod"));
    }
}
