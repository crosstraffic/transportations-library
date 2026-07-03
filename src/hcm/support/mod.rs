//! Cross-cutting support modules: geometry, network topology, traffic-flow
//! fundamentals, semantic validation, and the core facility abstractions.
//! These are not tied to a single HCM chapter.

pub mod constraints;
// `core/` is unfinished WIP inherited from main (was never declared in the
// module tree there either; does not compile). To be salvaged or removed by
// the Chapter 10 freeway-facilities work, which its work-zone content targets.
// pub mod core;
pub mod geometric;
pub mod topology;
pub mod traffic_flow;
