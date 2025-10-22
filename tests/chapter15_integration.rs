#[allow(unused_imports)]
use transportations_library::*;

mod common;

/// Test the complete HCM Chapter 15 analysis workflow
#[test]
fn test_complete_hcm_workflow() {
    let mut highway = common::create_sample_highway();
    
    // Verify we have segments to test
    assert!(!highway.get_segments().is_empty(), "Highway should have segments for testing");
    
    for segment_index in 0..highway.get_segments().len() {
        println!("Testing segment {} of {}", segment_index + 1, highway.get_segments().len());
        
        // Step 1: Identify vertical class (HCM Step 1)
        let (min_val, max_val) = highway.identify_vertical_class(segment_index);
        assert!(min_val >= 0.0, "Minimum vertical class value should be non-negative");
        assert!(max_val >= min_val, "Maximum should be >= minimum");
        assert!(max_val <= 3.0, "Maximum vertical class value should not exceed 3.0");
        
        // Step 2: Determine demand flow rates and capacity (HCM Step 2)
        let (flow_i, flow_o, capacity) = highway.determine_demand_flow(segment_index);
        assert!(flow_i > 0.0, "Demand flow rate should be positive");
        assert!(flow_o >= 0.0, "Opposing flow should be non-negative");
        assert!(capacity > 0, "Capacity should be positive");
        assert!(capacity >= 1100 && capacity <= 1700, "Capacity should be within HCM range");
        
        // Verify flow rates don't exceed capacity
        assert!(flow_i <= capacity as f64 * 1.2, "Flow shouldn't greatly exceed capacity");
        
        // Step 3: Determine vertical alignment classification (HCM Step 3)
        let alignment = highway.determine_vertical_alignment(segment_index);
        assert!(alignment >= 1 && alignment <= 5, "Vertical alignment class should be 1-5");
        
        // Step 4: Determine free-flow speed (HCM Step 4)
        let ffs = highway.determine_free_flow_speed(segment_index);
        let speed_limit = highway.get_segments()[segment_index].get_spl();
        assert!(ffs > 0.0, "Free flow speed should be positive");
        assert!(ffs >= 20.0 && ffs <= 80.0, "FFS should be reasonable: {} mph", ffs);
        assert!(ffs <= speed_limit * 1.3, "FFS shouldn't be much higher than speed limit");
        
        // Step 5: Estimate average speed (HCM Step 5)
        let (avg_speed, hor_class) = highway.estimate_average_speed(segment_index);
        assert!(avg_speed > 0.0, "Average speed should be positive");
        assert!(avg_speed <= ffs, "Average speed should not exceed free flow speed");
        assert!(avg_speed >= ffs * 0.5, "Average speed shouldn't be unreasonably low");
        assert!(hor_class >= 0 && hor_class <= 5, "Horizontal class should be 0-5");
        
        // Step 6: Estimate percent followers (HCM Step 6)
        let pf = highway.estimate_percent_followers(segment_index);
        assert!(pf >= 0.0 && pf <= 100.0, "Percent followers should be 0-100%");
        
        // Higher flow rates should generally result in higher percent followers
        if flow_i > 1000.0 {
            assert!(pf > 50.0, "High flow rates should result in high percent followers");
        }
        
        // Step 7: Determine follower density (HCM Step 7)
        let passing_type = highway.get_segments()[segment_index].get_passing_type();
        let fd = match passing_type {
            2 => { // Passing Lane
                let (fd, fd_mid) = highway.determine_follower_density_pl(segment_index);
                assert!(fd >= 0.0, "Follower density should be non-negative");
                assert!(fd_mid >= 0.0, "Mid-segment follower density should be non-negative");
                assert!(fd_mid <= 50.0, "Mid-segment follower density seems too high: {}", fd_mid);
                fd_mid // Use mid-point for LOS calculation
            }
            _ => { // Passing Constrained or Passing Zone
                let fd = highway.determine_follower_density_pc_pz(segment_index);
                assert!(fd >= 0.0, "Follower density should be non-negative");
                assert!(fd <= 50.0, "Follower density seems too high: {}", fd);
                fd
            }
        };
        
        // Step 8: Determine Level of Service (HCM Step 8)
        let los = highway.determine_segment_los(segment_index, avg_speed, capacity);
        assert!("ABCDEF".contains(los), "LOS should be A, B, C, D, E, or F");
        
        // Verify LOS makes sense with the metrics
        if avg_speed >= 50.0 && fd <= 2.0 {
            assert!(los == 'A' || los == 'B', "High speed + low density should be LOS A or B");
        }
        if flow_i > capacity as f64 {
            assert!(los == 'F', "Over-capacity flow should result in LOS F");
        }
        
        println!("Segment {} complete: LOS {}, Speed {:.1} mph, FD {:.1}", 
                segment_index + 1, los, avg_speed, fd);
    }
}

