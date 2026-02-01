#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_hcm_core() {
        let hcm = HcmCore::new("HCM 7th Edition", vec!["segment1".to_string(), "segment2".to_string()]);
        assert_eq!(hcm.version, "HCM 7th Edition");
        assert_eq!(hcm.data.len(), 2);
        assert!(hcm.vd.is_none());
        assert!(hcm.fd.is_none());
    }

    #[test]
    fn test_calculate_vc_ratio() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        hcm.vd = Some(1500);
        hcm.c = Some(2000);
        
        let ratio = hcm.calculate_vc_ratio();
        assert_eq!(ratio, 0.75);
    }

    #[test]
    fn test_calculate_vc_ratio_missing_data() {
        let hcm = HcmCore::new("HCM 7th", vec![]);
        let ratio = hcm.calculate_vc_ratio();
        assert_eq!(ratio, 0.0);
    }

    #[test]
    fn test_calculate_facility_density() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        
        // Example: 3 segments
        // Segment 1: density=20, length=1.0, lanes=3 -> weight=3.0, contribution=60.0
        // Segment 2: density=15, length=2.0, lanes=2 -> weight=4.0, contribution=60.0
        // Segment 3: density=25, length=0.5, lanes=4 -> weight=2.0, contribution=50.0
        // Total: numerator=170.0, denominator=9.0, result=18.89
        
        hcm.sd = Some(vec![20.0, 15.0, 25.0]);
        hcm.seg_length = Some(vec![1.0, 2.0, 0.5]);
        hcm.num_lanes = Some(vec![3, 2, 4]);
        
        let density = hcm.calculate_facility_density().unwrap();
        assert!((density - 18.888889).abs() < 0.001); // floating point comparison
    }

    #[test]
    fn test_calculate_facility_density_missing_data() {
        let hcm = HcmCore::new("HCM 7th", vec![]);
        let result = hcm.calculate_facility_density();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Segment density data missing");
    }

    #[test]
    fn test_calculate_facility_density_mismatched_vectors() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        hcm.sd = Some(vec![20.0, 15.0]);
        hcm.seg_length = Some(vec![1.0, 2.0, 0.5]); // Different length
        hcm.num_lanes = Some(vec![3, 2]);
        
        let result = hcm.calculate_facility_density();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mismatched vector lengths for segment data");
    }

    #[test]
    fn test_calculate_facility_los_urban() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        hcm.fd = Some(15.0); // Should be LOS B for urban
        hcm.vc_ratio = Some(0.8);
        
        let los = hcm.calculate_facility_los(FacilityType::Urban).unwrap();
        assert_eq!(los, "B");
    }

    #[test]
    fn test_calculate_facility_los_rural() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        hcm.fd = Some(15.0); // Should be LOS B for rural
        hcm.vc_ratio = Some(0.8);
        
        let los = hcm.calculate_facility_los(FacilityType::Rural).unwrap();
        assert_eq!(los, "B");
    }

    #[test]
    fn test_calculate_facility_los_overcapacity() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        hcm.fd = Some(10.0); // Would normally be LOS A
        hcm.vc_ratio = Some(1.2); // Over capacity
        
        let los = hcm.calculate_facility_los(FacilityType::Urban).unwrap();
        assert_eq!(los, "F"); // Should be F due to overcapacity
    }

    #[test]
    fn test_calculate_facility_los_with_calculated_density() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        
        // Set up segment data that will result in density of ~18.89 (LOS B for urban)
        hcm.sd = Some(vec![20.0, 15.0, 25.0]);
        hcm.seg_length = Some(vec![1.0, 2.0, 0.5]);
        hcm.num_lanes = Some(vec![3, 2, 4]);
        hcm.vc_ratio = Some(0.8);
        
        let los = hcm.calculate_facility_los(FacilityType::Urban).unwrap();
        assert_eq!(los, "B");
    }

    #[test]
    fn test_boundary_conditions() {
        let mut hcm = HcmCore::new("HCM 7th", vec![]);
        hcm.vc_ratio = Some(0.8);
        
        // Test exact boundary values for urban LOS
        hcm.fd = Some(11.0);
        assert_eq!(hcm.calculate_facility_los(FacilityType::Urban).unwrap(), "A");
        
        hcm.fd = Some(11.1);
        assert_eq!(hcm.calculate_facility_los(FacilityType::Urban).unwrap(), "B");
        
        hcm.fd = Some(45.0);
        assert_eq!(hcm.calculate_facility_los(FacilityType::Urban).unwrap(), "E");
        
        hcm.fd = Some(45.1);
        assert_eq!(hcm.calculate_facility_los(FacilityType::Urban).unwrap(), "F");
    }
}

// Example usage function
pub fn example_usage() {
    println!("=== HCM Core Example Usage ===");
    
    let mut hcm = HcmCore::new("HCM 7th Edition", vec![
        "I-95 Segment 1".to_string(),
        "I-95 Segment 2".to_string(),
        "I-95 Segment 3".to_string(),
    ]);
    
    // Set up segment data
    hcm.sd = Some(vec![22.0, 18.0, 28.0]);          // density (pc/mi/ln)
    hcm.seg_length = Some(vec![1.2, 0.8, 1.5]);     // length (mi)
    hcm.num_lanes = Some(vec![3, 4, 2]);            // number of lanes
    hcm.vd = Some(1800);                            // volume (vph)
    hcm.c = Some(2200);                             // capacity (vph)
    
    hcm.display_info();
    
    // Calculate facility density
    match hcm.calculate_facility_density() {
        Ok(density) => {
            println!("Calculated Facility Density: {:.2} pc/mi/ln", density);
            hcm.fd = Some(density); // Store calculated value
        }
        Err(e) => println!("Error calculating density: {}", e),
    }
    
    // Calculate V/C ratio
    let vc_ratio = hcm.calculate_vc_ratio();
    hcm.vc_ratio = Some(vc_ratio);
    println!("Volume-to-Capacity Ratio: {:.3}", vc_ratio);
    
    // Calculate LOS for both facility types
    match hcm.calculate_facility_los(FacilityType::Urban) {
        Ok(los) => println!("Urban Facility LOS: {}", los),
        Err(e) => println!("Error calculating urban LOS: {}", e),
    }
    
    match hcm.calculate_facility_los(FacilityType::Rural) {
        Ok(los) => println!("Rural Facility LOS: {}", los),
        Err(e) => println!("Error calculating rural LOS: {}", e),
    }
}