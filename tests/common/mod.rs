#![allow(dead_code)]
use transportations_library::*;
use transportations_library::hcm::twolanehighways::{TwoLaneHighways, Segment, SubSegment};
use std::fs::{self, File};
use std::io::BufReader;

/// Create a simple highway for basic testing
pub fn create_sample_highway() -> TwoLaneHighways {
    let segments = vec![
        Segment::new(
            0, // Passing Constrained
            1.0, // 1 mile length
            2.0, // 2% grade
            55.0, // 55 mph speed limit
            Some(false), // No horizontal curves
            Some(800.0), // 800 veh/hr volume
            Some(1500.0), // 1500 veh/hr opposing volume
            None, // Flow rate (calculated)
            None, // Opposing flow rate (calculated)
            None, // Capacity (calculated)
            None, // Free flow speed (calculated)
            None, // Average speed (calculated)
            Some(1), // Vertical class I
            Some(vec![]), // No subsegments
            Some(0.95), // 95% peak hour factor
            Some(5.0), // 5% heavy vehicles
            None, // Percent followers (calculated)
            None, // Follower density (calculated)
            None, // Mid follower density (calculated)
            Some(0), // Horizontal class 0
        )
    ];

    TwoLaneHighways::new(
        segments,
        Some(12.0), // 12 ft lane width
        Some(6.0),  // 6 ft shoulder width
        Some(5.0),  // 5 access points per mile
        Some(1.0),  // Heavy vehicle multiplier
        None,       // Effective distance (calculated)
    )
}

/// Create a complex highway with multiple segment types for comprehensive testing
pub fn create_complex_highway() -> TwoLaneHighways {
    // Create subsegments for the first segment (with horizontal curves)
    let subsegments = vec![
        SubSegment::new(
            Some(2640.0), // 0.5 mile in feet
            None,    // Average speed (calculated)
            None,    // Horizontal class (calculated)
            Some(800.0), // 800 ft radius curve
            Some(30.0),  // 30 degree central angle
            Some(4.0),   // 4% superelevation
        ),
        SubSegment::new(
            Some(2640.0), // 0.5 mile tangent section
            None,    // Average speed (calculated)
            None,    // Horizontal class (calculated)
            None,    // No curve (tangent)
            None,    // No central angle
            None,    // No superelevation
        ),
    ];

    let segments = vec![
        // Segment 1: Passing Constrained with horizontal curves
        Segment::new(
            0, // Passing Constrained
            1.0, // 1 mile
            3.0, // 3% grade (steeper)
            55.0, // 55 mph
            Some(true), // Has horizontal curves
            Some(900.0), // Higher volume
            Some(1500.0), // Standard opposing volume
            None, None, None, None, None,
            Some(2), // Vertical class II
            Some(subsegments),
            Some(0.92), // Lower PHF (more peaking)
            Some(8.0),  // Higher heavy vehicle %
            None, None, None, Some(0)
        ),
        
        // Segment 2: Passing Zone
        Segment::new(
            1, // Passing Zone
            0.8, // Shorter segment
            1.0, // Gentle grade
            55.0, // Same speed limit
            Some(false), // No horizontal curves
            Some(750.0), // Moderate volume
            Some(600.0), // Lower opposing volume (allows passing)
            None, None, None, None, None,
            Some(1), // Vertical class I
            Some(vec![]), // No subsegments
            Some(0.95), // Good PHF
            Some(4.0),  // Low heavy vehicle %
            None, None, None, Some(0)
        ),
        
        // Segment 3: Passing Lane
        Segment::new(
            2, // Passing Lane
            1.2, // Longer segment
            2.5, // Moderate grade
            55.0, // Same speed limit
            Some(false), // No horizontal curves
            Some(1100.0), // High volume (benefits from passing lane)
            Some(0.0), // No opposing traffic in passing lane
            None, None, None, None, None,
            Some(3), // Vertical class III
            Some(vec![]), // No subsegments
            Some(0.88), // Lower PHF (peak conditions)
            Some(12.0), // High heavy vehicle %
            None, None, None, Some(0)
        ),
    ];

    TwoLaneHighways::new(
        segments,
        Some(11.0), // Narrower lanes
        Some(4.0),  // Narrower shoulders
        Some(8.0),  // Higher access point density
        Some(0.85), // Heavy vehicle multiplier for passing lane
        None,       // Effective distance (calculated)
    )
}

