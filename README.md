<h1 align="center">Transportations Library</h1>

A comprehensive Rust-based library implementing transportation engineering methodologies (e.g. the Highway Capacity Manual (HCM)) with Python bindings.

## What this covers

Highway Capacity Manual 7th Edition computational chapters 10 through 24, with the supplemental
chapters (25, 27, 28, 30, 31, 32, 33, 34, 35) they draw on:

| Chapter | Topic | Chapter | Topic |
|---|---|---|---|
| 10 | Freeway Facilities | 18 | Urban Street Segments |
| 11 | Freeway Reliability | 19 | Signalized Intersections |
| 12 | Basic Freeway and Multilane Segments | 20 | Two-Way STOP-Controlled Intersections |
| 13 | Freeway Weaving Segments | 21 | All-Way STOP-Controlled Intersections |
| 14 | Freeway Merge and Diverge Segments | 22 | Roundabouts |
| 15 | Two-Lane Highways | 23 | Ramp Terminals and Alternative Intersections |
| 16 | Urban Street Facilities | 24 | Off-Street Pedestrian and Bicycle Facilities |
| 17 | Urban Street Reliability | | |

Methodologies are validated against the manual's own published example problems; see
`docs/hcm/procedures/` for per-chapter walkthroughs and `docs/hcm/VERIFICATION.md` for the places
where the manual is ambiguous, self-contradictory, or not reproducible from its printed procedure.

### Selecting an HCM edition

Edition 7.1 (November 2025) replaces Chapters 13, 14, 27, and 28 with new weaving, merge, and
diverge methodologies. It does not supersede the rest of the manual, so the edition is selected per
segment rather than globally, and defaults to the 7th Edition:

```python
import json, transportations_library as tl

tl.hcm_versions()          # ["7", "7.1"]
tl.hcm_latest_version()    # "7.1"

seg = tl.WeavingSegment(version="7.1", length_short=1500.0, num_lanes=4, ffs=65.0,
                        v_ff=1815.0, v_fr=692.0, v_rf=1037.0, v_rr=1297.0,
                        phf=0.91, heavy_vehicle_pct=0.05,
                        lc_rf=0, lc_fr=1, nw_rf=2, nw_fr=1)
seg.run_analysis()                       # "C"
json.loads(seg.analysis_v7_1())["speed_avg"]   # 59.32 mi/h
```

The two editions are different models, not successive refinements: the same segment can land a full
LOS letter apart between them. `tl.hcm_version_changes_chapter("7.1", 19)` returns `False`, because
Edition 7.1 left Chapter 19 alone.

## Installation
### Prerequisites

