//! Network Topology Types for OpenDRIVE Interoperability
//!
//! This module provides core network topology types that map to OpenDRIVE concepts,
//! enabling seamless integration with road network simulation and digital twin systems.
//!
//! # Key Types
//! - `Node`: Junction/intersection points in the network
//! - `NetworkSegment`: Road links between nodes
//! - `Intersection`: Detailed intersection properties
//!
//! # OpenDRIVE Mapping
//! - `Node` → OpenDRIVE junction element
//! - `NetworkSegment` → OpenDRIVE road element
//! - `Direction` → OpenDRIVE travel direction

use serde::{Deserialize, Serialize};
use crate::hcm::common::FacilityType;

// ═══════════════════════════════════════════════════════════════════════════════
// Node Types (OpenDRIVE junction types)
// ═══════════════════════════════════════════════════════════════════════════════

/// Node type classification for network topology
/// Maps to OpenDRIVE junction types and HCM intersection types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Standard at-grade intersection (multiple approach legs)
    Intersection,
    /// Road terminus / dead end (single connection)
    Terminus,
    /// Roadway transition point (lane count change, facility type change)
    Transition,
    /// Freeway ramp junction (on-ramp or off-ramp connection)
    RampJunction,
    /// Merge point (multiple lanes/roads combining into fewer)
    Merge,
    /// Diverge point (single road splitting into multiple)
    Diverge,
}