/// Create a highway with realistic rural characteristics
pub fn create_rural_highway() -> TwoLaneHighways {
    let segments = vec![
        Segment::new(
            0, // Passing Constrained (typical rural)
            2.5, // Longer segment
            1.5, // Gentle rolling terrain
            55.0, // Rural speed limit
            Some(false), // No significant curves
            Some(650.0), // Moderate rural volume
            Some(800.0), // Moderate opposing volume
            None, None, None, None, None,
            Some(1), // Class I terrain
            Some(vec![]),
            Some(0.94), // Rural PHF
            Some(6.0),  // Rural heavy vehicle %
            None, None, None, Some(0)
        )
    ];

    TwoLaneHighways::new(
        segments,
        Some(12.0), // Standard lane width
        Some(8.0),  // Wide rural shoulders
        Some(3.0),  // Low access density
        Some(1.0),  // Standard multiplier
        None,
    )
}

/// Create a highway with urban/suburban characteristics
pub fn create_suburban_highway() -> TwoLaneHighways {
    let segments = vec![
        Segment::new(
            0, // Passing Constrained (no passing in suburban)
            0.5, // Shorter urban segment
            0.5, // Flat terrain
            45.0, // Lower urban speed limit
            Some(false), // Well-designed geometry
            Some(1200.0), // High urban volume
            Some(1100.0), // High opposing volume
            None, None, None, None, None,
            Some(1), // Class I (flat)
            Some(vec![]),
            Some(0.85), // Urban peaking
            Some(3.0),  // Low heavy vehicle % (urban)
            None, None, None, Some(0)
        )
    ];

    TwoLaneHighways::new(
        segments,
        Some(11.0), // Narrower urban lanes
        Some(2.0),  // Narrow urban shoulders
        Some(15.0), // High access density
        Some(1.0),
        None,
    )
}

/// Load test data files if they exist
pub fn load_test_data_files() -> Vec<String> {
    let test_dir = "src/ExampleCases/hcm/TwoLaneHighways/";
    if std::path::Path::new(test_dir).exists() {
        let paths = fs::read_dir(test_dir).expect("Unable to read test directory");
        let mut files: Vec<String> = paths
            .map(|path| path.unwrap().path().display().to_string())
            .collect();
        files.sort();
        files
    } else {
        println!("Warning: Test data directory not found at {}", test_dir);
        vec![]
    }
}

/// Load a specific test case from JSON file
pub fn load_test_case(file_path: &str) -> Result<TwoLaneHighways> {
    let file = File::open(file_path).map_err(TransportationError::Io)?;
    let reader = BufReader::new(file);
    let highway: TwoLaneHighways = serde_json::from_reader(reader).map_err(TransportationError::Json)?;
    Ok(highway)
}
/// Run complete analysis on a highway (utility for tests)
pub fn run_complete_analysis(highway: &mut TwoLaneHighways, segment_index: usize) {
    highway.identify_vertical_class(segment_index);
    highway.determine_demand_flow(segment_index);
    highway.determine_vertical_alignment(segment_index);
    highway.determine_free_flow_speed(segment_index);
    highway.estimate_average_speed(segment_index);
    highway.estimate_percent_followers(segment_index);
    
    let passing_type = highway.get_segments()[segment_index].get_passing_type();
    if passing_type == 2 {
        highway.determine_follower_density_pl(segment_index);
    } else {
        highway.determine_follower_density_pc_pz(segment_index);
    }
    highway.determine_adjustment_to_follower_density(segment_index);
}

