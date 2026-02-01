use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HcmCore {
    pub version: String,
    pub data: Vec<String>,
    /// Volume
    pub vd: Option<u32>,
    /// Capacity
    pub c: Option<u32>,
    /// Segment density
    pub sd: Option<Vec<f32>>,
    /// Segment length
    pub seg_length: Option<Vec<f64>>,
    /// Number of lanes
    pub num_lanes: Option<Vec<u32>>,
    /// Volume-to-capacity ratio
    pub vc_ratio: Option<f64>,
    /// Freeway facility density (pc/mi/ln)
    pub fd: Option<u32>,
}

#[derive(Debug, PartialEq)]
enum FacilityType {
    Urban,
    Rural,
}


impl HcmCore {
    pub fn new(version: &str, data: Vec<String>) -> Self {
        HcmCore {
            version: version.to_string(),
            data,
            vd: None,
            c: None,
            sd: None,
            seg_length: None,
            num_lanes: None,
            vc_ratio: None,
            fd: None,
        }
    }

    pub fn display_info(&self) {
        println!("HCM Version: {}", self.version);
        println!("Data Entries: {:?}", self.data);
    }

    pub fn calculate_adj_exit_demand(&self, v_on_15: Vec<u32>, v_off_15: Vec<u32>) -> Result<u32, &'static str> {
        /// Calculate adjusted exit demand based on 15-minute volumes at period i
        /// Time interval scale factor for analysis period i
        let mut f_tis = 0.0;
        let mut adj_exit_demand = Vec::new();

        if v_on_15.len() != v_off_15.len() || v_on_15.is_empty() {
            return Err("Mismatched or empty volume vectors");
        }

        f_tis = v_on_15.iter().sum::<u32>() as f64 / v_off_15.iter().sum::<u32>() as f64;

        adj_exit_demand = v_off_15.iter()
            .map(|v_off| (v_off * f_tis as u32))
            .collect();

