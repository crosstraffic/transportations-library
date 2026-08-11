"""Run a TwoLaneHighways ExampleCase JSON through the library and check it
against the expected outputs. Loads inputs verbatim from the JSON, in the units
the engine documents: phv and sup_ele in PERCENT (Exhibit 15-9 brackets at
5/10/15/20/25 and Exhibit 15-22 superelevation thresholds at 1-8 are both read
as percentages), per-segment phf, opposing volume in veh/h.

Exits nonzero on any mismatch, so this can be used as a gate.

The default case, case_study1.json, is the River Falls corridor, the same
facility that analyze_river_falls.py and tests/test_river_falls_gate.py build by
hand in Python. It is NOT an HCM example problem and has no published answer
column, so the expectations below are this engine's own output, pinned to catch
drift rather than to certify correctness.

READ BEFORE TRUSTING THE FACILITY NUMBER. This fixture and the hand-built
facility in test_river_falls_gate.py do not agree, and the disagreement is a
live open question rather than a rounding difference:

  * This fixture yields facility FD 5.310.
  * The gate test and analyze_river_falls.py yield 5.223, the value the project
    currently treats as canonical (it supersedes the older 5.09, which was S5's
    segment density mislabeled as the facility value, and 5.25, which this
    script previously expected).
  * The gap is entirely in the inputs, not the engine. The Python constructions
    pass phv=0.08 and sup_ele=0.02 as fractions where the engine reads percent,
    which understates the heavy-vehicle FFS penalty and drops both curve
    subsegments into a lower horizontal class. Correcting only those two, with
    every other input held at the Python construction's values, gives 5.309;
    the residual 0.001 against this fixture is the fixture rounding segment 3 to
    2.34 mi where its subsegments sum to 2.3390 mi. The two constructions
    otherwise converge, so 5.223 is low by roughly 0.09 for a unit reason.

5.223 appears in the paper, so it is not changed here. Resolving 5.223 vs 5.310
is Rei's call.
"""
import json, sys
from transportations_library import TwoLaneHighways, Segment, SubSegment

DEFAULT_CASE = "tests/ExampleCases/hcm/TwoLaneHighways/case_study1.json"
CASE = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_CASE

# Engine output for case_study1.json, pinned as a drift gate. Not published values.
EXPECTED = {
    "ffs": [61.934]*5,
    "ats": [59.152, 59.360, 56.704, 59.361, 56.756],
    "pf":  [59.642, 54.472, 56.863, 54.525, 55.389],
    "fd":  [5.492, 4.998, 5.462, 5.003, 5.316],
    "facility_fd": 5.310,
}
TOL = 0.001

d = json.loads(open(CASE).read())
segs = []
for s in d["segments"]:
    subs = [SubSegment(length=ss["length"], avg_speed=ss["avg_speed"], hor_class=ss["hor_class"],
                       design_rad=ss["design_rad"], central_angle=ss["central_angle"], sup_ele=ss["sup_ele"])
            for ss in s.get("subsegments", [])]
    segs.append(Segment(passing_type=s["passing_type"], length=s["length"], grade=s["grade"], spl=s["spl"],
                        is_hc=s["is_hc"], volume=s["volume"], volume_op=s["volume_op"], phf=s["phf"],
                        phv=s["phv"], subsegments=subs or None))
hwy = TwoLaneHighways(segments=segs, lane_width=d["lane_width"], shoulder_width=d["shoulder_width"],
                      apd=d["apd"], pmhvfl=d["pmhvfl"], l_de=d.get("l_de", 0.0))

got = {"ffs": [], "ats": [], "pf": [], "fd": []}
tot = wsp = 0.0
for i in range(hwy.num_segments):
    hwy.identify_vertical_class(i); hwy.determine_demand_flow(i); hwy.determine_vertical_alignment(i)
    ffs = hwy.determine_free_flow_speed(i); ats = hwy.estimate_average_speed(i)[0]
    pf = hwy.estimate_percent_followers(i)
    (hwy.determine_follower_density_pl if segs[i].passing_type == 2 else hwy.determine_follower_density_pc_pz)(i)
    hwy.determine_adjustment_to_follower_density(i)
    seg = hwy.segments[i]; fd = seg.followers_density; L = seg.length
    got["ffs"].append(ffs); got["ats"].append(ats); got["pf"].append(pf); got["fd"].append(fd)
    tot += L; wsp += ats*L
fac_fd = hwy.determine_facility_follower_density()
# Exhibit 15-6 splits on POSTED SPEED LIMIT, not on the computed average speed.
# HCM Step 11 defines no facility-level posted limit, so length-weight it.
fac_spl = sum(s.spl * s.length for s in hwy.segments) / tot

checked = CASE.replace("\\", "/").endswith(DEFAULT_CASE.rsplit("/", 1)[-1])
bad = 0


def row(name, g, e):
    global bad
    flags = ""
    for gi, ei in zip(g, e):
        if abs(gi - ei) <= TOL:
            flags += " OK"
        else:
            flags += f" XX(exp {ei})"
            bad += 1
    print(f"{name:>5}: " + "  ".join(f"{x:7.3f}" for x in g) + " |" + flags)


print("seg:      S1       S2       S3       S4       S5")
for k in ("ffs", "ats", "pf", "fd"):
    if checked:
        row(k, got[k], EXPECTED[k])
    else:
        print(f"{k:>5}: " + "  ".join(f"{x:7.3f}" for x in got[k]))

if checked:
    ok = abs(fac_fd - EXPECTED["facility_fd"]) <= TOL
    bad += not ok
    print(f"\nfacility follower density: got {fac_fd:.3f}  expected {EXPECTED['facility_fd']}  "
          f"{'OK' if ok else 'XX'}")
else:
    print(f"\nfacility follower density: got {fac_fd:.3f}  (no expectations for {CASE})")
print("facility LOS:", hwy.determine_facility_los(fac_fd, fac_spl))

if bad:
    print(f"\n{bad} value(s) drifted from the pinned expectations.", file=sys.stderr)
sys.exit(1 if bad else 0)
