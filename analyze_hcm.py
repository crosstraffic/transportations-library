import transportations_library
from transportations_library import TwoLaneHighways, Segment, SubSegment

def analyze():
    # Global Parameters
    lane_width = 12.0
    shoulder_width = 6.0
    apd = 2.0
    phv = 0.08
    phf = 0.94
    bffs = 55.0
    
    # Segments
    segments_data = []
    
    # Segment 1: Passing-constrained, 0.16 mi, 1% grade
    segments_data.append(Segment(
        passing_type=0,
        length=0.16,
        grade=1.0,
        spl=bffs,
        volume=512.0,
        volume_op=512.0,
        phv=phv,
        phf=phf
    ))
    
    # Segment 2: Passing Zone, 0.64 mi, 1% grade
    segments_data.append(Segment(
        passing_type=1,
        length=0.64,
        grade=1.0,
        spl=bffs,
        volume=512.0,
        volume_op=512.0,
        phv=phv,
        phf=phf
    ))
    
    # Segment 3: Passing-constrained, 0% grade, Complex Alignment
    # Subsegments:
    # 1. Tangent: 3200 ft
    # 2. Curve: 3850 ft (R = 840 ft, e = 2%)
    # 3. Tangent: 2200 ft
    # 4. Curve: 1800 ft (R = 845 ft, e = 2%)
    # 5. Tangent: 2300 ft
    
    subsegments_3 = []
    subsegments_3.append(SubSegment(length=3200/5280, hor_class=1))
    subsegments_3.append(SubSegment(length=3850/5280, design_rad=840.0, sup_ele=0.02))
    subsegments_3.append(SubSegment(length=2200/5280, hor_class=1))
    subsegments_3.append(SubSegment(length=1800/5280, design_rad=845.0, sup_ele=0.02))
    subsegments_3.append(SubSegment(length=2300/5280, hor_class=1))
    
    len_3 = sum([s.length for s in subsegments_3])
    
    segments_data.append(Segment(
        passing_type=0,
        length=len_3,
        grade=0.0,
        spl=bffs,
        volume=512.0,
        volume_op=512.0,
        phv=phv,
        phf=phf,
        subsegments=subsegments_3
    ))
    
    # Segment 4: Passing Zone, 0.625 mi, 0% grade
    segments_data.append(Segment(
        passing_type=1,
        length=0.625,
        grade=0.0,
        spl=bffs,
        volume=512.0,
        volume_op=512.0,
        phv=phv,
        phf=phf
    ))
    
    # Segment 5: Passing-constrained, 1% grade, Complex Alignment
    # Subsegments:
    # 1. Tangent: 1848 ft
    # 2. Aggregated Curves: 6600 ft (R = 2520 ft, e = 2%)
    
    subsegments_5 = []
    subsegments_5.append(SubSegment(length=1848/5280, hor_class=1))
    subsegments_5.append(SubSegment(length=6600/5280, design_rad=2520.0, sup_ele=0.02))
    
    len_5 = sum([s.length for s in subsegments_5])
    
    segments_data.append(Segment(
        passing_type=0,
        length=len_5,
        grade=1.0,
        spl=bffs,
        volume=512.0,
        volume_op=512.0,
        phv=phv,
        phf=phf,
        subsegments=subsegments_5
    ))
    
    # Create TwoLaneHighways object
    # Note: l_de is optional, but maybe I should calculate it? 
    # The prompt doesn't specify it, so I'll leave it None.
    highway = TwoLaneHighways(
        segments=segments_data,
        lane_width=lane_width,
        shoulder_width=shoulder_width,
        apd=apd,
        pmhvfl=phv # Assuming pmhvfl = phv
    )
    
    print("Intermediate Calculations:")
    print("-" * 30)
    
    total_length = 0.0
    weighted_fd = 0.0
    weighted_speed = 0.0
    
    results = []
    
    for i in range(highway.num_segments):
        print(f"\nAnalyzing Segment {i+1}...")
        
        # Step 1: Identify vertical class
        vc_min, vc_max = highway.identify_vertical_class(i)
        print(f"  Vertical Class Factors: min={vc_min}, max={vc_max}")
        
        # Step 2: Determine demand flow rates and capacity
        v_i, v_o, cap = highway.determine_demand_flow(i)
        print(f"  Demand Flow: v_i={v_i:.2f}, v_o={v_o:.2f}, Capacity={cap}")
        
        # Step 3: Determine vertical alignment
        vc = highway.determine_vertical_alignment(i)
        print(f"  Vertical Class: {vc}")
        
        # Step 4: Determine free flow speed
        ffs = highway.determine_free_flow_speed(i)
        print(f"  Free Flow Speed: {ffs:.2f} mph")
        
        # Step 5: Estimate average speed
        ats_res = highway.estimate_average_speed(i)
        ats = ats_res[0]
        print(f"  Average Travel Speed: {ats:.2f} mph")
        
        # Step 6: Estimate percent followers
        pf = highway.estimate_percent_followers(i)
        print(f"  Percent Followers: {pf:.2f}%")
        
        # Step 7: Determine follower density
        # Need to check passing type from the segment object
        # Since highway methods update internal state, we can get the segment again?
        # Or just use the input data passing type since it doesn't change.
        # But let's use the getter if possible.
        # highway.get_segments(py) returns a list.
        # But I can just check the input segments_data[i].passing_type
        pt = segments_data[i].passing_type
        
        if pt == 2: # Passing Lane
             highway.determine_follower_density_pl(i)
        else:
             highway.determine_follower_density_pc_pz(i)
             
        # Step 8: Adjustment to follower density
        highway.determine_adjustment_to_follower_density(i)
        
        # Retrieve results from the updated segment
        # We need to get the updated segment from the highway object
        # Since the Python wrapper holds the Rust object, and methods update it.
        # But `get_segments` returns a copy. So we call it now.
        # Wait, `get_segments` returns a list of `Segment` objects.
        # I need to access the i-th segment.
        # Note: `get_segments` might be expensive if it copies all.
        # But for 5 segments it's fine.
        
        # Actually, I can use the getters on the segment object returned by `get_segments`.
        # But I can't index into `highway` directly.
        # I'll get the list.
        
        # Wait, `get_segments` requires `py` context in Rust, but in Python it's just a property/method.
        updated_segments = highway.segments # Using the property `segments` which maps to `get_segments`
        seg = updated_segments[i]
        
        fd = seg.followers_density
        if pt == 2:
            fd = seg.followers_density_mid # Use mid for PL? Or just fd?
            # The Rust code used `get_followers_density_mid` for PL.
            # But `determine_follower_density_pl` updates `fd` and `fd_mid`.
            # Let's check `determine_follower_density_pl` implementation.
            # It updates `fd`.
            # But `tests/common/mod.rs` uses `get_followers_density_mid` for PL.
            # I'll stick to `fd` unless I see a reason.
            # Actually, for facility analysis, we usually use the average FD over the segment.
            # `fd` should be the average.
            pass

        # Check if `followers_density` property exists on Segment in Python
        # In `py_transportationslibrary.rs`: `get_followers_density` is exposed.
        # So `seg.followers_density` should work.
        
        print(f"  Follower Density: {fd:.2f} followers/mi/ln")
        
        los = highway.determine_segment_los(i, ats, int(cap))
        print(f"  Segment LOS: {los}")
        
        length = seg.length
        weighted_fd += fd * length
        weighted_speed += ats * length
        total_length += length
        
        results.append({
            "id": i+1,
            "ffs": ffs,
            "ats": ats,
            "pf": pf,
            "fd": fd,
            "los": los
        })

    facility_fd = weighted_fd / total_length
    facility_speed = weighted_speed / total_length
    facility_los = highway.determine_facility_los(facility_fd, facility_speed)
    
    print("\nSummary Table:")
    print("| Segment ID | Adjusted FFS (mph) | Average Travel Speed (ATS) | Percent Follower (%) | Follower Density (FD) | Segment LOS |")
    print("|---|---|---|---|---|---|")
    for r in results:
        print(f"| {r['id']} | {r['ffs']:.2f} | {r['ats']:.2f} | {r['pf']:.2f} | {r['fd']:.2f} | {r['los']} |")
        
    print(f"\nFacility Level Follower Density: {facility_fd:.2f}")
    print(f"Facility Level LOS: {facility_los}")

if __name__ == "__main__":
    analyze()
