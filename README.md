<h1 align="center">Transportations Library</h1>

A comprehensive Rust-based library implementing transportation engineering methodologies (e.g. the Highway Capacity Manual (HCM)) with Python bindings.

## What this covers

Currently implements:

- Highway Capacity Manual (HCM) Chapter 15: Two-Lane Highways analysis
- Other chapters are to be added in future releases

## Installation
### Prerequisites

- Rust: Install from [rustup.rs](https://rustup.rs/)
- Python: 3.8 or higher
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

To test the alpha release from Test PyPI, use:
```bash
pip install --no-cache-dir --verbose -i https://test.pypi.org/simple/ transportations-library==0.1.9a4
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
