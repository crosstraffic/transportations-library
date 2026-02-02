# Getting Started

<!-- toc -->

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
transportations-library = "0.1"
```

## Quick Example: Two-Lane Highway Analysis

```rust
use transportations_library::twolanehighways::{Segment, TwoLaneHighways};

// Create a Passing Constrained segment
let segment = Segment::new(
    0,          // passing_type: 0 = Passing Constrained
    1.0,        // length: 1.0 mi
    2.0,        // grade: 2%
    55.0,       // posted speed limit: 55 mi/h
    None,       // no horizontal curves
    Some(800.0),// volume: 800 veh/h
    Some(600.0),// opposing volume: 600 veh/h
    None, None, None, None, None, None,
    Some(0.92), // PHF
    Some(6.0),  // 6% heavy vehicles
    None, None, None, None,
);

// Create the highway facility
let mut highway = TwoLaneHighways::new(
    vec![segment],
    Some(12.0),  // 12-ft lanes
    Some(6.0),   // 6-ft shoulders
    Some(5.0),   // 5 access points/mi
    None,
    None,
);

// Run the analysis (Steps 1-10)
let seg_num = 0;
let (_, _, capacity) = highway.determine_demand_flow(seg_num);
highway.determine_vertical_alignment(seg_num);
highway.determine_free_flow_speed(seg_num);
let (speed, _) = highway.estimate_average_speed(seg_num);
highway.estimate_percent_followers(seg_num);
highway.determine_follower_density_pc_pz(seg_num);

let los = highway.determine_segment_los(seg_num, speed, capacity);
println!("Segment LOS: {}", los);
```

## Quick Example: Bicycle LOS Analysis

```rust
use transportations_library::twolanehighways::BicycleLOS;

let bike_analysis = BicycleLOS::new(
    12.0,   // lane width (ft)
    6.0,    // shoulder width (ft)
    50.0,   // speed limit (mi/h)
    1,      // number of lanes
    4.0,    // pavement condition (1-5 scale)
    500.0,  // hourly volume (veh/h)
    0.88,   // peak hour factor
    0.06,   // heavy vehicle percentage
    0.0,    // on-highway parking percentage
);

let result = bike_analysis.analyze();
println!("BLOS Score: {:.2}", result.blos_score);
println!("Bicycle LOS: {}", result.los);
```

## HCM Chapters Implemented

| Chapter | Module | Description |
|---------|--------|-------------|
| 12 | `basicfreeways` | Basic Freeway Segments |
| 13 | `weaving` | Freeway Weaving Segments |
| 14 | `merge_diverge` | Freeway Merge and Diverge |
| 15 | `twolanehighways` | Two-Lane Highways |

## Next Steps

- See [How To](how-to.md) for detailed usage guides
- See [Configuration](config.md) for input parameters
- See [API Reference](https://docs.rs/transportations-library) for complete documentation
