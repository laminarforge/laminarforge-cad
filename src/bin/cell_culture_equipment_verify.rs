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