/// Analyze complete facility and return summary
pub fn analyze_facility(highway: &mut TwoLaneHighways) -> FacilitySummary {
    let segment_count = highway.get_segments().len();
    let mut total_length = 0.0;
    let mut weighted_fd = 0.0;
    let mut weighted_speed = 0.0;
    let mut segment_summaries = Vec::new();

    for i in 0..segment_count {
        run_complete_analysis(highway, i);
        
        let segment = &highway.get_segments()[i];
        let length = segment.get_length();
        let speed = segment.get_avg_speed();
        let passing_type = segment.get_passing_type();
        
        let fd = if passing_type == 2 {
            segment.get_followers_density_mid()
        } else {
            segment.get_followers_density()
        };
        
        total_length += length;
        weighted_fd += fd * length;
        weighted_speed += speed * length;
        
        segment_summaries.push(SegmentSummary {
            index: i,
            passing_type,
            length,
            speed,
            follower_density: fd,
            level_of_service: highway.determine_segment_los(i, speed, segment.get_capacity()),
        });
    }

    let facility_fd = weighted_fd / total_length;
    let facility_speed = weighted_speed / total_length;
    let facility_los = highway.determine_facility_los(facility_fd, facility_speed);

    FacilitySummary {
        total_length,
        average_speed: facility_speed,
        follower_density: facility_fd,
        level_of_service: facility_los,
        segments: segment_summaries,
    }
}

#[derive(Debug)]
pub struct SegmentSummary {
    pub index: usize,
    pub passing_type: usize,
    pub length: f64,
    pub speed: f64,
    pub follower_density: f64,
    pub level_of_service: char,
}

#[derive(Debug)]
pub struct FacilitySummary {
    pub total_length: f64,
    pub average_speed: f64,
    pub follower_density: f64,
    pub level_of_service: char,
    pub segments: Vec<SegmentSummary>,
}

impl std::fmt::Display for FacilitySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Facility Analysis Summary:")?;
        writeln!(f, "  Total Length: {:.1} miles", self.total_length)?;
        writeln!(f, "  Average Speed: {:.1} mph", self.average_speed)?;
        writeln!(f, "  Follower Density: {:.1}", self.follower_density)?;
        writeln!(f, "  Level of Service: {}", self.level_of_service)?;
        writeln!(f, "  Segments:")?;
        
        for segment in &self.segments {
            writeln!(f, "    {}: LOS {}, {:.1} mph, FD {:.1}, {:.1} mi", 
                    segment.index + 1, 
                    segment.level_of_service,
                    segment.speed,
                    segment.follower_density,
                    segment.length)?;
        }
        
        Ok(())
    }
}

/// Utility to create test scenarios for specific conditions
pub fn create_test_scenario(scenario: TestScenario) -> TwoLaneHighways {
    match scenario {
        TestScenario::LowVolume => {
            let mut highway = create_sample_highway();
            highway.segments[0].volume = Some(200.0);
            highway
        }
        TestScenario::HighVolume => {
            let mut highway = create_sample_highway();
            highway.segments[0].volume = Some(1400.0);
            highway
        }
        TestScenario::SteepGrade => {
            let mut highway = create_sample_highway();
            highway.segments[0].grade = 6.0;
            highway.segments[0].vertical_class = Some(4);
            highway
        }
        TestScenario::HighHeavyVehicles => {
            let mut highway = create_sample_highway();
            highway.segments[0].phv = Some(20.0);
            highway
        }
        TestScenario::NarrowLanes => {
            let mut highway = create_sample_highway();
            highway.lane_width = Some(10.0);
            highway.shoulder_width = Some(2.0);
            highway
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TestScenario {
    LowVolume,
    HighVolume,
    SteepGrade,
    HighHeavyVehicles,
    NarrowLanes,
}