#[test]
fn test_different_passing_types() {
    let mut highway = common::create_complex_highway();

    let mut found_pc = false;
    let mut found_pz = false;
    let mut found_pl = false;

    let segment_count = highway.get_segments().len();

    for index in 0..segment_count {
        // First, get immutable reference to segment
        let passing_type;
        let volume;
        {
            let segment = &highway.get_segments()[index];
            passing_type = segment.get_passing_type();
            volume = segment.get_volume(); // For PC segments
        }

        match passing_type {
            0 => found_pc = true,
            1 => found_pz = true,
            2 => found_pl = true,
            _ => panic!("Invalid passing type: {}", passing_type),
        }

        // Run complete analysis for each segment type
        highway.determine_demand_flow(index);
        highway.determine_free_flow_speed(index);
        let (_speed, _) = highway.estimate_average_speed(index);
        highway.estimate_percent_followers(index);


        match passing_type {
            0 => {
                let fd = highway.determine_follower_density_pc_pz(index);
                println!("Follower density at segment {}: {}", index, fd);
                assert!(
                    fd >= 0.0,
                    "PC segment follower density should be non-negative"
                );

                if volume > 800.0 {
                    assert!(
                        fd > 5.0,
                        "High volume PC should have significant follower density"
                    );
                }
            }
            1 => {
                let fd = highway.determine_follower_density_pc_pz(index);
                assert!(
                    fd >= 0.0,
                    "PZ segment follower density should be non-negative"
                );
            }
            2 => {
                let (fd, fd_mid) = highway.determine_follower_density_pl(index);
                assert!(
                    fd >= 0.0 && fd_mid >= 0.0,
                    "PL segment densities should be non-negative"
                );

                assert!(
                    fd_mid <= fd * 1.5,
                    "Mid-point density shouldn't be much higher than entrance"
                );

                if index < segment_count - 1 {
                    let fd_adj = highway.determine_adjustment_to_follower_density(index + 1);
                    assert!(
                        fd_adj >= 0.0,
                        "Follower density adjustment should be non-negative"
                    );
                }
            }
            _ => panic!("Unexpected passing type: {}", passing_type),
        }
    }

    assert!(
        found_pc || found_pz || found_pl,
        "Should test at least one segment type"
    );
}


#[test]
fn test_facility_level_analysis() {
    let mut highway = common::create_complex_highway();
    let segment_count = highway.get_segments().len();
    
    assert!(segment_count > 0, "Highway should have segments for facility analysis");
    
    let mut total_length = 0.0;
    let mut weighted_fd = 0.0;
    let mut weighted_speed = 0.0;
    let mut segment_results = Vec::new();
    
    // Analyze each segment and collect results
    for segment_index in 0..segment_count {
        highway.determine_demand_flow(segment_index);
        highway.determine_free_flow_speed(segment_index);
        let (speed, _) = highway.estimate_average_speed(segment_index);
        highway.estimate_percent_followers(segment_index);
        
        let segment_length = highway.get_segments()[segment_index].get_length();
        let passing_type = highway.get_segments()[segment_index].get_passing_type();
        
        assert!(segment_length > 0.0, "Segment length should be positive");
        
        let fd = if passing_type == 2 {
            let (_, fd_mid) = highway.determine_follower_density_pl(segment_index);
            fd_mid
        } else {
            highway.determine_follower_density_pc_pz(segment_index)
        };
        
        total_length += segment_length;
        weighted_fd += fd * segment_length;
        weighted_speed += speed * segment_length;
        
        segment_results.push((segment_index, speed, fd, segment_length));
    }
    
    // Calculate facility-wide metrics
    assert!(total_length > 0.0, "Total facility length should be positive");
    
    let facility_fd = weighted_fd / total_length;
    let facility_speed = weighted_speed / total_length;
    let facility_los = highway.determine_facility_los(facility_fd, facility_speed);
    
    // Validate facility metrics
    assert!(facility_fd >= 0.0, "Facility follower density should be non-negative");
    assert!(facility_speed > 0.0, "Facility average speed should be positive");
    assert!("ABCDEF".contains(facility_los), "Facility LOS should be A-F");
    
    // Facility metrics should be reasonable averages
    let min_speed = segment_results.iter().map(|(_, s, _, _)| *s).fold(f64::INFINITY, f64::min);
    let max_speed = segment_results.iter().map(|(_, s, _, _)| *s).fold(f64::NEG_INFINITY, f64::max);
    
    assert!(facility_speed >= min_speed && facility_speed <= max_speed,
            "Facility speed should be between segment extremes");
    
    println!("Facility Analysis Complete:");
    println!("  Total Length: {:.1} mi", total_length);
    println!("  Average Speed: {:.1} mph", facility_speed);
    println!("  Follower Density: {:.1}", facility_fd);
    println!("  Level of Service: {}", facility_los);
    
    // Print segment breakdown
    for (idx, speed, fd, length) in segment_results {
        println!("  Segment {}: {:.1} mph, FD {:.1}, Length {:.1} mi", 
                idx + 1, speed, fd, length);
    }
}

