//! Bit-exact dump of every Chapter 23 interchange fixture result, for proving
//! that a change to the ramp-terminal engine moves no number that is already
//! asserted anywhere.
//!
//! Every float is printed as its IEEE-754 bit pattern rather than as a decimal,
//! because a decimal rendering hides a difference in the last few ulps and a
//! tolerance-based comparison cannot distinguish "inert" from "slightly
//! different". Run it on the base commit, run it again on the branch, and
//! `cmp` the two outputs.
//!
//! ```text
//! cargo run --example ch23_dump > /tmp/before.txt
//! cargo run --example ch23_dump > /tmp/after.txt
//! cmp /tmp/before.txt /tmp/after.txt
//! ```
//!
//! Passing `--perturb` scales every fixture's cycle length by 1.0000001, which
//! is the control: it must make the dump differ. Without that control a
//! byte-identical pair of dumps is also what a harness that silently failed to
//! load anything would produce.

use transportations_library::hcm::ramp_terminals::{Interchange, OdMovement};

fn f(label: &str, v: f64) {
    println!("{label}\t{:016x}", v.to_bits());
}

fn opt(label: &str, v: Option<f64>) {
    match v {
        Some(x) => f(label, x),
        None => println!("{label}\tNONE"),
    }
}

fn main() {
    let perturb = std::env::args().any(|a| a == "--perturb");
    let dir = format!("{}/tests/ExampleCases/hcm/RampTerminals", env!("CARGO_MANIFEST_DIR"));
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read_dir {dir}: {e}"))
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort();
    assert!(!names.is_empty(), "no fixtures found in {dir}");

    for name in &names {
        let path = format!("{dir}/{name}");
        let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        let mut ix: Interchange =
            serde_json::from_str(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"));
        if perturb {
            ix.cycle_length_s *= 1.000_000_1;
        }
        ix.analyze();

        println!("=== {name} ===");
        f("cycle_length_s", ix.cycle_length_s);
        opt("interchange_ett_s", ix.interchange_ett_s);
        println!("interchange_los\t{:?}", ix.interchange_los);

        for r in ix.get_results() {
            let m = r.movement.name();
            f(&format!("{m}.flow_rate"), r.flow_rate);
            opt(&format!("{m}.lane_utilization"), r.lane_utilization);
            opt(&format!("{m}.traffic_pressure"), r.traffic_pressure);
            opt(&format!("{m}.sat_flow"), r.sat_flow);
            opt(&format!("{m}.dq_lost_time_s"), r.downstream_queue_lost_time_s);
            opt(&format!("{m}.ds_lost_time_s"), r.demand_starvation_lost_time_s);
            opt(&format!("{m}.adjusted_lost_time_s"), r.adjusted_lost_time_s);
            opt(&format!("{m}.effective_green_s"), r.effective_green_s);
            opt(&format!("{m}.capacity"), r.capacity);
            opt(&format!("{m}.vc_ratio"), r.vc_ratio);
            opt(&format!("{m}.upstream_filtering"), r.upstream_filtering);
            opt(&format!("{m}.back_of_queue_veh"), r.back_of_queue_veh);
            opt(&format!("{m}.queue_storage_ratio"), r.queue_storage_ratio);
            opt(&format!("{m}.uniform_delay_s"), r.uniform_delay_s);
            opt(&format!("{m}.incremental_delay_s"), r.incremental_delay_s);
            opt(&format!("{m}.initial_queue_delay_s"), r.initial_queue_delay_s);
            opt(&format!("{m}.control_delay_s"), r.control_delay_s);
        }

        for o in OdMovement::ALL {
            let Some(r) = ix.get_od_results().iter().find(|r| r.movement == o) else {
                println!("od.{o:?}\tABSENT");
                continue;
            };
            f(&format!("od.{o:?}.demand"), r.demand);
            f(&format!("od.{o:?}.control_delay_s"), r.control_delay_s);
            f(&format!("od.{o:?}.edtt_s"), r.edtt_s);
            f(&format!("od.{o:?}.ett_s"), r.ett_s);
            println!("od.{o:?}.vc_gt_1\t{}", r.vc_exceeds_one);
            println!("od.{o:?}.rq_gt_1\t{}", r.rq_exceeds_one);
            println!("od.{o:?}.los\t{:?}", r.los);
        }

        // Routing is part of the invariant: a new movement composition that
        // silently captured an existing O-D leg would not necessarily move a
        // number in a fixture that has no lane group for it, but it would move
        // the path.
        for o in OdMovement::ALL {
            let path: Vec<String> = ix.od_path(o).iter().map(|m| m.name()).collect();
            println!("path.{o:?}\t{}", path.join(","));
        }
    }
}
