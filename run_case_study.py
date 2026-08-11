"""Run a TwoLaneHighways ExampleCase JSON through the library and check it
against the published expected outputs. Loads inputs verbatim from the JSON
(correct units: phv and sup_ele in PERCENT, per-segment phf, volume_op=0).
"""
import json, sys
from transportations_library import TwoLaneHighways, Segment, SubSegment

CASE = sys.argv[1] if len(sys.argv) > 1 else "tests/ExampleCases/hcm/TwoLaneHighways/case_study1.json"
EXPECTED = {
    "ffs": [61.934]*5,
    "ats": [59.09, 59.807, 58.975, 59.807, 59.1],
    "pf":  [60.806, 49.556, 58.006, 49.608, 55.389],
    "fd":  [5.854, 4.714, 5.595, 4.719, 5.105],
    "facility_fd": 5.25,
}

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

def row(name, g, e):
    flags = "".join(" OK" if abs(gi-ei) <= 0.06 else f" XX(exp {ei})" for gi, ei in zip(g, e))
    print(f"{name:>5}: " + "  ".join(f"{x:7.3f}" for x in g) + " |" + flags)

print("seg:      S1       S2       S3       S4       S5")
for k in ("ffs", "ats", "pf", "fd"):
    row(k, got[k], EXPECTED[k])
print(f"\nfacility follower density: got {fac_fd:.3f}  expected {EXPECTED['facility_fd']}  "
      f"{'OK' if abs(fac_fd-EXPECTED['facility_fd'])<=0.02 else 'XX'}")
print("facility LOS:", hwy.determine_facility_los(fac_fd, wsp/tot))