- Rust: Install from [rustup.rs](https://rustup.rs/)
- Python: 3.10 or higher
- UV: Modern Python package manager (recommended)

**Using UV (Recommended)**
```bash
# Clone the repository
git clone https://github.com/crosstraffic/transportations-library
cd transportations-library

# Create and activate virtual environment
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install in development mode
uv pip install maturin pytest
maturin develop --release
```

**Using pip**
```bash
# Install dependencies
pip install maturin pytest

# Build and install
maturin develop --release
```

**From PyPI**
```bash
pip install transportations-library
```

### Quick Start

For Two Lane Highways.

**Python Usage**
```python
import transportations_library as tl

# Create a highway segment
segment = tl.Segment(
    passing_type=0,     # Passing Constrained
    length=1.5,         # 1.5 miles
    grade=2.0,          # 2% grade
    spl=55.0,           # 55 mph speed limit
    volume=800.0,       # 800 veh/hr
    phf=0.95,           # Peak hour factor
    phv=5.0             # 5% heavy vehicles
)

# Create highway facility
highway = tl.TwoLaneHighways([segment])

# Perform complete analysis
seg_num = 0
demand_flow, opposing_flow, capacity = highway.determine_demand_flow(seg_num)
ffs = highway.determine_free_flow_speed(seg_num)
avg_speed, _ = highway.estimate_average_speed(seg_num)
percent_followers = highway.estimate_percent_followers(seg_num)
follower_density = highway.determine_follower_density_pc_pz(seg_num)
los = highway.determine_segment_los(seg_num, avg_speed, capacity)

print(f"Level of Service: {los}")
print(f"Average Speed: {avg_speed:.1f} mph")
print(f"Follower Density: {follower_density:.1f} followers/mile")
```

Subsegment sections.
```python
# Highway with horizontal curves
subsegments = [
    tl.SubSegment(length=2640.0, design_rad=800.0, sup_ele=4.0),  # Curved section
    tl.SubSegment(length=2640.0, design_rad=0.0, sup_ele=0.0)     # Tangent section
]

segment_with_curves = tl.Segment(
    passing_type=0, length=1.0, grade=3.0, spl=55.0,
    is_hc=True,  # Has horizontal curves
    subsegments=subsegments,
    volume=900.0, phf=0.92, phv=8.0
)

highway = tl.TwoLaneHighways([segment_with_curves])
# ... perform analysis
```

### Parameter Constraints

The library exports all HCM/AASHTO parameter constraints as JSON, which can be used by validators and knowledge graphs:

```python
import transportations_library as tl
import json

# Get all constraints
constraints = json.loads(tl.get_constraints())
print(f"Version: {constraints['version']}")

# Access specific constraint
lane_width = constraints['two_lane_highways']['lane_width']
print(f"Lane width: {lane_width['min']}-{lane_width['max']} {lane_width['unit']}")
print(f"Source: {lane_width['source']}")
# Output: Lane width: 9.0-12.0 ft
# Output: Source: HCM 7th Edition, Exhibit 15-8

# Validate inputs directly
errors = tl.validate_input(lane_width=8.0)  # Invalid - below 9 ft
print(errors)
# Output: ['lane_width = 8 ft is outside valid range [9, 12]. Source: HCM 7th Edition, Exhibit 15-8']
```

Available constraints include:
- `lane_width`, `shoulder_width` (range)
- `passing_type`, `horizontal_class`, `vertical_class` (enum)
- `grade`, `phf`, `phv`, `speed_limit` (range)
- `speed_radius` (table lookup - AASHTO Table 3-7)

### Using from Rust, Python, and JavaScript

The same compute core is reachable from three languages, and the mapping is mechanical:

- **Rust** is the source of truth. Every chapter lives under `src/hcm/`, and the structs there (`BasicFreeways`, `WeavingSegment`, `RampSegment`, ...) are the API. Add the crate as a dependency and call the `run_analysis`/step methods directly.
- **Python** bindings are generated from the same structs via PyO3 (`src/copython/`), built with `maturin`. Field names, defaults, and units are identical to the Rust side; constructors take the struct fields as keyword arguments, and enums map to strings (`version="7.1"`, `terrain="level"`). A Rust method returning `Option<T>` returns `None` in Python.
- **JavaScript** goes through WebAssembly, but not from this repo: [`cross-traffic-middleware`](https://github.com/crosstraffic/cross-traffic-middleware) wraps these structs in `wasm_bindgen` types (`WasmBasicFreeways`, `WasmRampSegment`, ...) and is built with `wasm-pack`. The same `Option<T>` becomes `undefined`. The [web calculator](https://github.com/crosstraffic/cross-traffic-web-calculator) is the reference consumer.

One convention to know when porting numbers between languages: percentages are percent in the UI-facing bindings and decimals in Rust where the HCM equation wants a proportion; each binding's docstring states which it takes. Editions, LOS letters, and every published-example value are identical across the three surfaces, and the integration tests assert the Rust and Python sides against the same JSON fixtures in `tests/ExampleCases/`.

## Testing

### Run Tests
```bash
# Rust tests
cargo test

# Python tests
pytest tests/

# With coverage
pytest tests/ --cov=transportations_library

# Integration tests for chapter 15
cargo test --test chapter15_integration
```

**Note**: If you want to have changes in the Rust code to be reflected in Python, you need to run `cargo clean` and `maturin develop` again after making changes.

### Example Test Cases
The library includes comprehensive test cases based on HCM examples:

- Case 1: Basic passing constrained segment
- Case 2: Segment with horizontal curves
- Case 3: Multi-segment facility with different passing types
- Case 4: Steep grade conditions with heavy vehicles

## Development
### Project Structure
```plaintext
transportations-library/
├── src/
│   ├── hcm/
│   │   ├── chapter15/           # Two-lane highways implementation
│   │   └── common.rs            # Shared HCM utilities
│   ├── copython/                # Python bindings
│   ├── utils.rs                 # Mathematical utilities
│   └── lib.rs                   # Library root
├── tests/                       # Integration tests
├── examples/                    # Usage examples
└── Cargo.toml                   # Rust configuration
```

### Building from Source
```bash
# Development build
cargo build

# Release build
cargo build --release

# Build Python wheel
maturin build --release

# Development install with changes
cargo clean && maturin develop --release
```

### Pipeline
The project uses GitHub Actions for CI/CD, including:
- Running tests on push and pull requests
- Building and publishing to Test PyPI on alpha releases
- Building and publishing to Cargo and PyPI on new releases

To install a pre-release from Test PyPI, use (replace the version as needed):
```bash
pip install --no-cache-dir --verbose -i https://test.pypi.org/simple/ transportations-library==<version>
```

Versioning follows [Semantic Versioning](https://semver.org/).

Also, you can find the latest alpha releases on [Test PyPI](https://test.pypi.org/project/transportations-library/).


### Citation

If you use transportations-library or CrossTraffic in your research, please cite it as follows:

```bibtex
@software{tamaru2025tralib,
  title = {Transportations Library: Transportation knowledge management platform},
  author = {Tamaru, Rei},
  year = {2025},
  url = {https://github.com/crosstraffic/transportations-library},
  doi = {10.5281/zenodo.17295792},
}
```

You can also use the DOI to cite a specific version: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.15858845.svg)](https://doi.org/10.5281/zenodo.17295792)

Alternatively, you can find the citation information in the [CITATION.cff](CITATION.cff) file in this repository, which follows the Citation File Format standard.

---

**Note**: This library implements established transportation engineering methodologies for educational and professional use. Users should verify results and apply appropriate engineering judgment for real-world applications.