        Ok(adj_exit_demand)
    }

    pub fn calculate_vc_ratio(&self) -> f64 {
        if let (Some(vd), Some(c)) = (self.vd, self.c) {
            vd as f64 / c as f64
        } else {
            0.0
        }
    }

    pub fn calibrate_ffs(&self, ffs: f64, saf_cal: f64) -> Result<f64, &'static str> {
        /// Calibrate free-flow speed (ffs) using speed adjustment factor (SAF)
        if saf_cal <= 0.0 {
            return Err("Speed adjustment factor must be greater than zero");
        }
        Ok(ffs * saf_cal)
    }

    pub fn calibrate_capacity(&self, c: f64, caf_cal: f64) -> Result<f64, &'static str> {
        /// Calibrate capacity using capacity adjustment factor
        if caf_cal <= 0.0 {
            return Err("Capacity adjustment factor must be greater than zero");
        }
        Ok(c * caf_cal)
    }

    pub fn calibrate_demand(&self, v: f64, daf_cal: f64) -> Result<f64, &'static str> {
        /// Calibrate demand using demand adjustment factor
        if daf_cal <= 0.0 {
            return Err("Demand adjustment factor must be greater than zero");
        }
        Ok(v * daf_cal)
    }

    pub fn calculate_lane_closure_severity_index(&self, or: f64, n_o: u32) -> Result<f64, &'static str> {
        /// Calculate lane closure severity index (decimal)
        /// # Arguments:
        /// * OR: open ratio, the ratio of the number of open lanes during road work to the total (or normal) number of lanes (decimal)
        /// * n_o: number of open lanes in the work zone (ln)
        if or < 0.0 || n_o < 0 {
            return Err("Original and new volumes must be greater than zero");
        }

        Ok(1 / (or * n_o as f64))
    }

    pub fn calculate_queue_discharge_rate_wz(&self, lcsi: Option<f64>, f_br: f64, f_at: f64, f_lat: f64, 
        f_dn: f64, or: Option<f64>, n_o: Option<u32>) -> Result<f64, &'static str> {
        /// Calculate queue discharge rate during work zone
        /// # Arguments:
        /// * qdr_wz: average 15-minute queue discharge rate (pc/h/ln) at the work zone bottleneck
        /// * lcsi: lane closure severity index
        /// * f_br:
        /// * f_at: 
        /// * f_lat: 
        /// * f_dn: 

        if f_br <= 0.0 || f_at <= 0.0 || f_lat <= 0.0 || f_dn <= 0.0 {
            return Err("All factors must be greater than zero");
        }

        let qdr_wz = match lcsi {
            Some(lcsi) => 2093 - 154 * lcsi - 194 * f_br - 179 * f_at + 9 * f_lat - 59 * f_dn,
            None => {
                // Calculate lane closure severity index if not provided
                let or = or.ok_or("Open ratio missing")?;
                let n_o = n_o.ok_or("Number of open lanes missing")?;
                self.calculate_lane_closure_severity_index(or, n_o)?
            }
        };

        Ok(lcsi * f_br * f_at * f_lat * f_dn)
    }

    pub fn calculate_capacity_wz(&self, qdr_wz: f64, a_wz: f64) -> Result<f64, &'static str> {
        /// Calculate capacity during work zone
        /// # Arguments:
        /// * c_wz: capacity at the work zone bottleneck (pc/h/ln)
        /// * qdr_wz: average 15-minute queue discharge rate at the work zone bottleneck
        /// * a_wz: percentage drop in prebreakdown capacity at the work zone due to queuing conditions (%)

        /// # References:
        /// [1] Yeom, C., A. Hajbabaie, B. J. Schroeder, C. Vaughan, X. Xuan, and N. M. Rouphail. Innovative Work Zone Capacity Models from Nationwide Field and Archival Sources. In Transportation Research Record: Journal of the Transportation Research Board, No. 2485, Transportation Research Board, Washington, D.C., 2015, pp. 51–60.
        /// [2] Hu, J., B. J. Schroeder, and N. M. Rouphail. Rationale for Incorporating Queue Discharge Flow into Highway Capacity Manual Procedure for Analysis of Freeway Facilities. In Transportation Research Record: Journal of the Transportation Research Board, No. 2286, Transportation Research Board of the National Academies, Washington, D.C., 2012, pp. 76–83.

        if a_wz <= 0.0 {
            return Err("Adjustment factor for work zone conditions must be greater than zero");
        }

        Ok(qdr_wz / (100 - a_wz) * 100.0)
    }

    pub fn calculate_ffs_wz(&self, f_sr: f64, sl_wz: f64, lcsi: f64, f_br: f64, f_dn: f64, trd: f64) -> Result<f64, &'static str> {
        /// Calculate free-flow speed during work zone
        /// # Arguments:
        /// * ffs_wz: free-flow speed at the work zone bottleneck (mi/h)
        /// * f_sr: 
        /// * sl_wz: 
        /// * lcsi: lane closure severity index
        /// * f_br: 
        /// * f_dn: 
        /// * trd: 

        if f_sr <= 0.0 || sl_wz <= 0.0 || lcsi <= 0.0 || f_br <= 0.0 || f_dn <= 0.0 {
            return Err("All factors must be greater than zero");
        }

        /// Ensure f_sr is within the cap
        if f_sr < 1.0 {
            f_sr = 1.0;
        }
        if f_sr > 1.2 {
            f_sr = 1.2;
        }

        let mut ffs_wz = 9.95 + 33.49 * f_sr + 0.53 * sl_wz - 5.6 * lcsi - 3.84 * f_br - 1.71 * f_dn - 8.7 * trd;

        Ok(ffs_wz)
    }

    pub fn calculate_caf_wz(&self, c_wz: f64, c: u32) -> Result<f32, &'static str> {
        /// Calculate capacity adjustment factor during work zone
        /// # Arguments:
        /// * caf_wz: capacity adjustment factor at the work zone bottleneck (decimal)
        /// * c_wz: capacity at the work zone bottleneck (pc/h/ln)
        /// * c: capacity at the normal condition (pc/h/ln)

        if c == 0 {
            return Err("Capacity at normal condition cannot be zero");
        }

        Ok(c_wz as f32 / c)
    }

    pub fn calculate_saf_wz(&self, ffs_wz: f64, ffs: u32) -> Result<f32, &'static str> {
        /// Calculate speed adjustment factor during work zone
        /// # Arguments:
        /// * saf_wz: speed adjustment factor at the work zone bottleneck (decimal)
        /// * ffs_wz: free-flow speed at the work zone bottleneck (mi/h)
        /// * ffs: free-flow speed at normal condition (mi/h)

        if ffs == 0 {
            return Err("Free-flow speed at normal condition cannot be zero");
        }

        Ok(ffs_wz as f32 / ffs as f32)
    }

    pub fn calculate_facility_density(&self) -> Result<f32, &'static str> {
        // Calculate facility density as weighted average:

        if lcsi <= 0.0 || f_br <= 0.0 || f_at <= 0.0 || f_lat <= 0.0 || f_dn <= 0.0 {
            return Err("All factors must be greater than zero");
        }

        Ok(lcsi * f_br * f_at * f_lat * f_dn)
    }

    pub fn calculate_facility_density(&self) -> Result<f32, &'static str> {
        // Calculate facility density as weighted average:
        /// fd = Sum(density_i * length_i * lanes_i) / Sum(length_i * lanes_i)
        
        let sd = self.sd.as_ref().ok_or("Segment density missing")?;
        let seg_length = self.seg_length.as_ref().ok_or("Segment length missing")?;
        let num_lanes = self.num_lanes.as_ref().ok_or("Number of lanes missing")?;

        // Check that all vectors are of the same length
        if sd.len() != seg_length.len() || sd.len() != num_lanes.len() {
            return Err("Mismatched vector lengths for segment data");
        }

        if sd.is_empty() {
            return Err("No segment data available");
        }
        
        let mut numerator = 0.0f32;
        let mut denominator = 0.0f32;

        for i in 0..sd.len() {
            let weight = seg_length[i] as f32 * num_lanes[i] as f32;
            numerator += sd[i] * weight;
            denominator += weight;
        }

        if denominator == 0.0 {
            return Err("Denominator is zero, cannot calculate facility density");
        }

        Ok(numerator / denominator)
    }

    pub fn calculate_facility_los(&self, facility_type: FacilityType) -> Result<String, &'static str> {
        // LOS calculation logic
        let mut vc_ratio = self.vc_ratio.unwrap_or(self.calculate_vc_ratio());

        let mut los: char = 'F';

        let fd = match self.fd {
            Some(fd) => fd,
            None => {
                // Calculate facility density if not provided
                match self.calculate_facility_density() {
                    Ok(fd) => fd as u32,
                    Err(e) => return Err(e),
                }
            }
        };

        if vc_ratio > 1.00 {
            // If volume-to-capacity ratio is greater than 1, set to LOS F
            return Ok("F".to_string());
        }


        let los = match facility_type {
            FacilityType::Urban => {
                // Urban facility LOS calculation
                if fd <= 11 {
                    // LOS A
                    'A'
                } else if fd <= 18 {
                    // LOS B
                    'B'
                } else if fd <= 26 {
                    // LOS C
                    'C'
                } else if fd <= 35 {
                    // LOS D
                    'D'
                } else if fd <= 45 {
                    // LOS E
                    'E'
                } else {
                    // LOS F
                    'F'
                }
            }
            FacilityType::Rural => {
                // Rural facility LOS calculation
                if fd <= 6 {
                    // LOS A
                    'A'
                } else if fd <= 14 {
                    // LOS B
                    'B'
                } else if fd <= 22 {
                    // LOS C
                    'C'
                } else if fd <= 29 {
                    // LOS D
                    'D'
                } else if fd <= 39 {
                    // LOS E
                    'E'
                } else {
                    // LOS F
                    'F'
                }
            }
        };

        Ok(los.to_string());
    }
}