impl NodeType {
    /// Convert to OpenDRIVE junction type string
    pub fn to_opendrive(&self) -> &'static str {
        match self {
            NodeType::Intersection => "default",
            NodeType::Terminus => "virtual",
            NodeType::Transition => "virtual",
            NodeType::RampJunction => "direct",
            NodeType::Merge => "direct",
            NodeType::Diverge => "direct",
        }
    }

    /// Check if this node type typically has signal control
    pub fn typically_signalized(&self) -> bool {
        matches!(self, NodeType::Intersection)
    }

    /// Get minimum expected connection count
    pub fn min_connections(&self) -> u32 {
        match self {
            NodeType::Terminus => 1,
            NodeType::Merge | NodeType::Diverge | NodeType::Transition => 2,
            NodeType::RampJunction => 2,
            NodeType::Intersection => 3,
        }
    }

    /// Get maximum expected connection count
    pub fn max_connections(&self) -> u32 {
        match self {
            NodeType::Terminus => 1,
            NodeType::Merge | NodeType::Diverge => 4,
            NodeType::Transition => 2,
            NodeType::RampJunction => 3,
            NodeType::Intersection => 8,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Direction Types
// ═══════════════════════════════════════════════════════════════════════════════

/// Travel direction for network segments
/// Maps to OpenDRIVE road direction conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// One-way in positive s-direction (reference line direction)
    Forward,
    /// One-way in negative s-direction (opposite to reference line)
    Backward,
    /// Two-way traffic (bidirectional)
    Both,
}

impl Direction {
    /// Convert to OpenDRIVE direction attribute
    pub fn to_opendrive(&self) -> &'static str {
        match self {
            Direction::Forward => "same",
            Direction::Backward => "opposite",
            Direction::Both => "both",
        }
    }

    /// Check if direction allows forward travel
    pub fn allows_forward(&self) -> bool {
        matches!(self, Direction::Forward | Direction::Both)
    }

    /// Check if direction allows backward travel
    pub fn allows_backward(&self) -> bool {
        matches!(self, Direction::Backward | Direction::Both)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Intersection Control Types (MUTCD/HCM)
// ═══════════════════════════════════════════════════════════════════════════════

/// Intersection control type classification
/// Based on HCM intersection analysis methodology and MUTCD control types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlType {
    /// No traffic control device (free-flow)
    Uncontrolled,
    /// Two-way stop control (minor street stops)
    StopTwoWay,
    /// All-way stop control (all approaches stop)
    StopAllWay,
    /// Yield control (approaching traffic yields)
    Yield,
    /// Traffic signal control (fixed-time or actuated)
    Signal,
    /// Modern roundabout (yield on entry)
    Roundabout,
}

impl ControlType {
    /// Get HCM methodology chapter for this control type
    pub fn hcm_chapter(&self) -> &'static str {
        match self {
            ControlType::Uncontrolled => "Chapter 22",
            ControlType::StopTwoWay => "Chapter 20",
            ControlType::StopAllWay => "Chapter 21",
            ControlType::Yield => "Chapter 20",
            ControlType::Signal => "Chapter 19",
            ControlType::Roundabout => "Chapter 22",
        }
    }

    /// Check if this control type involves stopping
    pub fn requires_stop(&self) -> bool {
        matches!(self, ControlType::StopTwoWay | ControlType::StopAllWay)
    }

    /// Check if this control type has signal timing
    pub fn has_signal_timing(&self) -> bool {
        matches!(self, ControlType::Signal)
    }

    /// Get typical base saturation flow rate (pc/hr/ln)
    /// HCM default values for signalized and unsignalized intersections
    pub fn base_saturation_flow(&self) -> Option<f64> {
        match self {
            ControlType::Signal => Some(1900.0),
            ControlType::StopAllWay => Some(720.0), // HCM base for AWSC
            ControlType::Roundabout => Some(1200.0), // Typical roundabout entry capacity
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Node Structure
// ═══════════════════════════════════════════════════════════════════════════════

/// Network node representing a junction or connection point
/// Coordinates follow OpenDRIVE convention (meters in projected coordinate system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for the node
    pub id: String,
    /// X coordinate in meters (OpenDRIVE inertial frame)
    pub x: f64,
    /// Y coordinate in meters (OpenDRIVE inertial frame)
    pub y: f64,
    /// Elevation in meters above reference datum
    pub elevation: f64,
    /// Number of connected segments
    pub connection_count: u32,
    /// Classification of the node type
    pub node_type: NodeType,
}

impl Node {
    /// Create a new Node with validation
    pub fn new(
        id: String,
        x: f64,
        y: f64,
        elevation: f64,
        connection_count: u32,
        node_type: NodeType,
    ) -> Result<Self, String> {
        // Validate connection count is appropriate for node type
        if connection_count < node_type.min_connections() {
            return Err(format!(
                "Connection count {} is below minimum {} for {:?}",
                connection_count,
                node_type.min_connections(),
                node_type
            ));
        }
        if connection_count > node_type.max_connections() {
            return Err(format!(
                "Connection count {} exceeds maximum {} for {:?}",
                connection_count,
                node_type.max_connections(),
                node_type
            ));
        }

        Ok(Self {
            id,
            x,
            y,
            elevation,
            connection_count,
            node_type,
        })
    }

    /// Calculate 2D distance to another node (meters)
    pub fn distance_to(&self, other: &Node) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate 3D distance including elevation (meters)
    pub fn distance_3d_to(&self, other: &Node) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.elevation - other.elevation;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Network Segment Structure
// ═══════════════════════════════════════════════════════════════════════════════

/// Network segment representing a road link between nodes
/// Maps to OpenDRIVE road element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSegment {
    /// Unique identifier for the segment
    pub id: String,
    /// ID of the starting node
    pub start_node_id: String,
    /// ID of the ending node
    pub end_node_id: String,
    /// Length of the segment in meters
    pub length: f64,
    /// Travel direction(s) allowed
    pub direction: Direction,
    /// Facility type classification (from HCM)
    pub facility_type: FacilityType,
    /// Number of lanes (total, both directions if bidirectional)
    pub lane_count: u32,
}

impl NetworkSegment {
    /// Create a new NetworkSegment with validation
    pub fn new(
        id: String,
        start_node_id: String,
        end_node_id: String,
        length: f64,
        direction: Direction,
        facility_type: FacilityType,
        lane_count: u32,
    ) -> Result<Self, String> {
        // Validate length
        if length <= 0.0 {
            return Err("Segment length must be positive".to_string());
        }
        if length < 10.0 {
            // Warning: Very short segment
            eprintln!("Warning: Segment {} has length {} m (< 10 m minimum recommended)", id, length);
        }

        // Validate lane count
        if lane_count == 0 {
            return Err("Segment must have at least one lane".to_string());
        }

        // Validate node IDs are different
        if start_node_id == end_node_id {
            return Err("Start and end nodes must be different".to_string());
        }

        Ok(Self {
            id,
            start_node_id,
            end_node_id,
            length,
            direction,
            facility_type,
            lane_count,
        })
    }

    /// Check if segment is one-way
    pub fn is_one_way(&self) -> bool {
        !matches!(self.direction, Direction::Both)
    }

    /// Get lanes in forward direction
    pub fn lanes_forward(&self) -> u32 {
        match self.direction {
            Direction::Forward => self.lane_count,
            Direction::Backward => 0,
            Direction::Both => self.lane_count / 2 + self.lane_count % 2, // Majority forward
        }
    }

    /// Get lanes in backward direction
    pub fn lanes_backward(&self) -> u32 {
        match self.direction {
            Direction::Forward => 0,
            Direction::Backward => self.lane_count,
            Direction::Both => self.lane_count / 2,
        }
    }

    /// Convert length to miles (for HCM calculations)
    pub fn length_miles(&self) -> f64 {
        self.length * 0.000621371
    }

    /// Convert length to feet (for geometric calculations)
    pub fn length_feet(&self) -> f64 {
        self.length * 3.28084
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Intersection Structure
// ═══════════════════════════════════════════════════════════════════════════════

/// Intersection with detailed approach and control information
/// Extends Node with intersection-specific analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intersection {
    /// Unique identifier for the intersection
    pub id: String,
    /// Reference to the underlying node
    pub node_id: String,
    /// Number of approach legs
    pub approach_count: u32,
    /// Type of traffic control
    pub control_type: ControlType,
    /// IDs of segments forming approach legs
    pub approach_segment_ids: Vec<String>,
}

impl Intersection {
    /// Create a new Intersection with validation
    pub fn new(
        id: String,
        node_id: String,
        approach_count: u32,
        control_type: ControlType,
        approach_segment_ids: Vec<String>,
    ) -> Result<Self, String> {
        // Validate approach count
        if approach_count < 2 || approach_count > 6 {
            return Err(format!(
                "Approach count {} outside valid range (2-6)",
                approach_count
            ));
        }

        // Validate approach segment count matches approach count
        if approach_segment_ids.len() != approach_count as usize {
            return Err(format!(
                "Approach segment count {} doesn't match approach count {}",
                approach_segment_ids.len(),
                approach_count
            ));
        }

        Ok(Self {
            id,
            node_id,
            approach_count,
            control_type,
            approach_segment_ids,
        })
    }

    /// Check if intersection is signalized
    pub fn is_signalized(&self) -> bool {
        matches!(self.control_type, ControlType::Signal)
    }

    /// Get the appropriate HCM analysis methodology
    pub fn hcm_methodology(&self) -> &'static str {
        self.control_type.hcm_chapter()
    }

    /// Check if this is a standard 4-leg intersection
    pub fn is_four_leg(&self) -> bool {
        self.approach_count == 4
    }

    /// Check if this is a T-intersection (3 legs)
    pub fn is_t_intersection(&self) -> bool {
        self.approach_count == 3
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Network Graph Structure (Optional utility for connectivity analysis)
// ═══════════════════════════════════════════════════════════════════════════════

/// Simple network connectivity for topology analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    /// All nodes in the network
    pub nodes: Vec<Node>,
    /// All segments in the network
    pub segments: Vec<NetworkSegment>,
    /// All intersections (subset of nodes with detailed info)
    pub intersections: Vec<Intersection>,
}

impl NetworkTopology {
    /// Create an empty network topology
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            segments: Vec::new(),
            intersections: Vec::new(),
        }
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Add a segment to the network
    pub fn add_segment(&mut self, segment: NetworkSegment) {
        self.segments.push(segment);
    }

    /// Add an intersection to the network
    pub fn add_intersection(&mut self, intersection: Intersection) {
        self.intersections.push(intersection);
    }

    /// Get node by ID
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Get segment by ID
    pub fn get_segment(&self, id: &str) -> Option<&NetworkSegment> {
        self.segments.iter().find(|s| s.id == id)
    }

    /// Get all segments connected to a node
    pub fn get_connected_segments(&self, node_id: &str) -> Vec<&NetworkSegment> {
        self.segments
            .iter()
            .filter(|s| s.start_node_id == node_id || s.end_node_id == node_id)
            .collect()
    }

    /// Calculate total network length in meters
    pub fn total_length(&self) -> f64 {
        self.segments.iter().map(|s| s.length).sum()
    }

    /// Calculate total network length in miles
    pub fn total_length_miles(&self) -> f64 {
        self.total_length() * 0.000621371
    }

    /// Get count of nodes by type
    pub fn count_by_node_type(&self, node_type: NodeType) -> usize {
        self.nodes.iter().filter(|n| n.node_type == node_type).count()
    }

    /// Get count of intersections by control type
    pub fn count_by_control_type(&self, control_type: ControlType) -> usize {
        self.intersections.iter().filter(|i| i.control_type == control_type).count()
    }
}

impl Default for NetworkTopology {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(
            "N001".to_string(),
            100.0,
            200.0,
            0.0,
            4,
            NodeType::Intersection,
        );
        assert!(node.is_ok());
        let node = node.unwrap();
        assert_eq!(node.id, "N001");
        assert_eq!(node.connection_count, 4);
    }

    #[test]
    fn test_node_invalid_connection_count() {
        // Intersection requires at least 3 connections
        let node = Node::new(
            "N001".to_string(),
            100.0,
            200.0,
            0.0,
            2, // Invalid for intersection
            NodeType::Intersection,
        );
        assert!(node.is_err());
    }

    #[test]
    fn test_terminus_node() {
        let node = Node::new(
            "T001".to_string(),
            0.0,
            0.0,
            0.0,
            1,
            NodeType::Terminus,
        );
        assert!(node.is_ok());
    }

    #[test]
    fn test_node_distance() {
        let n1 = Node::new("N1".to_string(), 0.0, 0.0, 0.0, 1, NodeType::Terminus).unwrap();
        let n2 = Node::new("N2".to_string(), 3.0, 4.0, 0.0, 1, NodeType::Terminus).unwrap();
        assert!((n1.distance_to(&n2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_segment_creation() {
        let segment = NetworkSegment::new(
            "S001".to_string(),
            "N001".to_string(),
            "N002".to_string(),
            500.0,
            Direction::Both,
            FacilityType::TwoLaneHighway,
            2,
        );
        assert!(segment.is_ok());
        let segment = segment.unwrap();
        assert!((segment.length_miles() - 0.310686).abs() < 0.001);
    }

    #[test]
    fn test_segment_invalid_length() {
        let segment = NetworkSegment::new(
            "S001".to_string(),
            "N001".to_string(),
            "N002".to_string(),
            -100.0, // Invalid
            Direction::Forward,
            FacilityType::BasicFreeway,
            4,
        );
        assert!(segment.is_err());
    }

    #[test]
    fn test_segment_lane_distribution() {
        let segment = NetworkSegment::new(
            "S001".to_string(),
            "N001".to_string(),
            "N002".to_string(),
            1000.0,
            Direction::Both,
            FacilityType::BasicFreeway,
            4,
        ).unwrap();
        assert_eq!(segment.lanes_forward(), 2);
        assert_eq!(segment.lanes_backward(), 2);
    }

    #[test]
    fn test_intersection_creation() {
        let intersection = Intersection::new(
            "I001".to_string(),
            "N001".to_string(),
            4,
            ControlType::Signal,
            vec!["S1".to_string(), "S2".to_string(), "S3".to_string(), "S4".to_string()],
        );
        assert!(intersection.is_ok());
        let intersection = intersection.unwrap();
        assert!(intersection.is_signalized());
        assert!(intersection.is_four_leg());
    }

    #[test]
    fn test_intersection_invalid_approach_count() {
        let intersection = Intersection::new(
            "I001".to_string(),
            "N001".to_string(),
            7, // Invalid - max is 6
            ControlType::Signal,
            vec!["S1".to_string(); 7],
        );
        assert!(intersection.is_err());
    }

    #[test]
    fn test_control_type_saturation_flow() {
        assert_eq!(ControlType::Signal.base_saturation_flow(), Some(1900.0));
        assert_eq!(ControlType::StopAllWay.base_saturation_flow(), Some(720.0));
        assert_eq!(ControlType::Uncontrolled.base_saturation_flow(), None);
    }

    #[test]
    fn test_direction_opendrive() {
        assert_eq!(Direction::Forward.to_opendrive(), "same");
        assert_eq!(Direction::Backward.to_opendrive(), "opposite");
        assert_eq!(Direction::Both.to_opendrive(), "both");
    }

    #[test]
    fn test_network_topology() {
        let mut network = NetworkTopology::new();

        network.add_node(Node::new("N1".to_string(), 0.0, 0.0, 0.0, 1, NodeType::Terminus).unwrap());
        network.add_node(Node::new("N2".to_string(), 1000.0, 0.0, 0.0, 3, NodeType::Intersection).unwrap());

        network.add_segment(NetworkSegment::new(
            "S1".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            1000.0,
            Direction::Both,
            FacilityType::TwoLaneHighway,
            2,
        ).unwrap());

        assert_eq!(network.nodes.len(), 2);
        assert_eq!(network.segments.len(), 1);
        assert!((network.total_length() - 1000.0).abs() < 0.001);
    }
}
