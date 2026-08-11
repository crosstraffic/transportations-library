"""Directional operational analysis — WIS 35/WIS 65 (River Falls Bypass), EB.

HCM 7th ed. Chapter 15 (Two-Lane Highways) via the transportations_library
Rust engine. Inputs are verbatim from the analysis request; nothing is estimated.

Two engine conventions that are easy to get wrong (both verified against the
Rust source) and are applied correctly here:
  * `spl` is the POSTED SPEED LIMIT; the engine derives BFFS = 1.14 * spl
    (HCM default). The request's "BFFS = 55 (assumed from speed limit)" is read
    as PSL = 55 -> BFFS = 62.7. (Sensitivity: forcing BFFS = 55 keeps LOS = C.)
  * Complex-alignment segments must be built with is_hc=True, and SubSegment
    length is in FEET (the engine divides by 5280 internally) while Segment
    length is in MILES. Passing subsegment length in miles, or leaving is_hc
    False, silently drops the horizontal-curve speed penalty.
"""
import json
from transportations_library import TwoLaneHighways, Segment, SubSegment

LW, SW, APD, PHV, PHF, PSL, V, VO = 12.0, 6.0, 2.0, 0.08, 0.94, 55.0, 512.0, 512.0
FT = 5280.0


def sub(length_ft, rad=None):
    return (SubSegment(length=length_ft, hor_class=1) if rad is None
            else SubSegment(length=length_ft, design_rad=rad, sup_ele=0.02))


def seg(pt, length_mi, grade, ss=None, hc=None):
    return Segment(passing_type=pt, length=length_mi, grade=grade, spl=PSL,
                   is_hc=hc, volume=V, volume_op=VO, phv=PHV, phf=PHF, subsegments=ss)


ss3 = [sub(3200), sub(2850, 840.0), sub(2200), sub(1800, 845.0), sub(2300)]
ss5 = [sub(1848), sub(6600, 2520.0)]
len3 = sum(s.length for s in ss3) / FT      # feet -> miles for the segment length
len5 = sum(s.length for s in ss5) / FT

segments = [
    seg(0, 0.16, 1.0),                # S1 passing-constrained
    seg(1, 0.64, 1.0),                # S2 passing zone
    seg(0, len3, 0.0, ss3, hc=True),  # S3 passing-constrained, complex alignment
    seg(1, 0.625, 0.0),               # S4 passing zone
    seg(0, len5, 1.0, ss5, hc=True),  # S5 passing-constrained, complex alignment
]

hwy = TwoLaneHighways(segments=segments, lane_width=LW, shoulder_width=SW,
                      apd=APD, pmhvfl=PHV)

inter, rows = [], []
tot_len = w_spd = 0.0
for i in range(hwy.num_segments):
    vc_min, vc_max = hwy.identify_vertical_class(i)
    v_i, v_o, cap = hwy.determine_demand_flow(i)
    vc = hwy.determine_vertical_alignment(i)
    ffs = hwy.determine_free_flow_speed(i)
    ats = hwy.estimate_average_speed(i)[0]
    pf = hwy.estimate_percent_followers(i)
    hwy.determine_follower_density_pc_pz(i)
    hwy.determine_adjustment_to_follower_density(i)
    s = hwy.segments[i]
    fd = s.followers_density
    los = hwy.determine_segment_los(i, ats, int(cap))
    tot_len += s.length
    w_spd += ats * s.length
    inter.append({"seg": i + 1, "length_mi": round(s.length, 4),
                  "vc": vc, "v_i": round(v_i, 2), "v_o": round(v_o, 2),
                  "capacity": int(cap), "ffs": round(ffs, 2), "ats": round(ats, 2),
                  "pf": round(pf, 2), "fd": round(fd, 3), "los": los})
    rows.append((i + 1, ffs, ats, pf, fd, los))

# Equation 15-39 over the adjusted densities; see determine_facility_follower_density.
fac_fd, fac_spd = hwy.determine_facility_follower_density(), w_spd / tot_len
fac_los = hwy.determine_facility_los(fac_fd, fac_spd)
print(json.dumps({"intermediate": inter,
                  "facility": {"total_length_mi": round(tot_len, 4),
                               "length_weighted_ats": round(fac_spd, 2),
                               "facility_follower_density": round(fac_fd, 3),
                               "facility_los": fac_los}}, indent=2, default=str))
