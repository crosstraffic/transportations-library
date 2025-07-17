#!/usr/bin/env python3
"""
Example usage of the transportation library Python wrapper.
This demonstrates how to create segments and analyze a two-lane highway facility.
"""

import json
from transportations_library import (
    Segment, 
    TwoLaneHighways, 
    SubSegment
)

def create_highway_from_json(data):
    """
    Create a TwoLaneHighways object from JSON-like data structure.
    
    Args:
        data (dict): Highway configuration data
        
    Returns:
        TwoLaneHighways: Configured highway analysis object
    """
    
    # Create segments from the data
    segments = []
    
    for seg_data in data["segments"]:
        # Create subsegments if any
        subsegments = []
        for subseg_data in seg_data.get("subsegments", []):
            subseg = SubSegment(
                length=subseg_data.get("length"),
                avg_speed=subseg_data.get("avg_speed"),
                hor_class=subseg_data.get("hor_class"),
                design_rad=subseg_data.get("design_rad"),
                central_angle=subseg_data.get("central_angle"),
                sup_ele=subseg_data.get("sup_ele")
            )
            subsegments.append(subseg)
        
        # Create the segment
        segment = Segment(
            passing_type=seg_data["passing_type"],
            length=seg_data["length"],
            grade=seg_data["grade"],
            spl=seg_data["spl"],
            is_hc=seg_data.get("is_hc"),
            volume=seg_data.get("volume"),
            volume_op=seg_data.get("volume_op"),
            flow_rate=seg_data.get("flow_rate"),
            flow_rate_o=seg_data.get("flow_rate_o"),
            capacity=seg_data.get("capacity"),
            ffs=seg_data.get("ffs"),
            avg_speed=seg_data.get("avg_speed"),
            vertical_class=seg_data.get("vertical_class"),
            subsegments=subsegments if subsegments else None,
            phf=seg_data.get("phf"),
            phv=seg_data.get("phv"),
            pf=seg_data.get("pf"),
            fd=seg_data.get("fd"),
            fd_mid=seg_data.get("fd_mid"),
            hor_class=seg_data.get("hor_class")
        )
        segments.append(segment)
    
    # Create the highway facility
    highway = TwoLaneHighways(
        segments=segments,
        lane_width=data.get("lane_width"),
        shoulder_width=data.get("shoulder_width"),
        apd=data.get("apd"),
        pmhvfl=data.get("pmhvfl"),
        l_de=data.get("l_de")
    )
    
    return highway


def validate_input_data(data):
    """
    Validate input data and provide warnings/suggestions.
    
    Args:
        data (dict): Input data to validate
        
    Returns:
        list: List of validation warnings
    """
    
    warnings = []
    
    # Check for required fields
    if "segments" not in data or not data["segments"]:
        warnings.append("No segments provided")
        return warnings
    
    for i, seg_data in enumerate(data["segments"]):
        # Check required segment fields
        required_fields = ["passing_type", "length", "grade", "spl"]
        for field in required_fields:
            if field not in seg_data:
                warnings.append(f"Segment {i}: Missing required field '{field}'")
        
        # Validate ranges
        if seg_data.get("length", 0) <= 0:
            warnings.append(f"Segment {i}: Length must be positive")
        
        if seg_data.get("spl", 0) <= 0:
            warnings.append(f"Segment {i}: Speed limit must be positive")
        
        if seg_data.get("passing_type", 0) not in [0, 1]:
            warnings.append(f"Segment {i}: Passing type must be 0 (No Passing) or 1 (Passing)")
        
        if abs(seg_data.get("grade", 0)) > 10:
            warnings.append(f"Segment {i}: Grade {seg_data.get('grade')}% seems unusually steep")
        
        if seg_data.get("volume", 0) > 2000:
            warnings.append(f"Segment {i}: Volume {seg_data.get('volume')} seems high for two-lane highway")
    
    return warnings

def main():
    """Main execution function demonstrating the library usage."""
    
    # Your input data
    input_data = {
        "segments": [
            {
                "passing_type": 0,
                "length": 0.75,
                "grade": 0.0,
                "spl": 50.0,
                "is_hc": False,
                "volume": 752.0,
                "volume_op": 0.0,
                "flow_rate": 0.0,
                "flow_rate_o": 0.0,
                "capacity": 0,
                "ffs": 0.0,
                "avg_speed": 0.0,
                "vertical_class": 1,
                "subsegments": [],
                "phf": 0.94,
                "phv": 5.0,
                "pf": 0.0,
                "fd": 0.0,
                "fd_mid": 0.0,
                "hor_class": 0
            }
        ],
        "lane_width": 12.0,
        "shoulder_width": 6.0,
        "apd": 0.0,
        "pmhvfl": 0.4,
        "l_de": 0.0
    }
    
    print("Transportation Library Usage Example")
    print("=" * 50)
    
    # Validate input data
    warnings = validate_input_data(input_data)
    if warnings:
        print("Input Validation Warnings:")
        for warning in warnings:
            print(f"  ⚠️  {warning}")
        print()
    
    try:
        # Method 1: Direct creation
        print("Method 1: Creating highway with direct Segment constructor")
        highway1 = create_highway_from_json(input_data)
        print()
        
        
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()