#[test]
fn test_edge_cases_and_robustness() {
    // Test with minimal traffic
    let mut low_volume_highway = common::create_sample_highway();
    // Modify to have very low volume
    if let Some(segment) = low_volume_highway.segments.get_mut(0) {
        segment.volume = Some(50.0); // Very low volume
    }
    
    for i in 0..low_volume_highway.get_segments().len() {
        low_volume_highway.determine_demand_flow(i);
        low_volume_highway.determine_free_flow_speed(i);
        let (speed, _) = low_volume_highway.estimate_average_speed(i);
        let pf = low_volume_highway.estimate_percent_followers(i);
        
        // Low volume should result in high speeds, low percent followers
        assert!(speed > 45.0, "Low volume should result in high speeds");
        assert!(pf < 50.0, "Low volume should result in low percent followers");
    }
    
    // Test with high traffic
    let mut high_volume_highway = common::create_sample_highway();
    if let Some(segment) = high_volume_highway.segments.get_mut(0) {
        segment.volume = Some(1400.0); // High volume
    }
    
    for i in 0..high_volume_highway.get_segments().len() {
        high_volume_highway.determine_demand_flow(i);
        high_volume_highway.determine_free_flow_speed(i);
        let (_speed, _) = high_volume_highway.estimate_average_speed(i);
        let pf = high_volume_highway.estimate_percent_followers(i);
        
        // High volume should result in lower speeds, high percent followers
        assert!(pf > 60.0, "High volume should result in high percent followers");
    }
}

#[test]
fn test_parameter_sensitivity() {
    let base_highway = common::create_sample_highway();
    
    // Test sensitivity to lane width
    let mut narrow_highway = base_highway.clone();
    narrow_highway.lane_width = Some(10.0); // Narrow lanes
    
    let mut wide_highway = base_highway.clone();
    wide_highway.lane_width = Some(14.0); // Wide lanes
    
    // Wide lanes should generally result in higher free flow speeds
    for i in 0..base_highway.get_segments().len() {
        narrow_highway.determine_demand_flow(i);
        wide_highway.determine_demand_flow(i);
        
        let narrow_ffs = narrow_highway.determine_free_flow_speed(i);
        let wide_ffs = wide_highway.determine_free_flow_speed(i);
        
        assert!(wide_ffs >= narrow_ffs, 
                "Wider lanes should result in equal or higher FFS: {} vs {}", 
                wide_ffs, narrow_ffs);
    }
}

#[test]
fn test_data_consistency() {
    let mut highway = common::create_complex_highway();
    
    for i in 0..highway.get_segments().len() {
        // Run analysis
        let (flow_i, _, capacity) = highway.determine_demand_flow(i);
        let ffs = highway.determine_free_flow_speed(i);
        let (avg_speed, _) = highway.estimate_average_speed(i);
        let pf = highway.estimate_percent_followers(i);
        
        // Test relationships between variables
        // Higher flow should generally mean lower speed (within reason)
        // Higher flow should generally mean higher percent followers
        
        // Verify calculated values are stored properly
        assert_eq!(highway.get_segments()[i].get_flow_rate(), flow_i);
        assert_eq!(highway.get_segments()[i].get_capacity(), capacity);
        assert_eq!(highway.get_segments()[i].get_ffs(), ffs);
        assert_eq!(highway.get_segments()[i].get_avg_speed(), avg_speed);
        assert_eq!(highway.get_segments()[i].get_percent_followers(), pf);
    }
}