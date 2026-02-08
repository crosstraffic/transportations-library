#!/usr/bin/env python3
"""
Export constraints from transportations-library to JSON.

This script exports the library's parameter constraints to a JSON file
that can be used by:
1. transportations-validator for input validation
2. Knowledge Graph for semantic storage
3. Documentation generation

Usage:
    python scripts/export_constraints.py [--output constraints.json]
"""

import argparse
import json
import sys
from pathlib import Path


def export_constraints(output_path: Path) -> dict:
    """Export constraints from the library."""
    try:
        import transportations_library as tl

        # Get constraints from the library
        constraints_json = tl.get_constraints()
        constraints = json.loads(constraints_json)

        # Write to file
        with open(output_path, 'w') as f:
            json.dump(constraints, f, indent=2)

        print(f"✓ Exported constraints to {output_path}")
        print(f"  Version: {constraints.get('version', 'unknown')}")

        # Summary
        tlh = constraints.get('two_lane_highways', {})
        range_count = sum(1 for k, v in tlh.items() if isinstance(v, dict) and 'min' in v and 'max' in v)
        enum_count = sum(1 for k, v in tlh.items() if isinstance(v, dict) and 'values' in v)
        table_count = sum(1 for k, v in tlh.items() if isinstance(v, dict) and 'table' in v)

        print(f"  Two-Lane Highways constraints:")
        print(f"    - Range constraints: {range_count}")
        print(f"    - Enum constraints: {enum_count}")
        print(f"    - Table constraints: {table_count}")

        return constraints

    except ImportError as e:
        print(f"✗ Error: Could not import transportations_library: {e}")
        print("  Make sure to run: maturin develop --release")
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(
        description="Export constraints from transportations-library"
    )
    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=Path("constraints.json"),
        help="Output JSON file path (default: constraints.json)"
    )

    args = parser.parse_args()
    export_constraints(args.output)


if __name__ == "__main__":
    main()
